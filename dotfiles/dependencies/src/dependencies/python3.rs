use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{
    run_command, DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
};
use crate::{
    dependencies::package_cache_refresh::PackageCacheRefresh, OperatingSystem, OPERATING_SYSTEM,
};

#[derive(Debug, Default, Singleton)]
pub struct Python3 {
    // current_version: Option<String>,
    python3_available: RwLock<bool>,
}

impl DependencyInfo for Python3 {
    fn name(&self) -> &'static str {
        "python3"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![PackageCacheRefresh::singleton()]
    }
}

impl DependencyInstallable for Python3 {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // check if python3 is installed by using `which python3`
        let res = run_command("which", &vec!["python3"])?;
        *self.python3_available.write().unwrap() = res.success;
        match res.success {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if *self.python3_available.read().unwrap() {
            return Ok(());
        }

        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu1804
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu2204 => {
                run_command("apt-get", &vec!["install", "-y", "python3"])?
                    .error
                    .map_or(Ok(()), |e| Err(e))?;
                *self.python3_available.write().unwrap() = true;
            }
            //TODO: support other operating systems, with fallback to compile from source.
            _ => return Err(DependencyError::UnsupportedOperatingSystem),
        }

        Ok(())
    }
}
