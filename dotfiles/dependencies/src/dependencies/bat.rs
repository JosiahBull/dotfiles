use std::{fs::metadata, sync::RwLock};

use singleton_derive::Singleton;

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::{AsUser, CommandError, DCommand},
    dependencies::rust::Rust,
    gcc::Gcc,
    HOME_DIR,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref BAT_PATH: String = format!("{}/.cargo/bin/bat", *HOME_DIR);
}

#[derive(Debug, Default, Singleton)]
pub struct Bat {
    // current_version: Option<String>,
    bat_available: RwLock<bool>,
}

impl DependencyInfo for Bat {
    fn name(&self) -> &'static str {
        "bat"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        // XXX: there are many precompiled binaries, we should look at using them
        vec![Rust::singleton(), Gcc::singleton()]
    }
}

impl DependencyInstallable for Bat {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // check if bat is present in file, or as cmd
        let is_present = metadata(&*BAT_PATH).is_ok();
        let cmd_available = DCommand::new("which", &["bat"]).run();
        let cmd_available = matches!(cmd_available, Err(CommandError::CommandFailed(_)));
        *self.bat_available.write().unwrap() = is_present || cmd_available;
        match is_present || cmd_available {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if *self.bat_available.read().unwrap() {
            return Ok(());
        }

        let cargo_path = format!("{}/.cargo/bin/cargo", *HOME_DIR);
        DCommand::new(cargo_path.as_str(), &["install", "bat"])
            .user(AsUser::DefaultUser)
            .run()?;

        *self.bat_available.write().unwrap() = true;

        Ok(())
    }
}
