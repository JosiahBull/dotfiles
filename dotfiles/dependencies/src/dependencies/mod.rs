use std::{
    collections::HashMap,
    fmt::{Debug, Display, self},
    fs::metadata,
    io::{BufRead, Read},
    string::FromUtf8Error,
    sync::{Arc, Weak, RwLock, TryLockError},
    ops::Deref, borrow::BorrowMut, cell::RefCell,
};

use lazy_static::__Deref;

use crate::command::{CommandError, Output};

/* General System Dependencies */
pub mod gcc;
pub mod package_cache_refresh;

/* Docker + Dependencies */
pub mod apt_transport_https;
pub mod ca_certificates;
pub mod curl;
// pub mod docker;
// pub mod gnupg;

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
// pub mod pip3;
// pub mod python3;
// pub mod python3_dev;
// pub mod setuptools;
// pub mod thefuck;

/* Rust + Applications */
pub mod bat;
pub mod rust;
// pub mod tokei;
// pub mod zoxide;

/* Firefox */
// pub mod firefox;
// pub mod firefox_config;

/* NodeJs + Applications */
// pub mod nodejs;
// pub mod nvm;
// pub mod yarn;
// pub mod redoc_cli;

/* SSH + Git */
pub mod git;
// pub mod gitconfig;
// pub mod ssh;
// pub mod openssh;
// pub mod authorized_keys;
// pub mod ed25519key;
// pub mod sshconfig;

/* Misc */
// pub mod scripts;
// pub mod vscode;

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
    fn optional(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![]
    }
}

pub trait DependencyInstallable: DependencyInfo {
    /// Check if the dependency is installed on the current system.
    /// Updates internal state to reflect the current status.
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError>;

    /// Install the dependency.
    fn install(&self) -> Result<(), DependencyError>;
}

// pub trait Dependency: std::fmt::Debug + DependencyInfo {
//     /// Get a list of all dependencies that this application requires
//     // fn dependencies<'b>(&'b self) -> &'b[&'b dyn DependencyGraph];
//     fn dependencies(&self) -> Vec<Rc<dyn Dependency>>;

//     /// Get a list of dependants that require this application
//     // FIXME: this can use self: Rc<Self> instead of &self
//     fn dependants(&self) -> Vec<Weak<dyn Dependency>>;

//     /// Add a dependency to this application
//     fn add_dependency(&self, dependency: Rc<dyn Dependency>);

//     /// Add a dependant to this application
//     fn add_dependant(&self, dependant: Weak<dyn Dependency>);

//     /// Enable or disable this dependency
//     fn set_enabled(&self, enabled: bool);

//     /// Check if this dependency is enabled
//     fn is_enabled(&self) -> bool;
// }

pub struct DependencyGraphNode<'a, T: DependencyInstallable> {
    enabled: bool,
    dependencies: RwLock<Vec<&'a DependencyGraphNode<'a, T>>>,
    dependants: RwLock<Vec<&'a DependencyGraphNode<'a, T>>>,
    wrapped: T,
}

impl<T: DependencyInstallable + Debug> Debug for DependencyGraphNode<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dependency_names = match self.dependencies.try_read() {
            Ok(guard) => {
                let mut names = vec![];
                for dependency in guard.iter() {
                    names.push(dependency.wrapped.name());
                }
                names
            },
            Err(TryLockError::Poisoned(err)) => {
                let mut names = vec![];
                for dependency in err.into_inner().iter() {
                    names.push(dependency.wrapped.name());
                }
                names
            },
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
            },
            Err(TryLockError::Poisoned(err)) => {
                let mut names = vec![];
                for dependency in err.into_inner().iter() {
                    names.push(dependency.wrapped.name());
                }
                names
            },
            Err(TryLockError::WouldBlock) => {
                vec!["<locked>"]
            }
        };

        f.debug_struct("DependencyGraphNode")
            .field("enabled", &self.enabled)
            .field("dependencies", &dependency_names.join(", "))
            .field("dependants", &dependant_names.join(", "))
            .field("wrapped", &self.wrapped)
            .finish()
    }
}

impl<T: DependencyInstallable + Display> Display for DependencyGraphNode<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "{}: {}",
            self.wrapped.name(),
            if self.enabled { "enabled" } else { "disabled" }
        )
    }
}

impl<T: DependencyInstallable + Clone> Clone for DependencyGraphNode<'_, T> {
    fn clone(&self) -> Self {
        Self {
            enabled: self.enabled,
            dependencies: RwLock::new(self.dependencies.read().unwrap().clone()),
            dependants: RwLock::new(self.dependants.read().unwrap().clone()),
            wrapped: self.wrapped.clone(),
        }
    }
}

impl<T: DependencyInstallable> Deref for DependencyGraphNode<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.wrapped
    }
}

impl<'a, T: DependencyInstallable> DependencyGraphNode<'a, T> {
    fn new(wrapped: T) -> Self {
        Self {
            enabled: false,
            dependencies: RwLock::new(vec![]),
            dependants: RwLock::new(vec![]),
            wrapped,
        }
    }

    fn add_dependency(&self, dependency: &'a DependencyGraphNode<'a, T>) {
        self.dependencies.write().unwrap().push(dependency);
    }

    fn add_dependant(&self, dependant: &'a DependencyGraphNode<'a, T>) {
        self.dependants.write().unwrap().push(dependant);
    }
}

pub struct DependencyGraph<'a, T: DependencyInstallable> {
    nodes: HashMap<&'static str, &'a DependencyGraphNode<'a, T>>,
    top_level_nodes: Vec<&'a DependencyGraphNode<'a, T>>,
}

impl<'a, T: DependencyInstallable> DependencyGraph<'a, T> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            top_level_nodes: vec![],
        }
    }

    pub fn add_node(&mut self, node: &'a DependencyGraphNode<'a, T>) {
        self.nodes.insert(node.wrapped.name(), node);
        self.top_level_nodes.push(&node);

        for dependency in node.wrapped.requires() {
            let dependency = *self.nodes.get_mut(dependency.name()).unwrap();
            node.add_dependency(dependency);
            dependency.add_dependant(node);
        }
    }

    pub fn get(&self, name: &'static str) -> Option<&'a DependencyGraphNode<'a, T>> {
        self.nodes.get(name).copied()
    }

    pub fn is_top_level(&self, name: &'static str) -> bool {
        self.top_level_nodes.iter().any(|node| node.wrapped.name() == name)
    }

    pub fn top_nodes(&self) -> Vec<&'a DependencyGraphNode<'a, T>> {
        self.top_level_nodes.clone()
    }
}

impl<'a, T: DependencyInstallable> Deref for DependencyGraph<'a, T> {
    type Target = HashMap<&'static str, &'a DependencyGraphNode<'a, T>>;
    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}
