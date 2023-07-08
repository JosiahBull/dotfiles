use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::{CommandError, DCommand},
    dependencies::package_cache_refresh::PackageCacheRefresh,
    OperatingSystem, OPERATING_SYSTEM,
};

#[derive(Debug, Default, Singleton)]
pub struct Firefox {
    // current_version: Option<String>,
    firefox_available: RwLock<bool>,
}

impl DependencyInfo for Firefox {
    fn name(&self) -> &'static str {
        "firefox"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![PackageCacheRefresh::singleton()]
    }
}

impl DependencyInstallable for Firefox {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // check if firefox is installed by using `which firefox`
        let res = DCommand::new("which", &["firefox"]).run();
        let res = matches!(res, Err(CommandError::CommandFailed(_)));
        *self.firefox_available.write().unwrap() = res;
        match res {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if *self.firefox_available.read().unwrap() {
            return Ok(());
        }

        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu1804
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu2204
            | OperatingSystem::PopOS2104 => {
                DCommand::new("apt-get", &["install", "-y", "firefox"]).run()?;
                *self.firefox_available.write().unwrap() = true;
            }
            OperatingSystem::Fedora38 | OperatingSystem::Rocky8 | OperatingSystem::Rocky9 => {
                DCommand::new("dnf", &["install", "-y", "firefox"]).run()?;
                *self.firefox_available.write().unwrap() = true;
            }
        }

        Ok(())
    }
}
