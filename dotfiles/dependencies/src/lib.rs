pub mod command;
pub mod dependencies;
pub mod system_data;
pub mod tree;

// Make all dependencies top-level for convenience
pub use dependencies::*;

// Other Imports
use lazy_static::lazy_static;
use sysinfo::SystemExt;

use crate::command::DCommand;

// TODO: refactor into various supporting files //

lazy_static! {
    pub static ref OPERATING_SYSTEM: OperatingSystem =
        OperatingSystem::from_sysinfo().expect("Unable to determine operating system");
    pub static ref CURRENT_USER: String = {
        // get the current user through sudo by checking SUDO_USER env var, if it doesn't exist
        // then use whoami to acquire the current user and roll with that. :)
        std::env::var("SUDO_USER").unwrap_or_else(|_|whoami::username())
    };
    pub static ref HOME_DIR: String = {
        // Get home dir of *CURRENT_USER*
        let res = DCommand::new("getent", &["passwd", &*CURRENT_USER])
            .run()
            .expect("able to run shell command");
        assert!(res.success, "Unable to get home dir due to an error. Stdout {} stderr {}", res.stdout, res.stderr);
        let home_dir: String = res.stdout.split(':').nth(5).unwrap().to_string();
        home_dir
    };
}

#[derive(Debug)]
enum DotfilesError {
    UnknownOperatingSystem(String),
    UnsupportedOperatingSystem,
}

#[derive(Debug)]
pub enum OperatingSystem {
    Ubuntu2204,
    Ubuntu2004,
    Ubuntu1804,

    Fedora38,

    Rocky9,
    Rocky8,

    PopOS2104,
}

impl OperatingSystem {
    fn from_sysinfo() -> Result<Self, DotfilesError> {
        let system = sysinfo::System::new_all();

        // print out the current system information
        println!("System name:             {:?}", system.name());
        println!("System kernel version:   {:?}", system.kernel_version());
        println!("System OS version:       {:?}", system.os_version());
        println!("System host name:        {:?}", system.host_name());
        println!("System uptime:           {}", system.uptime());
        println!("System number of users:  {}", system.users().len());
        println!("System processes:        {}", system.processes().len());
        println!("System total memory:     {} kB", system.total_memory());
        println!("System free memory:      {} kB", system.free_memory());

        if let Some(os) = system.long_os_version() {
            match os.as_str() {
                "Linux 22.04 Ubuntu" => Ok(OperatingSystem::Ubuntu2204),
                "Linux 20.04 Ubuntu" => Ok(OperatingSystem::Ubuntu2004),
                "Linux 18.04 Ubuntu" => Ok(OperatingSystem::Ubuntu1804),

                "Linux 38 Fedora" => Ok(OperatingSystem::Fedora38),

                "Linux 9 Rocky" => Ok(OperatingSystem::Rocky9),
                "Linux 8 Rocky" => Ok(OperatingSystem::Rocky8),

                "Linux 21.04 Pop!_OS" => Ok(OperatingSystem::PopOS2104),
                _ => Err(DotfilesError::UnknownOperatingSystem(os)),
            }
        } else {
            Err(DotfilesError::UnknownOperatingSystem(
                "Unable to determine operating system".to_string(),
            ))
        }
    }
}
