use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{
    run_command, DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
};
use crate::{
    dependencies::{curl::Curl, package_cache_refresh::PackageCacheRefresh, zshrc::Zshrc},
    HOME_DIR,
};
use lazy_static::lazy_static;

const SCRIPT_URL: &str = "https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh";

lazy_static! {
    static ref NVM_PATH: String = format!("{}/.nvm/nvm.sh", *HOME_DIR);
    // TODO: make these global const paths
    static ref ZSHRC_PATH: String = format!("{}/.zshrc", *HOME_DIR);
    static ref BASHRC_PATH: String = format!("{}/.bashrc", *HOME_DIR);
}

#[derive(Debug, Default, Singleton)]
pub struct Nvm {
    // current_version: Option<String>,
    nvm_available: RwLock<bool>,
    nvm_path: RwLock<bool>,
}

impl DependencyInfo for Nvm {
    fn name(&self) -> &'static str {
        "nvm"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![PackageCacheRefresh::singleton(), Curl::singleton()]
    }

    fn optional(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![Zshrc::singleton()]
    }
}

impl DependencyInstallable for Nvm {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // check if nvm is installed by using `which nvm`
        let res = run_command("which", &vec!["nvm"])?;
        *self.nvm_available.write().unwrap() = res.success;

        // check if nvm is in EITHER of .zshrc or .bashrc
        let content = match Zshrc::singleton().is_installed()? {
            InstallationStatus::FullyInstalled => {
                std::fs::read_to_string(&*ZSHRC_PATH).unwrap_or_default()
            }
            _ => std::fs::read_to_string(&*BASHRC_PATH).unwrap_or_default(),
        };
        *self.nvm_path.write().unwrap() = content.to_lowercase().contains("nvm"); // XXX: is this good enough?

        match (
            *self.nvm_available.read().unwrap(),
            *self.nvm_path.read().unwrap(),
        ) {
            (true, true) => Ok(InstallationStatus::FullyInstalled),
            (false, false) => Ok(InstallationStatus::NotInstalled),
            _ => Ok(InstallationStatus::PartialInstall),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if *self.nvm_available.read().unwrap() && *self.nvm_path.read().unwrap() {
            return Ok(());
        }

        // Install NVM if installation is missing
        if *self.nvm_available.read().unwrap() == false {
            // curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash
            run_command("curl", &["-o-", SCRIPT_URL])?
                .error
                .map_or(Ok(()), |e| Err(e))?;
            *self.nvm_available.write().unwrap() = true;
        }

        // The script should have auto added the nvm path to .zshrc and .bashrc
        self.is_installed()?;

        if *self.nvm_path.read().unwrap() == false {
            return Err(DependencyError::DependencyFailed(
                "NVM path not found in .zshrc or .bashrc - should have been added by install script".to_string()
            ));
        }

        Ok(())
    }
}
