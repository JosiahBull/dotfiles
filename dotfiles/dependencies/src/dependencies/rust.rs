use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::{AsUser, DCommand},
    dependencies::{curl::Curl, package_cache_refresh::PackageCacheRefresh},
};

const RUST_URL: &str = "https://sh.rustup.rs";

#[derive(Debug, Default, Singleton)]
pub struct Rust {
    rust_available: RwLock<bool>,
    cargo_available: RwLock<bool>,
}

impl DependencyInfo for Rust {
    fn name(&self) -> &'static str {
        "rust"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![PackageCacheRefresh::singleton(), Curl::singleton()]
    }
}

impl DependencyInstallable for Rust {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // check if rust is installed by using `which rustc`
        let res_rustc = DCommand::new("which", &["rustc"]).run()?;
        *self.rust_available.write().unwrap() = res_rustc.success;

        // check if cargo is installed by using `which cargo`
        let res_cargo = DCommand::new("which", &["cargo"]).run()?;
        *self.cargo_available.write().unwrap() = res_cargo.success;

        // if rustc is installed and NOT cargo, throw an unresolvable error
        if res_rustc.success && !res_cargo.success {
            return Err(DependencyError::DependencyFailed(String::from("This system seems to have rustc installed, but not cargo. This is an unresolvable error.")));
        }

        match (res_rustc.success, res_cargo.success) {
            (true, true) => Ok(InstallationStatus::FullyInstalled),
            (false, false) => Ok(InstallationStatus::NotInstalled),
            _ => Ok(InstallationStatus::PartialInstall),
        }
    }

    fn install(&self) -> Result<(), DependencyError> {
        if *self.rust_available.read().unwrap() && *self.cargo_available.read().unwrap() {
            return Ok(());
        }
        // curl https://sh.rustup.rs -sSf | sh -s -- -y"
        DCommand::new("curl", &[RUST_URL, "-sSf", "|", "sh", "-s", "--", "-y"])
            .user(AsUser::DefaultUser)
            .run()?;
        *self.rust_available.write().unwrap() = true;
        *self.cargo_available.write().unwrap() = true;

        Ok(())
    }
}
