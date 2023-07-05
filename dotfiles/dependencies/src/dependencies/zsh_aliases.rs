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

const ZSH_ALIASES_REF: &str = include_str!("../assets/.zsh_aliases");

#[derive(Debug, Default, Singleton)]
pub struct ZshAliases {
    // current_version: Option<String>,
    zsh_aliases_available: RwLock<ConfigStatus>,
}

impl DependencyInfo for ZshAliases {
    fn name(&self) -> &'static str {
        "zsh_aliases"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![Zsh::singleton()]
    }
}

impl DependencyInstallable for ZshAliases {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // Check if zsh_aliases is present
        let zsh_aliases_path = format!("{}/.zsh_aliases", *HOME_DIR);
        let is_present = metadata(zsh_aliases_path.clone()).is_ok();

        // Check if zsh_aliases matches expected
        let mut all_present = false;
        if is_present {
            let zsh_aliases_contents = read_to_string(zsh_aliases_path.clone())?;

            // Check if all lines of the ZSH_ALIASES are present in the zsh_aliases file
            all_present = ZSH_ALIASES_REF
                .lines()
                .all(|line| zsh_aliases_contents.contains(line));
        }

        *self.zsh_aliases_available.write().unwrap() = match (is_present, all_present) {
            (false, _) => ConfigStatus::NotPresent,
            (true, false) => ConfigStatus::PresentIncorrect,
            (true, true) => ConfigStatus::PresentCorrect,
        };

        if matches!(
            *self.zsh_aliases_available.read().unwrap(),
            ConfigStatus::PresentCorrect
        ) {
            Ok(InstallationStatus::FullyInstalled)
        } else {
            Ok(InstallationStatus::NotInstalled)
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if matches!(
            *self.zsh_aliases_available.read().unwrap(),
            ConfigStatus::PresentCorrect
        ) {
            return Ok(());
        }

        // copy .zsh_aliases-base to ~/.zsh_aliases
        let zsh_aliases_path = format!("{}/.zsh_aliases", *HOME_DIR);
        let zsh_aliases_contents = read_to_string(&zsh_aliases_path).unwrap_or(String::new());
        if metadata(&zsh_aliases_path).is_ok() {
            rename_bak_file(&zsh_aliases_path)?;
        }

        std::fs::write(&zsh_aliases_path, ZSH_ALIASES_REF)?;

        // for each line in zsh_aliases_contents, check if it is present in .zsh_aliases if not, insert it
        for line in zsh_aliases_contents.lines() {
            if !ZSH_ALIASES_REF.contains(line) {
                std::fs::write(
                    &zsh_aliases_path,
                    format!("{}\n{}", zsh_aliases_contents, line),
                )?;
            }
        }

        *self.zsh_aliases_available.write().unwrap() = ConfigStatus::PresentCorrect;

        Ok(())
    }
}
