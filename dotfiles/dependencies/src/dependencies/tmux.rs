use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::{CommandError, DCommand},
    dependencies::package_cache_refresh::PackageCacheRefresh,
    OperatingSystem, OPERATING_SYSTEM,
};

#[derive(Debug, Default, Singleton)]
pub struct Tmux {
    // current_version: Option<String>,
    tmux_available: RwLock<bool>,
}

impl DependencyInfo for Tmux {
    fn name(&self) -> &'static str {
        "tmux"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![PackageCacheRefresh::singleton()]
    }
}

impl DependencyInstallable for Tmux {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        let res = DCommand::new("which", &["tmux"]).run();
        let res = matches!(res, Err(CommandError::CommandFailed(_)));
        *self.tmux_available.write().unwrap() = res;
        match res {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if *self.tmux_available.read().unwrap() {
            return Ok(());
        }

        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu1804
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu2204 => {
                DCommand::new("apt-get", &["install", "-y", "tmux"]).run()?;
            }
            _ => {
                return Err(DependencyError::UnsupportedOperatingSystem);
            }
        }

        *self.tmux_available.write().unwrap() = true;

        Ok(())
    }
}
