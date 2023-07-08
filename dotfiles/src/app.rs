use std::{
    error,
    sync::Arc,
};

use dependencies::{DependencyGraphNode, DependencyGraph, all_top_level};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// All dependencies, nicely organised
    pub dependency_view: Vec<Vec<Arc<DependencyGraphNode>>>,
    /// X Y loc of view
    pub view_loc: (f64, f64),
}

impl Default for App {
    fn default() -> Self {
        let mut dep_graph = DependencyGraph::new();
        dep_graph.add_nodes(all_top_level()).unwrap();
        let dependency_view = dep_graph.build_dependency_view();

        Self { running: true, dependency_view, view_loc: (0.0, 0.0) }
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
