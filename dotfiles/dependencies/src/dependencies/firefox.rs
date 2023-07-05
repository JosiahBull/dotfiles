use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{
    run_command, DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
};
use crate::{
    dependencies::package_cache_refresh::PackageCacheRefresh, OperatingSystem, OPERATING_SYSTEM,
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
        let res = run_command("which", &vec!["firefox"])?;
        *self.firefox_available.write().unwrap() = res.success;
        match res.success {
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
                run_command("apt-get", &vec!["install", "-y", "firefox"])?
                    .error
                    .map_or(Ok(()), |e| Err(e))?;
                *self.firefox_available.write().unwrap() = true;
            }
            OperatingSystem::Fedora38 | OperatingSystem::Rocky8 | OperatingSystem::Rocky9 => {
                run_command("dnf", &vec!["install", "-y", "firefox"])?
                    .error
                    .map_or(Ok(()), |e| Err(e))?;
                *self.firefox_available.write().unwrap() = true;
            }
        }

        Ok(())
    }
}
