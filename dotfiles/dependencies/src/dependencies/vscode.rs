use std::{fs::metadata, sync::RwLock};

use singleton_derive::Singleton;

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::{AsUser, CommandError, DCommand},
    OperatingSystem, OPERATING_SYSTEM,
};
use reqwest;

const CODE_BINARY_PATH: &str = "/usr/bin/code";
const DEB_DOWNLOAD_URL: &str =
    "https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64";
const RPM_DOWNLOAD_URL: &str =
    "https://code.visualstudio.com/sha/download?build=stable&os=linux-rpm-x64";

#[derive(Debug, Default, Singleton)]
pub struct VsCode {
    // current_version: Option<String>,
    vscode_in_path: RwLock<bool>,
}

impl DependencyInfo for VsCode {
    fn name(&self) -> &'static str {
        "vscode"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![]
    }
}

impl DependencyInstallable for VsCode {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // check if vscode is present in file, or as cmd
        let is_present = metadata(CODE_BINARY_PATH).is_ok();
        let cmd_available = DCommand::new("which", &["code"]).run();
        // XXX: The following match statement has a hidden failure mode (what if cmd fails for a different reason?)
        let cmd_available = matches!(cmd_available, Err(CommandError::CommandFailed(_)));
        *self.vscode_in_path.write().unwrap() = is_present || cmd_available;
        match is_present || cmd_available {
            true => Ok(InstallationStatus::FullyInstalled),
            false => Ok(InstallationStatus::NotInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if *self.vscode_in_path.read().unwrap() {
            return Ok(());
        }

        // XXX: Could use something like the following syntax to download the file
        // let download_url = match crate::utils::get_os() {
        //     crate::utils::OS::Debian => DEB_DOWNLOAD_URL,
        //     crate::utils::OS::RedHat => RPM_DOWNLOAD_URL,
        //     _ => panic!("Unsupported OS"),
        // };
        // let download_path = crate::utils::download_file(download_url, "vscode.deb")?;

        let url = match *OPERATING_SYSTEM {
            // TODO: A check should be made to see if this operating system has a gui (e.g. Gnome)
            OperatingSystem::Ubuntu1804
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu2204
            | OperatingSystem::PopOS2104 => DEB_DOWNLOAD_URL,
            OperatingSystem::Fedora38 | OperatingSystem::Rocky8 | OperatingSystem::Rocky9 => {
                RPM_DOWNLOAD_URL
            }
            _ => return Err(DependencyError::UnsupportedOperatingSystem),
        };

        // use reqwest to download to /tmp/vscode.osinstall
        // XXX: Reqwest wrapped error is required - ideally with some global interface for handling downloading of files
        let mut req = reqwest::blocking::get(url)
            .map_err(|e| DependencyError::DependencyFailed(e.to_string()))?;
        let mut dest = std::fs::File::create("/tmp/vscode.osinstall")?;
        req.copy_to(&mut dest)
            .map_err(|e| DependencyError::DependencyFailed(e.to_string()))?;

        // install the file
        DCommand::new("dpkg", &["-i", "/tmp/vscode.osinstall"])
            .user(AsUser::Root)
            .run()?;

        *self.vscode_in_path.write().unwrap() = true;
        Ok(())
    }
}
