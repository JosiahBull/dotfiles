use std::{fs::metadata, sync::RwLock};

use singleton_derive::Singleton;

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::{CommandError, DCommand},
    dependencies::{pip3::Pip3, python3::Python3, python3_dev::Python3Dev, setuptools::SetupTools},
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
        let cmd_available = DCommand::new("which", &["thefuck"]).run();
        let cmd_available = matches!(cmd_available, Err(CommandError::CommandFailed(_)));

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
        DCommand::new("python3", &["-m", "pip3", "install", "thefuck", "--user"]).run()?;
        *self.the_fuck_available.write().unwrap() = true;
        Ok(())
    }
}
