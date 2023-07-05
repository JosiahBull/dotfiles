use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::{AsUser, CommandError, DCommand},
    dependencies::{git::Git, package_cache_refresh::PackageCacheRefresh, tmux::Tmux},
    OperatingSystem, OPERATING_SYSTEM,
};

#[derive(Debug, Default, Singleton)]
pub struct Zsh {
    // current_version: Option<String>,
    zsh_base_installed: RwLock<bool>,
    zsh_chsh: RwLock<bool>,
}

impl DependencyInfo for Zsh {
    fn name(&self) -> &'static str {
        "zsh"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![
            Tmux::singleton(),
            Git::singleton(),
            PackageCacheRefresh::singleton(),
        ]
    }
}

impl DependencyInstallable for Zsh {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // Check if zsh application is installed
        let res_installed = DCommand::new("which", &["zsh"]).run();
        let res_installed = matches!(res_installed, Err(CommandError::CommandFailed(_)));
        *self.zsh_base_installed.write().unwrap() = res_installed;

        // Check if zsh is the default shell
        let res_default = DCommand::new("echo", &["$SHELL"]).run()?;
        *self.zsh_chsh.write().unwrap() = res_default.success && res_default.stdout.contains("zsh");

        match (res_installed, res_default.success) {
            (true, true) => Ok(InstallationStatus::FullyInstalled),
            (true, false) => Ok(InstallationStatus::PartialInstall),
            (false, _) => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if !*self.zsh_base_installed.read().unwrap() {
            match *OPERATING_SYSTEM {
                OperatingSystem::Ubuntu1804
                | OperatingSystem::Ubuntu2004
                | OperatingSystem::Ubuntu2204
                | OperatingSystem::PopOS2104 => {
                    DCommand::new("apt-get", &["install", "-y", "zsh"]).run()?;
                }
                OperatingSystem::Fedora38 => {
                    DCommand::new("dnf", &["install", "-y", "zsh"]).user(AsUser::DoNothing).run()?;
                }
                _ => return Err(DependencyError::UnsupportedOperatingSystem),
            }

            *self.zsh_base_installed.write().unwrap() = true;
        }

        if !*self.zsh_chsh.read().unwrap() {
            DCommand::new("chsh", &["-s", "/usr/bin/zsh"])
                .user(AsUser::DoNothing)
                .run()?;
            *self.zsh_chsh.write().unwrap() = true;
        }

        Ok(())
    }
}
