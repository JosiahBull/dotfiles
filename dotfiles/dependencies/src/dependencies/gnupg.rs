use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::{CommandError, DCommand},
    dependencies::package_cache_refresh::PackageCacheRefresh,
    OperatingSystem, OPERATING_SYSTEM,
};

#[derive(Debug, Default, Singleton)]
pub struct Gnupg {
    // current_version: Option<String>,
    gpg_available: RwLock<bool>,
}

impl DependencyInfo for Gnupg {
    fn name(&self) -> &'static str {
        "GnuPG"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![PackageCacheRefresh::singleton()]
    }
}

impl DependencyInstallable for Gnupg {
    fn is_installed(&self) -> Result<super::InstallationStatus, super::DependencyError> {
        // check if gnupg is installed by using `which gpg`
        let res = DCommand::new("which", &["gpg"]).run();
        let res = matches!(res, Err(CommandError::CommandFailed(_)));
        *self.gpg_available.write().unwrap() = res;
        match res {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), super::DependencyError> {
        if *self.gpg_available.read().unwrap() {
            return Ok(());
        }

        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu1804
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu2204 => {
                DCommand::new("apt-get", &["install", "-y", "gnupg"]).run()?;
                *self.gpg_available.write().unwrap() = true;
            }
            //TODO: support other operating systems, with fallback to compile from source.
            _ => return Err(DependencyError::UnsupportedOperatingSystem),
        }

        Ok(())
    }
}
