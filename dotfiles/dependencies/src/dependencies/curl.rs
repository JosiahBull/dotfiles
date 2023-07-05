use std::sync::RwLock;

use singleton_derive::Singleton;

use crate::{
    command::{CommandError, DCommand},
    dependencies::package_cache_refresh::PackageCacheRefresh,
    OperatingSystem, OPERATING_SYSTEM,
};

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};

#[derive(Debug, Default, Singleton)]
pub struct Curl {
    // current_version: Option<String>,
    curl_available: RwLock<bool>,
}

impl DependencyInfo for Curl {
    fn name(&self) -> &'static str {
        "curl"
    }
    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![PackageCacheRefresh::singleton()]
    }
}

impl DependencyInstallable for Curl {
    fn is_installed(&self) -> Result<super::InstallationStatus, super::DependencyError> {
        // check if curl is installed by using `which curl`

        // TODO: add check for `curl` being available as a binary on this OS
        // TODO: add check for 'universe' being installed. This may make sense as a derive function.

        let res = DCommand::new("which", &["curl"]).run();
        let res = matches!(res, Err(CommandError::CommandFailed(_)));
        *self.curl_available.write().unwrap() = res;
        match res {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), super::DependencyError> {
        if *self.curl_available.read().unwrap() {
            return Ok(());
        }

        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu1804
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu2204 => {
                DCommand::new("apt-get", &["install", "-y", "curl"]).run()?;
            }
            //TODO: support other operating systems, with fallback to compile from source.
            _ => return Err(DependencyError::UnsupportedOperatingSystem),
        }

        *self.curl_available.write().unwrap() = true;

        Ok(())
    }
}
