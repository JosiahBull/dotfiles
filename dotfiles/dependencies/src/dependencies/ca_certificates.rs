use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::{CommandError, DCommand},
    dependencies::package_cache_refresh::PackageCacheRefresh,
    OperatingSystem, OPERATING_SYSTEM,
};

#[derive(Debug, Default, Singleton)]
pub struct CaCertificates {
    // current_version: Option<String>,
    ca_certificates_available: RwLock<bool>,
}

impl DependencyInfo for CaCertificates {
    fn name(&self) -> &'static str {
        "ca-certificates"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![PackageCacheRefresh::singleton()]
    }
}

impl DependencyInstallable for CaCertificates {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        let res = DCommand::new("which", &["update-ca-certificates"]).run();
        let res = matches!(res, Err(CommandError::CommandFailed(_)));
        *self.ca_certificates_available.write().unwrap() = res;
        match res {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if *self.ca_certificates_available.read().unwrap() {
            return Ok(());
        }

        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu1804
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu2204 => {
                DCommand::new("apt-get", &["install", "-y", "ca-certificates"]).run()?;
                DCommand::new("update-ca-certificates", &[]).run()?;
            }
            _ => return Err(DependencyError::UnsupportedOperatingSystem),
        }

        *self.ca_certificates_available.write().unwrap() = true;

        Ok(())
    }
}
