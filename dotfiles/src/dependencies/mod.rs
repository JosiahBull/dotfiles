use std::string::FromUtf8Error;

use async_trait::async_trait;

pub mod docker;
pub mod zsh;

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
            DependencyError::UnsupportedOperatingSystem => write!(f, "Unsupported operating system"),
            DependencyError::IoError(e) => write!(f, "IO error: {}", e),
            DependencyError::DependencyFailed(e) => write!(f, "Missing or unable to install required dependency: {}", e),
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
    NotInstalled
}


#[async_trait]
pub trait Dependency {
    /// Check if the dependency is installed on the current system.
    async fn is_installed(&mut self) -> Result<InstallationStatus, DependencyError>;

    async fn install(&mut self, version: Option<&str>) -> Result<(), DependencyError>;
    async fn uninstall(&mut self) -> Result<(), DependencyError>;
}