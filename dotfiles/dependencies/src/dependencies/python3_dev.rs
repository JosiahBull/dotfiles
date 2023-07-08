use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::DCommand,
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
            | OperatingSystem::Ubuntu2204 => DCommand::new("dpkg", &["-s", "python3-dev"]).run()?,
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
                DCommand::new("apt-get", &["install", "-y", "python3-dev"]).run()?;
            }
            // TODO
            _ => return Err(DependencyError::UnsupportedOperatingSystem),
        }

        Ok(())
    }
}
