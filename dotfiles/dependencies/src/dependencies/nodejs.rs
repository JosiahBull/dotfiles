use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::{CommandError, DCommand},
    dependencies::{nvm::Nvm, package_cache_refresh::PackageCacheRefresh},
};

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
        let res = DCommand::new("which", &["node"]).run();
        let res = matches!(res, Err(CommandError::CommandFailed(_)));
        *self.nodejs_available.write().unwrap() = res;
        match res {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        // Install using NVM
        if *self.nodejs_available.read().unwrap() {
            return Ok(());
        }

        DCommand::new("nvm", &["install", "node"]).run()?;
        *self.nodejs_available.write().unwrap() = true;

        Ok(())
    }
}
