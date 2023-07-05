use std::{
    fs::{metadata, read_to_string},
    sync::RwLock,
};

use singleton_derive::Singleton;

use super::{
    rename_bak_file, ConfigStatus, DependencyError, DependencyInfo, DependencyInstallable,
    InstallationStatus,
};
use crate::HOME_DIR;
use lazy_static::lazy_static;

const GITHUB_KEYS_URL: &str = "https://github.com/josiahbull.keys";

lazy_static! {
    static ref AUTHORIZED_KEYS_PATH: String = format!("{}/.ssh/authorized_keys", *HOME_DIR);
    // HACK: Significantly shrink the binary by baking this in as a const, with an OPTION for CURL at runtime.
    static ref AUTHORIZED_KEYS_CONTENT: Vec<String> = {
        // use reqwest to get the authorized keys from github
        let resp = reqwest::blocking::get(GITHUB_KEYS_URL).unwrap();

        // check if the request was successful
        if !resp.status().is_success() {
            panic!("Failed to get authorized keys from github");
        }

        // split the response into lines
        resp.text().unwrap().split("\n").map(|s| s.to_string()).collect()
    };
}

#[derive(Debug, Default, Singleton)]
pub struct AuthorizedKeys {
    authorized_keys_available: RwLock<ConfigStatus>,
}

impl DependencyInfo for AuthorizedKeys {
    fn name(&self) -> &'static str {
        "authorized_keys"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![
            //TODO
            // Ssh::singleton(),
            // OpenSsh::singleton(),
        ]
    }
}

impl DependencyInstallable for AuthorizedKeys {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // Check if authorized keys file is present
        let is_present = metadata(&*AUTHORIZED_KEYS_PATH).is_ok();
        let mut is_correct = true;
        if is_present {
            // check that all lines in the authorized keys file are present
            let authorized_keys_contents = read_to_string(&*AUTHORIZED_KEYS_PATH)?;
            for key in AUTHORIZED_KEYS_CONTENT.iter() {
                if !authorized_keys_contents.contains(key) {
                    is_correct = false;
                    break;
                }
            }
        }

        let config = match (is_present, is_correct) {
            (true, true) => ConfigStatus::PresentCorrect,
            (_, false) => ConfigStatus::PresentIncorrect,
            _ => ConfigStatus::NotPresent,
        };
        *self.authorized_keys_available.write().unwrap() = config;

        match config {
            ConfigStatus::PresentCorrect => Ok(InstallationStatus::FullyInstalled),
            ConfigStatus::PresentIncorrect => Ok(InstallationStatus::PartialInstall),
            ConfigStatus::NotPresent => Ok(InstallationStatus::FullyInstalled),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if matches!(
            *self.authorized_keys_available.read().unwrap(),
            ConfigStatus::PresentCorrect
        ) {
            return Ok(());
        }

        // if the file is not present, create it
        if !metadata(&*AUTHORIZED_KEYS_PATH).is_ok() {
            std::fs::File::create(&*AUTHORIZED_KEYS_PATH)?;
        } else {
            // backup the file
            rename_bak_file(&*AUTHORIZED_KEYS_PATH)?;
            let bak_path = format!("{}.bak", &*AUTHORIZED_KEYS_PATH);
            std::fs::copy(bak_path, &*AUTHORIZED_KEYS_PATH)?;
        }

        // for each line not present in the authorized keys file, add it
        let mut authorized_keys_contents = read_to_string(&*AUTHORIZED_KEYS_PATH)?;
        for key in AUTHORIZED_KEYS_CONTENT.iter() {
            if !authorized_keys_contents.contains(key) {
                authorized_keys_contents.push_str(&format!("\n{}", key));
            }
        }
        std::fs::write(&*AUTHORIZED_KEYS_PATH, authorized_keys_contents)?;

        Ok(())
    }
}
