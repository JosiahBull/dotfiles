use singleton_derive::Singleton;
use std::{fs::metadata, sync::RwLock};

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::{CommandError, DCommand},
    dependencies::{nodejs::NodeJs, package_cache_refresh::PackageCacheRefresh, zshrc::Zshrc},
    OperatingSystem, HOME_DIR, OPERATING_SYSTEM,
};
use lazy_static::lazy_static;

const YARN_PATH: &str =
    "export PATH=\"$HOME/.yarn/bin:$HOME/.config/yarn/global/node_modules/.bin:$PATH\"";
lazy_static! {
    static ref YARN_BIN_PATH: String = format!("{}/.yarn/bin/yarn", *HOME_DIR);
    static ref ZSHRC_PATH: String = format!("{}/.zshrc", *HOME_DIR);
    static ref BASHRC_PATH: String = format!("{}/.bashrc", *HOME_DIR);
}

#[derive(Debug, Default, Singleton)]
pub struct Yarn {
    // current_version: Option<String>,
    yarn_available: RwLock<bool>,
    yarn_in_path: RwLock<bool>,
}

impl DependencyInfo for Yarn {
    fn name(&self) -> &'static str {
        "yarn"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        vec![NodeJs::singleton()]
    }

    fn optional(
        &self,
    ) -> Vec<(
        &'static str,
        &'static str,
        &'static dyn DependencyInstallable,
    )> {
        vec![("zshrc", "Will automatically load zshrc", Zshrc::singleton())]
    }
}

impl DependencyInstallable for Yarn {
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // check if yarn is installed by using `which yarn` OR if yarn is in path with which yarn
        let yarn_bin = metadata(&*YARN_BIN_PATH).is_ok();
        let yarn_which = DCommand::new("which", &["yarn"]).run();
        let yarn_which = matches!(yarn_which, Err(CommandError::CommandFailed(_)));
        *self.yarn_available.write().unwrap() = yarn_bin || yarn_which;

        // check if yarn is in EITHER of .zshrc or .bashrc
        let bashrc = metadata(&*BASHRC_PATH).is_ok();
        let zshrc = metadata(&*ZSHRC_PATH).is_ok();

        let mut zshrc_contents = String::new();

        todo!()
    }

    fn install(&self) -> Result<(), DependencyError> {
        todo!()
    }
}
