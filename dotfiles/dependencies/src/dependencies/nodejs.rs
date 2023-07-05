use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{
    run_command, DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
};
use crate::dependencies::{nvm::Nvm, package_cache_refresh::PackageCacheRefresh};

#[derive(Debug, Default, Singleton)]
pub struct NodeJs {
    // current_version: Option<String>,
    nodejs_available: RwLock<bool>,
}

impl DependencyInfo for NodeJs {
    fn name(&self) -> &'static str {
        "nodejs"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![PackageCacheRefresh::singleton(), Nvm::singleton()]
    }
}

impl DependencyInstallable for NodeJs {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // check if nodejs is installed by using `which nodejs`
        let res = run_command("which", &vec!["node"])?;
        *self.nodejs_available.write().unwrap() = res.success;
        match res.success {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        // Install using NVM
        if *self.nodejs_available.read().unwrap() {
            return Ok(());
        }

        run_command("nvm", &vec!["install", "node"])?
            .error
            .map_or(Ok(()), |e| Err(e))?;

        *self.nodejs_available.write().unwrap() = true;

        Ok(())
    }
}
