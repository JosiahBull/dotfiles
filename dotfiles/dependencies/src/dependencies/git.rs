use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::{CommandError, DCommand},
    dependencies::package_cache_refresh::PackageCacheRefresh,
    OperatingSystem, OPERATING_SYSTEM,
};

#[derive(Debug, Default, Singleton)]
pub struct Git {
    // current_version: Option<String>,
    git_available: RwLock<bool>,
}

impl DependencyInfo for Git {
    //TODO: auto derive this
    fn name(&self) -> &'static str {
        "git"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![PackageCacheRefresh::singleton()]
    }
}

impl DependencyInstallable for Git {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // check if git is installed by using `which git`
        let res = DCommand::new("which", &["git"]).run();
        let res = matches!(res, Err(CommandError::CommandFailed(_)));
        *self.git_available.write().unwrap() = res;
        match res {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if *self.git_available.read().unwrap() {
            return Ok(());
        }

        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu1804
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu2204 => {
                DCommand::new("apt-get", &["install", "-y", "git"]).run()?;
                *self.git_available.write().unwrap() = true;
            }
            //TODO: support other operating systems, with fallback to compile from source.
            _ => return Err(DependencyError::UnsupportedOperatingSystem),
        }

        Ok(())
    }
}
