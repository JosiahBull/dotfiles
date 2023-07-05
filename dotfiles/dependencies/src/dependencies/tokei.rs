use std::{fs::metadata, sync::RwLock};

use singleton_derive::Singleton;

use super::{
    run_command, DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
};
use crate::{dependencies::rust::Rust, HOME_DIR};
use lazy_static::lazy_static;

lazy_static! {
    static ref BAT_PATH: String = format!("{}/.cargo/bin/tokei", *HOME_DIR);
}

#[derive(Debug, Default, Singleton)]
pub struct Tokei {
    // current_version: Option<String>,
    tokei_available: RwLock<bool>,
}

impl DependencyInfo for Tokei {
    fn name(&self) -> &'static str {
        "tokei"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        // XXX: there are many precompiled binaries, we should look at using them
        vec![Rust::singleton()]
    }
}

impl DependencyInstallable for Tokei {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // check if tokei is present in file, or as cmd
        let is_present = metadata(&*BAT_PATH).is_ok();
        let cmd_available = run_command("which", &vec!["tokei"])?.success;
        *self.tokei_available.write().unwrap() = is_present || cmd_available;
        match is_present || cmd_available {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if *self.tokei_available.read().unwrap() {
            return Ok(());
        }

        run_command("cargo", &["install", "tokei"])?
            .error
            .map_or(Ok(()), |e| Err(e))?;
        *self.tokei_available.write().unwrap() = true;

        Ok(())
    }
}
