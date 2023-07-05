use std::{fs::metadata, sync::RwLock};

use singleton_derive::Singleton;

use super::{
    run_command, DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
};
use crate::dependencies::{
    pip3::Pip3, python3::Python3, python3_dev::Python3Dev, setuptools::SetupTools,
};

const THE_FUCK_PATH: &str = "/usr/local/bin/thefuck";

#[derive(Debug, Default, Singleton)]
pub struct TheFuck {
    // current_version: Option<String>,
    the_fuck_available: RwLock<bool>,
}

impl DependencyInfo for TheFuck {
    fn name(&self) -> &'static str {
        "the-fuck"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![
            Pip3::singleton(),
            Python3::singleton(),
            SetupTools::singleton(),
            Python3Dev::singleton(),
        ]
    }
}

impl DependencyInstallable for TheFuck {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // check if THE_FUCK_PATH exists, of if "which thefuck" works
        let if_path_exists = metadata(THE_FUCK_PATH).is_ok();
        let cmd_available = run_command("which", &vec!["thefuck"])?.success;

        *self.the_fuck_available.write().unwrap() = if_path_exists || cmd_available;
        match if_path_exists || cmd_available {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if *self.the_fuck_available.read().unwrap() {
            return Ok(());
        }

        run_command(
            "python3",
            &vec!["-m", "pip3", "install", "thefuck", "--user"],
        )?
        .error
        .map_or(Ok(()), |e| Err(e))?;
        *self.the_fuck_available.write().unwrap() = true;
        Ok(())
    }
}
