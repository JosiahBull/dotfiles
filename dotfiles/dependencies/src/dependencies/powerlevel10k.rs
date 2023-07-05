use std::{
    fs::{metadata, read_to_string},
    sync::RwLock,
};

use singleton_derive::Singleton;

use super::{
    rename_bak_file, ConfigStatus, DependencyError, DependencyInfo, DependencyInstallable,
    InstallationStatus,
};
use crate::{
    command::DCommand,
    dependencies::{ohmyzsh::OhMyZsh, zsh::Zsh},
    HOME_DIR,
}; //XXX: shouldn't this be config directory (does zsh always place the .zshrc file in the home dir?
use lazy_static::lazy_static;

const POWER_LEVEL_10K_GITHUB_URL: &str = "https://gitee.com/romkatv/powerlevel10k.git";
const POWER_LEVEL_10K_CONFIG_BASE: &str = include_str!("../assets/.p10k.zsh");

lazy_static! {
    static ref P10K_CONFIG_PATH: String = format!("{}/.p10k.zsh", *HOME_DIR);
    static ref P10K_FILES_PATH: String =
        format!("{}/.oh-my-zsh/custom/themes/powerlevel10k", *HOME_DIR);
}

#[derive(Debug, Default, Singleton)]
pub struct PowerLevel10k {
    // current_version: Option<String>,
    p10k_config_available: RwLock<ConfigStatus>,
    p10k_repo_installed: RwLock<bool>,
}

impl DependencyInfo for PowerLevel10k {
    fn name(&self) -> &'static str {
        "powerlevel10k"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![Zsh::singleton(), OhMyZsh::singleton()]
    }
}

impl DependencyInstallable for PowerLevel10k {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // check if p10k config is present
        let is_present = metadata(&*P10K_CONFIG_PATH).is_ok();

        // Check if p10k config matches expected
        let mut is_correct = false;
        if is_present {
            let p10k_contents = read_to_string(&*P10K_CONFIG_PATH)?;
            is_correct = p10k_contents == POWER_LEVEL_10K_CONFIG_BASE;
        }

        *self.p10k_config_available.write().unwrap() = match (is_present, is_correct) {
            (false, _) => ConfigStatus::NotPresent,
            (true, false) => ConfigStatus::PresentIncorrect,
            (true, true) => ConfigStatus::PresentCorrect,
        };

        // check that the p10k files are present
        let is_present = metadata(&*P10K_FILES_PATH).is_ok();
        *self.p10k_repo_installed.write().unwrap() = is_present;

        match (
            &*self.p10k_config_available.read().unwrap(),
            &*self.p10k_repo_installed.read().unwrap(),
        ) {
            (ConfigStatus::PresentCorrect, true) => Ok(InstallationStatus::FullyInstalled),
            (ConfigStatus::NotPresent, false) => Ok(InstallationStatus::NotInstalled),
            _ => Ok(InstallationStatus::PartialInstall),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if matches!(
            *self.p10k_config_available.read().unwrap(),
            ConfigStatus::PresentCorrect
        ) && *self.p10k_repo_installed.read().unwrap()
        {
            return Ok(());
        }

        if !matches!(
            *self.p10k_config_available.read().unwrap(),
            ConfigStatus::PresentCorrect
        ) {
            // copy .p10k.zsh to ~/.p10k.zsh
            if metadata(&*P10K_CONFIG_PATH).is_ok() {
                rename_bak_file(&*P10K_CONFIG_PATH)?;
            }

            // Write the new p10k config
            std::fs::write(&*P10K_CONFIG_PATH, POWER_LEVEL_10K_CONFIG_BASE)?;

            *self.p10k_config_available.write().unwrap() = ConfigStatus::PresentCorrect;
        }

        if !*self.p10k_repo_installed.read().unwrap() {
            // Install powerlevel10k by cloning the git repo
            // XXX: Move git to it's own command, we use it a lot
            DCommand::new(
                "git",
                &[
                    "clone",
                    "--depth=1",
                    POWER_LEVEL_10K_GITHUB_URL,
                    &*P10K_FILES_PATH,
                ],
            )
            .run()?;
            *self.p10k_repo_installed.write().unwrap() = true;
        }

        Ok(())
    }
}
