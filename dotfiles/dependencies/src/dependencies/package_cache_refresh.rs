use std::sync::RwLock;

use singleton_derive::Singleton;

use crate::{command::DCommand, OperatingSystem, OPERATING_SYSTEM};

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};

#[derive(Debug, Default, Singleton)]
pub struct PackageCacheRefresh {
    updated: RwLock<bool>,
}

impl DependencyInfo for PackageCacheRefresh {
    fn name(&self) -> &'static str {
        "package-cache-refresh"
    }
}

impl DependencyInstallable for PackageCacheRefresh {
    fn is_installed(&self) -> Result<super::InstallationStatus, super::DependencyError> {
        match *self.updated.read().unwrap() {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu1804
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu2204 => DCommand::new("apt-get", &["update"]).run()?,
            OperatingSystem::Fedora38 | OperatingSystem::Rocky8 | OperatingSystem::Rocky9 => {
                DCommand::new("dnf", &["clean", "expire-cache"]).run()?
            }
            _ => return Err(DependencyError::UnsupportedOperatingSystem),
        };

        *self.updated.write().unwrap() = true;

        Ok(())
    }
}
