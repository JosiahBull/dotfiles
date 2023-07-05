use std::{
    fs::{metadata, read_to_string},
    sync::RwLock,
};

use singleton_derive::Singleton;

use super::{
    rename_bak_file, ConfigStatus, DependencyError, DependencyInfo, DependencyInstallable,
    InstallationStatus,
};
use crate::{dependencies::zsh::Zsh, HOME_DIR}; //XXX: shouldn't this be config directory (does zsh always place the .zshrc file in the home dir?

const ZSH_CONFIG_BASE: &str = include_str!("../assets/.zshrc-base");

#[derive(Debug, Default, Singleton)]
pub struct Zshrc {
    // current_version: Option<String>,
    zshrc_available: RwLock<ConfigStatus>,
}

impl DependencyInfo for Zshrc {
    fn name(&self) -> &'static str {
        "zshrc"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        // XXX: test that each lad can operate with nothing but it's dependencies installed.
        vec![Zsh::singleton()]
    }
}

impl DependencyInstallable for Zshrc {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // check if the zshrc file is present

        // Check if zshrc is present
        let zshrc_path = format!("{}/.zshrc", *HOME_DIR);
        let is_present = metadata(zshrc_path.clone()).is_ok();

        // Check if zshrc matches expected
        let mut is_correct = false;
        if is_present {
            let zshrc_contents = read_to_string(zshrc_path.clone())?;
            // Check if the first 23 lines of the zshrc match the expected - the rest is user-specific
            is_correct = zshrc_contents
                .lines()
                .take(33)
                .eq(ZSH_CONFIG_BASE.lines().take(33));
        }

        *self.zshrc_available.write().unwrap() = match (is_present, is_correct) {
            (false, _) => ConfigStatus::NotPresent,
            (true, false) => ConfigStatus::PresentIncorrect,
            (true, true) => ConfigStatus::PresentCorrect,
        };

        if matches!(
            *self.zshrc_available.read().unwrap(),
            ConfigStatus::PresentCorrect
        ) {
            Ok(InstallationStatus::FullyInstalled)
        } else {
            Ok(InstallationStatus::NotInstalled)
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if matches!(
            *self.zshrc_available.read().unwrap(),
            ConfigStatus::PresentCorrect
        ) {
            return Ok(());
        }

        // copy .zshrc-base to ~/.zshrc
        let zshrc_path = format!("{}/.zshrc", *HOME_DIR);
        if metadata(zshrc_path.clone()).is_ok() {
            rename_bak_file(&zshrc_path)?;
        }

        // Write zshrc to file
        std::fs::write(zshrc_path, ZSH_CONFIG_BASE)?;

        // XXX: add additions tidbits based on what's installed

        Ok(())
    }
}
