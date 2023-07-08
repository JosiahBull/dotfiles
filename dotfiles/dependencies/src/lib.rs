pub mod command;
mod dependencies;
pub mod system_data;

// Make all dependencies top-level for convenience
// High Level
use dependencies::*;
pub use dependencies::{
    AsUser, CommandResult, DependencyGraph, DependencyGraphNode, DependencyInstallable,
    InstallationStatus,
};

// Individual Applications
pub use apt_transport_https::AptTransportHttps;
// pub use authorized_key_updater::AuthorizedKeyUpdater;
pub use authorized_keys::AuthorizedKeys;
pub use bat::Bat;
pub use ca_certificates::CaCertificates;
pub use curl::Curl;
pub use docker::Docker;
pub use ed25519key::Ed25519Key;
// pub use firefox_config::FirefoxConfig;
pub use firefox::Firefox;
pub use gcc::Gcc;
pub use git::Git;
// pub use gitconfig::GitConfig;
pub use gnupg::Gnupg;
pub use nodejs::NodeJs;
pub use nvm::Nvm;
pub use ohmyzsh::OhMyZsh;
pub use package_cache_refresh::PackageCacheRefresh;
pub use pip3::Pip3;
pub use powerlevel10k::PowerLevel10k;
pub use python3::Python3;
pub use python3_dev::Python3Dev;
pub use rust::Rust;
// pub use scripts::Scripts;
pub use setuptools::SetupTools;
// pub use sshconfig::SshConfig;
pub use thefuck::TheFuck;
pub use tmux::Tmux;
pub use tokei::Tokei;
pub use vscode::VsCode;
pub use yarn::Yarn;
// pub use zoxide::Zoxide;
pub use zsh::Zsh;
pub use zsh_aliases::ZshAliases;
pub use zsh_autosuggestions::ZshAutoSuggestions;
pub use zsh_syntax_highlighting::ZshSyntaxHighlighting;
pub use zshrc::Zshrc;

// Other Imports
use lazy_static::lazy_static;
use sysinfo::SystemExt;

use crate::command::DCommand;

// TODO: refactor into various supporting files //

lazy_static! {
    pub static ref OPERATING_SYSTEM: OperatingSystem =
        OperatingSystem::from_sysinfo().expect("Unable to determine operating system");
    pub static ref CURRENT_USER: String = {
        // get the current user through sudo by checking SUDO_USER env var, if it doesn't exist
        // then use whoami to acquire the current user and roll with that. :)
        std::env::var("SUDO_USER").unwrap_or_else(|_|whoami::username())
    };
    pub static ref HOME_DIR: String = {
        // Get home dir of *CURRENT_USER*
        let res = DCommand::new("getent", &["passwd", &*CURRENT_USER])
            .run()
            .expect("able to run shell command");
        assert!(res.success, "Unable to get home dir due to an error. Stdout {} stderr {}", res.stdout, res.stderr);
        let home_dir: String = res.stdout.split(':').nth(5).unwrap().to_string();
        home_dir
    };
}

#[derive(Debug)]
enum DotfilesError {
    UnknownOperatingSystem(String),
    UnsupportedOperatingSystem,
}

#[derive(Debug)]
pub enum OperatingSystem {
    Ubuntu2204,
    Ubuntu2004,
    Ubuntu1804,

    Fedora38,

    Rocky9,
    Rocky8,

    PopOS2104,
}

impl OperatingSystem {
    fn from_sysinfo() -> Result<Self, DotfilesError> {
        let system = sysinfo::System::new_all();

        // print out the current system information
        println!("System name:             {:?}", system.name());
        println!("System kernel version:   {:?}", system.kernel_version());
        println!("System OS version:       {:?}", system.os_version());
        println!("System host name:        {:?}", system.host_name());
        println!("System uptime:           {}", system.uptime());
        println!("System number of users:  {}", system.users().len());
        println!("System processes:        {}", system.processes().len());
        println!("System total memory:     {} kB", system.total_memory());
        println!("System free memory:      {} kB", system.free_memory());

        if let Some(os) = system.long_os_version() {
            match os.as_str() {
                "Linux 22.04 Ubuntu" => Ok(OperatingSystem::Ubuntu2204),
                "Linux 20.04 Ubuntu" => Ok(OperatingSystem::Ubuntu2004),
                "Linux 18.04 Ubuntu" => Ok(OperatingSystem::Ubuntu1804),

                "Linux 38 Fedora Linux" => Ok(OperatingSystem::Fedora38),

                "Linux 9 Rocky" => Ok(OperatingSystem::Rocky9),
                "Linux 8 Rocky" => Ok(OperatingSystem::Rocky8),

                "Linux 21.04 Pop!_OS" => Ok(OperatingSystem::PopOS2104),
                _ => Err(DotfilesError::UnknownOperatingSystem(os)),
            }
        } else {
            Err(DotfilesError::UnknownOperatingSystem(
                "Unable to determine operating system".to_string(),
            ))
        }
    }
}

pub fn all_top_level() -> Vec<&'static dyn DependencyInstallable> {
    vec![
        // authorized_key_updater::AuthorizedKeyUpdater::singleton(),
        authorized_keys::AuthorizedKeys::singleton(),
        bat::Bat::singleton(),
        docker::Docker::singleton(),
        ed25519key::Ed25519Key::singleton(),
        firefox::Firefox::singleton(),
        git::Git::singleton(),
        // gitconfig::GitConfig::singleton(),
        nodejs::NodeJs::singleton(),
        ohmyzsh::OhMyZsh::singleton(),
        powerlevel10k::PowerLevel10k::singleton(),
        // scripts::Scripts::singleton(),
        // sshconfig::SshConfig::singleton(),
        thefuck::TheFuck::singleton(),
        tmux::Tmux::singleton(),
        tokei::Tokei::singleton(),
        vscode::VsCode::singleton(),
        // zoxide::Zoxide::singleton(),
        // XXX: create "zsh collection" to be the top level
        zsh_aliases::ZshAliases::singleton(),
        zsh_autosuggestions::ZshAutoSuggestions::singleton(),
        zsh_syntax_highlighting::ZshSyntaxHighlighting::singleton(),
        zsh::Zsh::singleton(),
        zshrc::Zshrc::singleton(),
    ]
}
