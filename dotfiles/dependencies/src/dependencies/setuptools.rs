use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{
    run_command, DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
};
use crate::{
    dependencies::{
        package_cache_refresh::PackageCacheRefresh, python3::Python3, python3_dev::Python3Dev,
    },
    OperatingSystem, OPERATING_SYSTEM,
};

#[derive(Debug, Default, Singleton)]
pub struct SetupTools {
    // current_version: Option<String>,
    setup_tools_available: RwLock<bool>,
}

impl DependencyInfo for SetupTools {
    fn name(&self) -> &'static str {
        "setuptools"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![
            PackageCacheRefresh::singleton(),
            Python3::singleton(),
            Python3Dev::singleton(),
        ]
    }
}

impl DependencyInstallable for SetupTools {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        let res = run_command("python3", &vec!["-m", "pip", "show", "setuptools"])?;
        *self.setup_tools_available.write().unwrap() = res.success;
        match res.success {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if *self.setup_tools_available.read().unwrap() {
            return Ok(());
        }

        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu1804
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu2204 => {
                run_command("apt-get", &vec!["install", "-y", "python3-setuptools"])?
                    .error
                    .map_or(Ok(()), |e| Err(e))?;
            }
            // TODO
            _ => return Err(DependencyError::UnsupportedOperatingSystem),
        }
        *self.setup_tools_available.write().unwrap() = true;
        Ok(())
    }
}
