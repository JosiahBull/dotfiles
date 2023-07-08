use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    fs::metadata,
    ops::Deref,
    string::FromUtf8Error,
    sync::{Arc, RwLock, TryLockError},
};

use crate::command::{CommandError, Output};

/* General System Dependencies */
pub mod gcc;
pub mod package_cache_refresh;

/* Docker + Dependencies */
pub mod apt_transport_https;
pub mod ca_certificates;
pub mod curl;
pub mod docker;
pub mod gnupg;

/* Zsh + Dependencies */
pub mod ohmyzsh;
pub mod powerlevel10k;
pub mod tmux;
pub mod zsh;
pub mod zsh_aliases;
pub mod zsh_autosuggestions;
pub mod zsh_syntax_highlighting;
pub mod zshrc;

/* Python3 + Applications */
pub mod pip3;
pub mod python3;
pub mod python3_dev;
pub mod setuptools;
pub mod thefuck;

/* Rust + Applications */
pub mod bat;
pub mod rust;
pub mod tokei;
pub mod zoxide;

/* Firefox */
pub mod firefox;
pub mod firefox_config;

/* NodeJs + Applications */
pub mod nodejs;
pub mod nvm;
pub mod yarn;
// pub mod redoc_cli;

/* SSH + Git */
pub mod git;
pub mod gitconfig;
// pub mod ssh;
// pub mod openssh;
pub mod authorized_keys;
pub mod ed25519key;
pub mod sshconfig;

/* Misc */
pub mod scripts;
pub mod vscode;

/* Helper Functions */
pub fn rename_bak_file(file_path: &str) -> Result<(), std::io::Error> {
    let bak_path = format!("{}.bak", file_path);
    // check if path already taken
    if metadata(&bak_path).is_ok() {
        rename_bak_file(&bak_path)?;
        std::fs::remove_file(&bak_path)?;
    }

    std::fs::rename(file_path, bak_path)?;
    Ok(())
}

/* General Deps */
#[derive(Debug, Clone, Copy)]
pub enum AsUser {
    User,
    Root,
}

#[derive(Debug)]
pub struct CommandResult {
    pub success: bool,
    pub error: Option<DependencyError>,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, Copy)]
pub enum ConfigStatus {
    NotPresent,
    PresentIncorrect,
    PresentCorrect,
}

impl Default for ConfigStatus {
    fn default() -> Self {
        ConfigStatus::NotPresent
    }
}

#[derive(Debug)]
pub enum DependencyError {
    Unknown,
    NotInstalled,
    UnsupportedOperatingSystem,
    IoError(std::io::Error),
    DependencyFailed(String),
    Utf8Error(FromUtf8Error),
    CommandError(Output),
}

impl std::fmt::Display for DependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DependencyError::Unknown => write!(f, "Unknown error"),
            DependencyError::NotInstalled => write!(f, "Dependency not installed"),
            DependencyError::UnsupportedOperatingSystem => {
                write!(f, "Unsupported operating system")
            }
            DependencyError::IoError(e) => write!(f, "IO error: {}", e),
            DependencyError::DependencyFailed(e) => {
                write!(f, "Missing or unable to install required dependency: {}", e)
            }
            DependencyError::Utf8Error(e) => write!(f, "UTF-8 error: {}", e),
            DependencyError::CommandError(e) => write!(f, "Command error: {}", e),
        }
    }
}

impl std::error::Error for DependencyError {}

impl From<std::io::Error> for DependencyError {
    fn from(e: std::io::Error) -> Self {
        DependencyError::IoError(e)
    }
}

impl From<FromUtf8Error> for DependencyError {
    fn from(e: FromUtf8Error) -> Self {
        DependencyError::Utf8Error(e)
    }
}

impl From<CommandError> for DependencyError {
    fn from(e: CommandError) -> Self {
        match e {
            CommandError::IoError(e) => DependencyError::IoError(e),
            CommandError::Utf8Error(e) => DependencyError::Utf8Error(e),
            CommandError::CommandFailed(e) => DependencyError::CommandError(e),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum InstallationStatus {
    FullyInstalled,
    PartialInstall,
    NotInstalled,
}

#[derive(Debug)]
pub enum Installable {
    AlreadyInstalled,
    MissingDependency,
    InvalidOS,
    Other(String),
    Unknown,
}

pub trait DependencyInfo {
    /// Get the name of the dependency.
    fn name(&self) -> &'static str;

    /// Get a list of all dependencies that this application requires
    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![]
    }

    /// Get a list of optional dependencies which will enable additional features
    /// Should be a list of tuples of (name, description, dependency)
    // XXX: update this to have a struct
    fn optional(
        &self,
    ) -> Vec<(
        &'static str,
        &'static str,
        &'static dyn DependencyInstallable,
    )> {
        vec![]
    }
}

pub trait DependencyInstallable: DependencyInfo + Debug {
    /// Check if the dependency is installed on the current system.
    /// Updates internal state to reflect the current status.
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError>;

    /// Install the dependency.
    fn install(&self) -> Result<(), DependencyError>;
}

pub struct DependencyGraphNode {
    top_level: bool,
    enabled: bool,
    // XXX: replace this arc with references
    dependencies: RwLock<Vec<Arc<DependencyGraphNode>>>,
    dependants: RwLock<Vec<Arc<DependencyGraphNode>>>,
    wrapped: &'static dyn DependencyInstallable,
}

impl Debug for DependencyGraphNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dependency_names = match self.dependencies.try_read() {
            Ok(guard) => {
                let mut names = vec![];
                for dependency in guard.iter() {
                    names.push(dependency.wrapped.name());
                }
                names
            }
            Err(TryLockError::Poisoned(err)) => {
                let mut names = vec![];
                for dependency in err.into_inner().iter() {
                    names.push(dependency.wrapped.name());
                }
                names
            }
            Err(TryLockError::WouldBlock) => {
                vec!["<locked>"]
            }
        };
        let dependant_names = match self.dependants.try_read() {
            Ok(guard) => {
                let mut names = vec![];
                for dependency in guard.iter() {
                    names.push(dependency.wrapped.name());
                }
                names
            }
            Err(TryLockError::Poisoned(err)) => {
                let mut names = vec![];
                for dependency in err.into_inner().iter() {
                    names.push(dependency.wrapped.name());
                }
                names
            }
            Err(TryLockError::WouldBlock) => {
                vec!["<locked>"]
            }
        };

        f.debug_struct("DependencyGraphNode")
            .field("enabled", &self.enabled)
            .field("dependencies", &dependency_names.join(", "))
            .field("dependants", &dependant_names.join(", "))
            // .field("wrapped", &self.wrapped) // HACK
            .finish()
    }
}

impl Display for DependencyGraphNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.wrapped.name(),
            if self.enabled { "enabled" } else { "disabled" }
        )
    }
}

impl Clone for DependencyGraphNode {
    fn clone(&self) -> Self {
        Self {
            top_level: self.top_level,
            enabled: self.enabled,
            dependencies: RwLock::new(self.dependencies.read().unwrap().clone()),
            dependants: RwLock::new(self.dependants.read().unwrap().clone()),
            wrapped: self.wrapped.clone(),
        }
    }
}

impl Deref for DependencyGraphNode {
    type Target = dyn DependencyInstallable;
    fn deref(&self) -> &Self::Target {
        self.wrapped
    }
}

impl DependencyGraphNode {
    fn new(wrapped: &'static dyn DependencyInstallable) -> Self {
        Self {
            top_level: false,
            enabled: false,
            dependencies: RwLock::new(vec![]),
            dependants: RwLock::new(vec![]),
            wrapped,
        }
    }

    fn add_dependency(&self, dependency: Arc<DependencyGraphNode>) {
        self.dependencies.write().unwrap().push(dependency);
    }

    fn add_dependant(&self, dependant: Arc<DependencyGraphNode>) {
        self.dependants.write().unwrap().push(dependant);
    }
}

impl DependencyGraphNode {
    pub fn children(&self) -> Vec<Arc<DependencyGraphNode>> {
        self.dependencies.read().unwrap().clone()
    }

    pub fn parents(&self) -> Vec<Arc<DependencyGraphNode>> {
        self.dependants.read().unwrap().clone()
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn name(&self) -> &'static str {
        self.wrapped.name()
    }

    pub fn top_level(&self) -> bool {
        self.top_level
    }

    pub fn set_top_level(&mut self, top_level: bool) {
        self.top_level = top_level;
    }
}

#[derive(Clone)]
pub struct DependencyGraph {
    nodes: HashMap<&'static str, Arc<DependencyGraphNode>>,
}

impl Debug for DependencyGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut nodes = vec![];
        for node in self.nodes.values() {
            // each node should be {name: "hello", enabled: true, dependencies: ["world", "foo"], dependants: ["bar", "baz"]}
            let format = format!(
                "{{name: {}, enabled: {}, dependencies: {:?}, dependants: {:?}}}",
                node.wrapped.name(),
                node.enabled,
                node.dependencies
                    .read()
                    .unwrap()
                    .iter()
                    .map(|n| n.name())
                    .collect::<Vec<&str>>(), //XXX: should be TryRead
                node.dependants
                    .read()
                    .unwrap()
                    .iter()
                    .map(|n| n.name())
                    .collect::<Vec<&str>>(), //XXX: should be TryRead
            );

            nodes.push(format);
        }

        f.debug_struct("DependencyGraph")
            .field("nodes", &nodes)
            .finish()
    }
}

impl Display for DependencyGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut nodes = vec![];
        for node in self.nodes.values() {
            nodes.push(format!("{}", node));
        }
        write!(f, "{}", nodes.join("\n"))
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for DependencyGraph {
    type Target = HashMap<&'static str, Arc<DependencyGraphNode>>;
    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

// XXX: allow iterator over nodes

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(
        &mut self,
        node: &'static dyn DependencyInstallable,
    ) -> Result<(), &'static dyn DependencyInstallable> {
        if self.nodes.contains_key(node.name()) {
            return Ok(());
        }

        // insert all dependencies as non-top-level nodes
        let node = Arc::new(DependencyGraphNode::new(node));
        for dependency in node.wrapped.requires() {
            let dependency_name = dependency.name();
            if !self.nodes.contains_key(dependency_name) {
                if let Err(e) = self.add_node(dependency) {
                    unreachable!(
                        "Unable to add dependency {} to graph: {}",
                        dependency_name,
                        e.name()
                    );
                }
            }
            let dependency_node = self.nodes.get(dependency_name).unwrap();
            node.add_dependency(dependency_node.clone());
            dependency_node.add_dependant(node.clone());
        }

        self.nodes.insert(node.wrapped.name(), node.clone());
        Ok(())
    }

    pub fn add_nodes(
        &mut self,
        nodes: Vec<&'static dyn DependencyInstallable>,
    ) -> Result<(), Vec<&'static dyn DependencyInstallable>> {
        let mut errs = vec![];
        for node in nodes {
            if let Err(e) = self.add_node(node) {
                errs.push(e);
            }
        }

        match errs.len() {
            0 => Ok(()),
            _ => Err(errs),
        }
    }

    /// Try to build a dependency view. Each column of the view has all it's dependencies satisfied
    /// by the previous column(s). This allows us to construct a dependency tree that can be
    /// installed in parallel, and visually present it in a pleasing way to the user.
    pub fn build_dependency_view(&self) -> Vec<Vec<Arc<DependencyGraphNode>>> {
        let mut contained: HashSet<&str> = HashSet::new();
        let mut columns = vec![];

        let mut to_insert = self.nodes.clone();

        while !to_insert.is_empty() {
            let mut next_column = HashMap::new();
            let this_loop = to_insert.clone();
            to_insert.clear();

            for (name, item) in this_loop.into_iter() {
                let parents = item.dependants.read().unwrap();

                // If this dependency has no parents, add it to the first column
                if parents.iter().all(|parent| {
                    let parent = parent.wrapped;
                    let name = parent.name();
                    contained.contains(name) && !next_column.contains_key(name)
                }) {
                    next_column.insert(name.to_owned(), item.clone());
                } else {
                    to_insert.insert(name, item.clone());
                    continue;
                }

                // Add all children to the queue
                contained.insert(name);
                let children = item.dependencies.read().unwrap();
                for child in children.iter() {
                    if !contained.contains(child.wrapped.name())
                        && !next_column.contains_key(child.wrapped.name())
                    {
                        to_insert.insert(child.wrapped.name(), child.clone());
                    }
                }
            }

            columns.push(next_column.into_values().collect());
        }

        columns
    }
}

// #[cfg(test)]
// mod test_build_dependency_graph {
//     use std::{
//         cell::RefCell,
//         fmt::Debug,
//         rc::{Rc, Weak},
//     };

//     use ntest::timeout;

//     use super::build_dependency_graph;

//     struct TestDependency {
//         name: String,
//         parents: RefCell<Vec<Weak<dyn Dependency>>>,
//         children: RefCell<Vec<Rc<dyn Dependency>>>,
//         self_ref: RefCell<Option<Rc<dyn Dependency>>>,
//     }

//     impl Debug for TestDependency {
//         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//             let parents = self.parents.borrow();
//             let parents = parents
//                 .clone()
//                 .into_iter()
//                 .map(|parent| {
//                     let parent = parent.upgrade().unwrap();
//                     parent.name().to_owned()
//                 })
//                 .collect::<Vec<_>>();
//             let children = self.children.borrow();
//             let children = children
//                 .iter()
//                 .map(|child| child.name())
//                 .collect::<Vec<_>>();

//             f.debug_struct("TestDependency")
//                 .field("name", &self.name)
//                 .field("depends_on", &children)
//                 .field("depended_on_by", &parents)
//                 .finish()
//         }
//     }

//     impl TestDependency {
//         fn new(name: &str) -> Rc<Self> {
//             let self_ref = Rc::new(Self {
//                 name: name.to_owned(),
//                 parents: RefCell::new(Vec::new()),
//                 children: RefCell::new(Vec::new()),
//                 self_ref: RefCell::new(None),
//             });

//             *self_ref.self_ref.borrow_mut() = Some(self_ref.clone());

//             self_ref
//         }
//     }

//     impl DependencyInfo for TestDependency {
//         fn name(&self) -> &str {
//             self.name.as_str()
//         }
//     }

//     impl Dependency for TestDependency {
//         fn dependencies(&self) -> Vec<Rc<dyn Dependency>> {
//             self.children.borrow().clone()
//         }

//         fn dependants(&self) -> Vec<Weak<dyn Dependency>> {
//             self.parents.borrow().clone()
//         }

//         fn add_dependency(&self, dependency: Rc<dyn Dependency>) {
//             let self_ref = self.self_ref.borrow().clone().unwrap();
//             dependency.add_dependant(Rc::downgrade(&self_ref));
//             self.children.borrow_mut().push(dependency);
//         }

//         fn add_dependant(&self, dependant: Weak<dyn Dependency>) {
//             self.parents.borrow_mut().push(dependant);
//         }

//         fn set_enabled(&self, _: bool) {
//             unreachable!()
//         }

//         fn is_enabled(&self) -> bool {
//             unreachable!()
//         }
//     }

//     #[test]
//     #[timeout(1000)]
//     fn test_build_dependency_graph_simple_a_b() {
//         // create two dependencies
//         let a = TestDependency::new("a");
//         let b = TestDependency::new("b");

//         a.add_dependency(b.clone());

//         let graph = build_dependency_graph(vec![a, b]);
//         println!("{:#?}", graph);

//         assert_eq!(graph.len(), 2);
//         assert_eq!(graph[0].len(), 1);
//         assert_eq!(graph[1].len(), 1);

//         assert_eq!(graph[0][0].name(), "a");
//         assert_eq!(graph[1][0].name(), "b");
//     }

//     #[test]
//     #[timeout(1000)]
//     fn test_build_dependency_graph_multiple_dependencies() {
//         let a = TestDependency::new("a");
//         let b = TestDependency::new("b");
//         let c = TestDependency::new("c");

//         a.add_dependency(b.clone());
//         a.add_dependency(c.clone());

//         let graph = build_dependency_graph(vec![a, b, c]);
//         println!("{:#?}", graph);

//         assert_eq!(graph.len(), 2);

//         assert_eq!(graph[0].len(), 1);
//         assert_eq!(graph[1].len(), 2);

//         assert_eq!(graph[0][0].name(), "a");
//         assert!(graph[1].iter().any(|dep| dep.name() == "b"));
//         assert!(graph[1].iter().any(|dep| dep.name() == "c"));
//     }

//     #[test]
//     #[timeout(1000)]
//     fn test_build_dependency_graph_simple_multilayer_dependencies() {
//         let a = TestDependency::new("a");
//         let b = TestDependency::new("b");
//         let c = TestDependency::new("c");

//         a.add_dependency(b.clone());
//         b.add_dependency(c.clone());

//         assert_eq!(a.dependencies().len(), 1);
//         assert_eq!(b.dependencies().len(), 1);
//         assert_eq!(c.dependencies().len(), 0);

//         assert!(a.dependants().is_empty());
//         assert_eq!(b.dependants().len(), 1);
//         assert_eq!(c.dependants().len(), 1);

//         assert_eq!(a.dependencies()[0].name(), "b");
//         assert_eq!(b.dependencies()[0].name(), "c");

//         let graph = build_dependency_graph(vec![a, c, b]);
//         println!("{:#?}", graph);

//         assert_eq!(graph.len(), 3);

//         assert_eq!(graph[0].len(), 1);
//         assert_eq!(graph[1].len(), 1);
//         assert_eq!(graph[2].len(), 1);

//         assert_eq!(graph[0][0].name(), "a");
//         assert_eq!(graph[1][0].name(), "b");
//         assert_eq!(graph[2][0].name(), "c");
//     }

//     #[test]
//     #[timeout(1000)]
//     fn test_build_dependency_graph_complex_multilayer_dependencies() {
//         let a = TestDependency::new("a");
//         let b = TestDependency::new("b");
//         let c = TestDependency::new("c");

//         a.add_dependency(b.clone());
//         a.add_dependency(c.clone());
//         b.add_dependency(c.clone());

//         let graph = build_dependency_graph(vec![c, b, a]);
//         println!("{:#?}", graph);

//         assert_eq!(graph.len(), 3);

//         assert_eq!(graph[0].len(), 1);
//         assert_eq!(graph[1].len(), 1);
//         assert_eq!(graph[2].len(), 1);

//         assert_eq!(graph[0][0].name(), "a");
//         assert_eq!(graph[1][0].name(), "b");
//         assert_eq!(graph[2][0].name(), "c");
//     }
// }
