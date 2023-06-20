use async_trait::async_trait;
use log::{trace, warn};

use super::{Dependency, InstallationStatus, DependencyError};
use crate::{HOME_DIR, OPERATING_SYSTEM, OperatingSystem};


#[derive(Debug)]
pub enum ConfigStatus {
    NotPresent,
    PresentIncorrect,
    PresentCorrect,
}

#[derive(Debug)]
pub struct Zsh {
    zsh_base_installed: bool,
    zshrc_present: ConfigStatus,
    zshaliases_present: ConfigStatus,

    p10k_installed: bool,
    p10k_config: ConfigStatus,

    ohmyzsh_installed: bool,

    zsh_autosuggestions_installed: bool,
    zsh_syntax_highlighting_installed: bool,

    tmux_available: bool,
    tmux_plugin_installed: bool,
    git_available: bool,
    git_plugin_installed: bool,
}

impl Zsh {
    pub fn new() -> Self {
        Self {
            zsh_base_installed: false,
            zshrc_present: ConfigStatus::NotPresent,
            zshaliases_present: ConfigStatus::NotPresent,

            p10k_installed: false,
            p10k_config: ConfigStatus::NotPresent,

            ohmyzsh_installed: false,

            zsh_autosuggestions_installed: false,
            zsh_syntax_highlighting_installed: false,

            tmux_available: false,
            tmux_plugin_installed: false,
            git_available: false,
            git_plugin_installed: false,
        }
    }
}

#[async_trait]
impl Dependency for Zsh {
    async fn is_installed(&mut self) -> Result<InstallationStatus, DependencyError> {
        let zsh_config = include_str!("../assets/.zshrc-base");
        let zsh_aliases = include_str!("../assets/.zsh_aliases");
        let p10k_config = include_str!("../assets/.p10k.zsh");

        // Check if tmux and git are installed
        let output = tokio::process::Command::new("which")
            .arg("tmux")
            .output()
            .await?;
        self.tmux_available = output.status.success();

        let output = tokio::process::Command::new("which")
            .arg("git")
            .output()
            .await?;
        self.git_available = output.status.success();

        // Check if zsh application is installed
        let output = tokio::process::Command::new("zsh")
            .arg("--version")
            .output()
            .await?;

        // outputs: zsh 5.8 (x86_64-ubuntu-linux-gnu)
        let stdout = String::from_utf8(output.stdout)?;

        if !output.status.success() || stdout.is_empty() || !stdout.contains("zsh") {
            trace!("zsh is not installed");
            self.zsh_base_installed = false;
        } else {
            trace!("zsh is installed");
            self.zsh_base_installed = true;
        }

        // Check if zshrc is present
        let zshrc_path = format!("{}/.zshrc", *HOME_DIR);
        let is_present = tokio::fs::metadata(zshrc_path.clone()).await.is_ok();

        // Check if zshrc matches expected
        let zshrc_contents = tokio::fs::read_to_string(zshrc_path.clone()).await?;

        // Check if the first 23 lines of the zshrc match the expected - the rest is user-specific
        let mut is_correct = zshrc_contents.lines().take(33).eq(zsh_config.lines().take(33));

        // if tmux is installed, check if the tmux plugin is present in the zshrc
        if self.tmux_available {
            let tmux_plugin = "tmux";
            self.tmux_plugin_installed = zshrc_contents.contains(tmux_plugin);

            // if tmux not present in zshrc, set is_correct to false
            if !self.tmux_plugin_installed {
                is_correct = false;
            }
        }

        // if git is installed, check if the git plugin is present in the zshrc
        if self.git_available {
            let git_plugin = "git";
            self.git_plugin_installed = zshrc_contents.contains(git_plugin);

            // if git not present in zshrc, set is_correct to false
            if !self.git_plugin_installed {
                is_correct = false;
            }
        }

        self.zshrc_present = match (is_present, is_correct) {
            (false, _) => ConfigStatus::NotPresent,
            (true, false) => ConfigStatus::PresentIncorrect,
            (true, true) => ConfigStatus::PresentCorrect,
        };

        // Check if zsh_aliases is present
        let zsh_aliases_path = format!("{}/.zsh_aliases", *HOME_DIR);
        let is_present = tokio::fs::metadata(zsh_aliases_path.clone()).await.is_ok();

        // Check if zsh_aliases matches expected
        let zsh_aliases_contents = tokio::fs::read_to_string(zsh_aliases_path.clone()).await.unwrap_or_else(|_| String::from(""));

        // Only compare the first 11 lines of the zsh_aliases file
        let is_correct = zsh_aliases_contents.lines().take(11).eq(zsh_aliases.lines().take(11));

        self.zshaliases_present = match (is_present, is_correct) {
            (false, _) => ConfigStatus::NotPresent,
            (true, false) => ConfigStatus::PresentIncorrect,
            (true, true) => ConfigStatus::PresentCorrect,
        };

        // Check if p10k config is present
        let p10k_path = format!("{}/.p10k.zsh", *HOME_DIR);
        let is_present = tokio::fs::metadata(p10k_path.clone()).await.is_ok();

        // Check if p10k matches expected
        let p10k_contents = tokio::fs::read_to_string(p10k_path.clone()).await.unwrap_or_else(|_| String::from(""));
        let is_correct = p10k_contents == p10k_config;

        self.p10k_config = match (is_present, is_correct) {
            (false, _) => ConfigStatus::NotPresent,
            (true, false) => ConfigStatus::PresentIncorrect,
            (true, true) => ConfigStatus::PresentCorrect,
        };

        // Check if oh-my-zsh is installed
        let ohmyzsh_path = format!("{}/.oh-my-zsh", *HOME_DIR);
        let is_present = tokio::fs::metadata(ohmyzsh_path.clone()).await.is_ok();
        self.ohmyzsh_installed = is_present;

        // Check if p10k git repo is present
        let p10k_path = format!("{}/.oh-my-zsh/custom/themes/powerlevel10k", *HOME_DIR);
        let is_present = tokio::fs::metadata(p10k_path.clone()).await.is_ok();
        self.p10k_installed = is_present;

        // Check if zsh-autosuggestions is installed
        let zsh_autosuggestions_path = format!("{}/.oh-my-zsh/custom/plugins/zsh-autosuggestions", *HOME_DIR);
        let is_present = tokio::fs::metadata(zsh_autosuggestions_path.clone()).await.is_ok();
        self.zsh_autosuggestions_installed = is_present;

        // Check if zsh-syntax-highlighting is installed
        let zsh_syntax_highlighting_path = format!("{}/.oh-my-zsh/custom/plugins/zsh-syntax-highlighting", *HOME_DIR);
        let is_present = tokio::fs::metadata(zsh_syntax_highlighting_path.clone()).await.is_ok();
        self.zsh_syntax_highlighting_installed = is_present;

        // if zsh is not installed, return not installed
        if !self.zsh_base_installed {
            return Ok(InstallationStatus::NotInstalled);
        }

        // if all sub-dependencies are installed, return installed
        if matches!(self.zshrc_present, ConfigStatus::PresentCorrect)
            && matches!(self.zshaliases_present, ConfigStatus::PresentCorrect)
            && matches!(self.p10k_config, ConfigStatus::PresentCorrect)
            && self.ohmyzsh_installed
            && self.p10k_installed
            && self.zsh_autosuggestions_installed
            && self.zsh_syntax_highlighting_installed
            && (!self.tmux_available || self.tmux_plugin_installed)
            && (!self.git_available || self.git_plugin_installed)
        {
            return Ok(InstallationStatus::FullyInstalled);
        }

        Ok(InstallationStatus::PartialInstall)
    }

    async fn install(&mut self, version: Option<&str>) -> Result<(), DependencyError> {
        if version.is_some() {
            warn!("Zsh::install() does not support version specification");
        }

        if !self.zsh_base_installed {
            match *OPERATING_SYSTEM {
                OperatingSystem::Ubuntu1804 | OperatingSystem::Ubuntu2004 | OperatingSystem::Ubuntu2204 | OperatingSystem::PopOS2104 => {
                    let output = tokio::process::Command::new("sudo")
                        .arg("apt")
                        .arg("update")
                        .output()
                        .await
                        ?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed(String::from("apt update")));
                    }

                    let output = tokio::process::Command::new("sudo")
                        .arg("apt")
                        .arg("install")
                        .arg("-y")
                        .arg("zsh")
                        .output()
                        .await
                        ?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed(String::from("apt install zsh")));
                    }

                    self.zsh_base_installed = true;
                },
                OperatingSystem::Fedora38 => {
                    let output = tokio::process::Command::new("sudo")
                        .arg("dnf")
                        .arg("install")
                        .arg("-y")
                        .arg("zsh")
                        .output()
                        .await
                        ?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed(String::from("dnf install zsh")));
                    }

                    self.zsh_base_installed = true;
                },
                _ => return Err(DependencyError::UnsupportedOperatingSystem)
            }
        }

        todo!()
    }

    async fn uninstall(&mut self) -> Result<(), DependencyError> {
        unimplemented!("Zsh::uninstall() not implemented");
    }
}