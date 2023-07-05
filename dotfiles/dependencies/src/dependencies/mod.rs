use std::{
    fmt::{Debug, Display},
    fs::metadata,
    io::{BufRead, Read},
    string::FromUtf8Error,
};

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
