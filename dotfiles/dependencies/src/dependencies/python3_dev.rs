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
pub struct Python3Dev {
    // current_version: Option<String>,
    python3_dev_available: RwLock<bool>,
}

impl DependencyInfo for Python3Dev {
    fn name(&self) -> &'static str {
        "python3-dev"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![PackageCacheRefresh::singleton(), Python3::singleton()]
    }
}

impl DependencyInstallable for Python3Dev {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        let res = match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu1804
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu2204 => run_command("dpkg", &vec!["-s", "python3-dev"])?,
            // FIXME
            _ => return Err(DependencyError::UnsupportedOperatingSystem),
        };

        *self.python3_dev_available.write().unwrap() = res.success;
        match res.success {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if *self.python3_dev_available.read().unwrap() {
            return Ok(());
        }

        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu1804
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu2204 => {
                run_command("apt-get", &vec!["install", "-y", "python3-dev"])?
                    .error
                    .map_or(Ok(()), |e| Err(e))?;
            }
            // TODO
            _ => return Err(DependencyError::UnsupportedOperatingSystem),
        }

        Ok(())
    }
}
