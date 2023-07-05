use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::{CommandError, DCommand},
    dependencies::package_cache_refresh::PackageCacheRefresh,
    OperatingSystem, OPERATING_SYSTEM,
};

#[derive(Debug, Default, Singleton)]
pub struct AptTransportHttps {
    // current_version: Option<String>,
    apt_transport_https_available: RwLock<bool>,
}

impl DependencyInfo for AptTransportHttps {
    fn name(&self) -> &'static str {
        "apt-transport-https"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![PackageCacheRefresh::singleton()]
    }
}

// Could rename this to `Dependency`
impl DependencyInstallable for AptTransportHttps {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        let res = DCommand::new("which", &["apt-transport-https"]).run();
        let res = matches!(res, Err(CommandError::CommandFailed(_)));
        *self.apt_transport_https_available.write().unwrap() = res;
        match res {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if *self.apt_transport_https_available.read().unwrap() {
            return Ok(());
        }

        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu1804
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu2204 => {
                DCommand::new("apt-get", &["install", "-y", "apt-transport-https"]).run()?;
            }
            _ => return Err(DependencyError::UnsupportedOperatingSystem),
        }

        *self.apt_transport_https_available.write().unwrap() = true;

        Ok(())
    }
}
