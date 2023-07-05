use std::{fs::metadata, sync::RwLock};

use lazy_static::lazy_static;
use singleton_derive::Singleton;

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::DCommand,
    dependencies::{git::Git, zsh::Zsh},
    HOME_DIR,
};

const OH_MY_ZSH_GITHUB_URL: &str = "https://github.com/ohmyzsh/ohmyzsh";

lazy_static! {
    static ref OH_MY_ZSH_PATH: String = format!("{}/.oh-my-zsh", *HOME_DIR);
}

#[derive(Debug, Default, Singleton)]
pub struct OhMyZsh {
    // current_version: Option<String>,
    ohmyzsh_installed: RwLock<bool>,
}

impl DependencyInfo for OhMyZsh {
    fn name(&self) -> &'static str {
        "oh-my-zsh"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![Git::singleton(), Zsh::singleton()]
    }
}

impl DependencyInstallable for OhMyZsh {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // Check if oh-my-zsh is installed
        let is_present = metadata(&*OH_MY_ZSH_PATH).is_ok();
        *self.ohmyzsh_installed.write().unwrap() = is_present;

        match is_present {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if *self.ohmyzsh_installed.read().unwrap() {
            return Ok(());
        }

        // Install oh-my-zsh by cloning the git repo
        DCommand::new(
            "git",
            &["clone", "--depth=1", OH_MY_ZSH_GITHUB_URL, &*OH_MY_ZSH_PATH],
        )
        .run()?;
        *self.ohmyzsh_installed.write().unwrap() = true;

        Ok(())
    }
}
