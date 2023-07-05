use std::{fs::metadata, sync::RwLock};

use singleton_derive::Singleton;

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::DCommand,
    dependencies::{git::Git, ohmyzsh::OhMyZsh, zsh::Zsh},
    HOME_DIR,
}; //XXX: shouldn't this be config directory (does zsh always place the .zshrc file in the home dir?
use lazy_static::lazy_static;

const ZSH_AUTO_SUGGESTIONS_GITHUB_PATH: &str = "https://github.com/zsh-users/zsh-autosuggestions";

lazy_static! {
    static ref ZSH_AUTO_SUGGESTIONS_PATH: String = format!(
        "{}/.oh-my-zsh/custom/plugins/zsh-autosuggestions",
        *HOME_DIR
    );
}

#[derive(Debug, Default, Singleton)]
pub struct ZshAutoSuggestions {
    // current_version: Option<String>,
    zsh_auto_suggestions_available: RwLock<bool>,
}

impl DependencyInfo for ZshAutoSuggestions {
    fn name(&self) -> &'static str {
        "zsh_auto_suggestions"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![Zsh::singleton(), OhMyZsh::singleton(), Git::singleton()]
    }
}

impl DependencyInstallable for ZshAutoSuggestions {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // check if zsh-syntax-highlighting is present
        let is_present = metadata(&*ZSH_AUTO_SUGGESTIONS_PATH).is_ok();
        *self.zsh_auto_suggestions_available.write().unwrap() = is_present;
        match is_present {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        DCommand::new(
            "git",
            &[
                "clone",
                "--depth=1",
                ZSH_AUTO_SUGGESTIONS_GITHUB_PATH,
                &*ZSH_AUTO_SUGGESTIONS_PATH,
            ],
        )
        .run()?;
        *self.zsh_auto_suggestions_available.write().unwrap() = true;
        Ok(())
    }
}
