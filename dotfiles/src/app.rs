use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    error,
    rc::{Rc, Weak},
};

use dependencies::dependencies::DependencyInfo;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

// fn build_dependency_graph(
//     dependencies: Vec<Rc<dyn Dependency + 'static>>,
// ) -> Vec<Vec<Rc<dyn Dependency + 'static>>> {
//     let mut contained = HashSet::new();
//     let mut columns = vec![];

//     // The goal is for each dependency to only have it's dependants in the next column.

//     let mut to_insert = dependencies
//         .into_iter()
//         .map(|x| (x.name().to_owned(), x))
//         .collect::<HashMap<_, _>>();
//     while !to_insert.is_empty() {
//         let mut next_column = HashMap::new();
//         let this_loop = to_insert.clone();
//         to_insert.clear();

//         for (name, item) in this_loop.into_iter() {
//             let parents = item.dependants();

//             // If this dependency has no parents, add it to the first column
//             if parents.iter().all(|parent| {
//                 let parent = parent.upgrade().unwrap();
//                 let name = parent.name();
//                 contained.contains(name) && !next_column.contains_key(name)
//             }) {
//                 next_column.insert(name.to_owned(), item.clone());
//             } else {
//                 to_insert.insert(name, item.clone());
//                 continue;
//             }

//             // Add all children to the queue
//             contained.insert(name.to_owned());
//             let children = item.dependencies();
//             for child in children {
//                 if !contained.contains(child.name()) && !next_column.contains_key(child.name()) {
//                     to_insert.insert(child.name().to_owned(), child.clone());
//                 }
//             }
//         }

//         columns.push(next_column.into_values().collect());
//     }

//     columns
// }

// #[cfg(test)]
// mod test_build_dependency_graph {
//     use std::{
//         cell::RefCell,
//         fmt::Debug,
//         rc::{Rc, Weak},
//     };

//     use ntest::timeout;

//     use crate::dependencies::{Dependency, DependencyInfo};

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

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { running: true }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
