use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{
    run_command, DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
};
use crate::{
    dependencies::{package_cache_refresh::PackageCacheRefresh, python3::Python3},
    OperatingSystem, OPERATING_SYSTEM,
};

#[derive(Debug, Default, Singleton)]
pub struct Pip3 {
    // current_version: Option<String>,
    pip3_available: RwLock<bool>,
}

impl DependencyInfo for Pip3 {
    fn name(&self) -> &'static str {
        "pip3"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![PackageCacheRefresh::singleton(), Python3::singleton()]
    }
}

impl DependencyInstallable for Pip3 {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // check if pip3 is installed by using `which pip3`
        let res = run_command("which", &vec!["pip3"])?;
        *self.pip3_available.write().unwrap() = res.success;
        match res.success {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if *self.pip3_available.read().unwrap() {
            return Ok(());
        }

        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu1804
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu2204 => {
                run_command("apt-get", &vec!["install", "-y", "python3-pip"])?
                    .error
                    .map_or(Ok(()), |e| Err(e))?;
                *self.pip3_available.write().unwrap() = true;
            }
            //TODO: support other operating systems, with fallback to compile from source.
            _ => return Err(DependencyError::UnsupportedOperatingSystem),
        }

        Ok(())
    }
}
