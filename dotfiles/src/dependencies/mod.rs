use std::{
    string::FromUtf8Error,
    sync::{Arc, Weak},
};

pub mod docker;
// pub mod zsh;

#[derive(Debug)]
pub enum DependencyError {
    Unknown,
    NotInstalled,
    UnsupportedOperatingSystem,
    IoError(std::io::Error),
    DependencyFailed(String),
    Utf8Error(FromUtf8Error),
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
    // XXX: new?

    /// Get the name of the dependency.
    fn name(&self) -> &'static str;
    // XXX: handle versioning, etc
}

pub trait DependencyInstallable {
    /// Check if this package is installable on the current system.
    fn installable(&self) -> Result<Installable, DependencyError>;

    /// Check if the dependency is installed on the current system.
    /// Updates internal state to reflect the current status.
    fn is_installed(&mut self) -> Result<InstallationStatus, DependencyError>;

    /// Install the dependency.
    fn install(&mut self, version: Option<&str>) -> Result<(), DependencyError>;

    /// Uninstall the dependency.
    fn uninstall(&mut self) -> Result<(), DependencyError>;
}

pub trait DependencyGraph: std::fmt::Debug + Sync + Send {
    /// Get a list of all dependencies that this application requires
    // fn dependencies<'b>(&'b self) -> &'b[&'b dyn DependencyGraph];
    fn dependencies(&self) -> Vec<Arc<dyn DependencyGraph>>;

    /// Get a list of dependants that require this application
    fn dependants(&self) -> Vec<Weak<dyn DependencyGraph>>;

    /// Add a dependency to this application
    fn add_dependency(&self, dependency: Arc<dyn DependencyGraph>);

    /// Add a dependant to this application
    fn add_dependant(&self, dependant: Weak<dyn DependencyGraph>);

    /// Enable or disable this dependency
    fn set_enabled(&self, enabled: bool);

    /// Check if this dependency is enabled
    // XXX: this might be able to have a default implementation, with a slight rename.
    fn is_enabled(&self) -> bool;
}

// auto implement it :)
pub trait Dependency: DependencyInfo + DependencyInstallable + DependencyGraph {}
impl<T> Dependency for T where T: DependencyInfo + DependencyInstallable + DependencyGraph {}

#[cfg(test)]
mod test_graphing_functions {
    use super::DependencyGraph;

    trait Graphable {
        fn name(&self) -> String;
    }

    // fn isolate_for_display(top_level_dependencies: &[&dyn DependencyGraph], target_dependency: &str) -> Vec<Vec<Box<dyn Graphable>>> {
    //     // recursively walk the dependency graph, and find the target dependency
    // }
}
