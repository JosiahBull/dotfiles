// Application Requirements
// - Check if all required dependencies are installed
// - If an NVIDIA GPU is present, check if the NVIDIA drivers are installed
//      - If missing, ask user if they want to install them
// - Install any that are missing using the relevant package manager
// - Run dotfiles checks
//      - Check if .zshrc exists, and is the expected file
//      - Check if ssh is configured correctly
//      - Check if git is configured correctly
//      - Check if tmux is configured correctly
//      - Check if vim is configured correctly
//      - Check if Docker is configured correctly

mod app;
mod dependencies;
mod event;
mod handler;
mod tui;
mod ui;

use std::io;

use app::{App, AppResult};
use event::{Event, EventHandler};
use handler::handle_key_events;
use lazy_static::lazy_static;
use ratatui::{backend::CrosstermBackend, Terminal};
use sudo::RunningAs;
use sysinfo::SystemExt;

use log::{debug, error, info, trace, warn, LevelFilter};
use tui::Tui;

lazy_static! {
    static ref OPERATING_SYSTEM: OperatingSystem =
        OperatingSystem::from_sysinfo().expect("Unable to determine operating system");
    static ref CURRENT_USER: String = whoami::username();
    static ref HOME_DIR: String = home::home_dir()
        .expect("Unable to find home directory")
        .to_str()
        .expect("Unable to convert home directory to String")
        .to_string();
}

#[derive(Debug)]
enum DotfilesError {
    UnknownOperatingSystem(String),
    UnsupportedOperatingSystem,
}

#[derive(Debug)]
enum OperatingSystem {
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

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(500);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

// fn main() {
//     // TODO: setup clap to parse arguments,
//     // TODO: setup reading from a version file that gets written
//     // TODO: setup proper logging with a very aggressive log level

//     //XXX: move configuration load into a separate function
//     //XXX: setup cargo deny

//     // pretty_env_logger::init();
//     pretty_env_logger::formatted_builder()
//         .filter_level(LevelFilter::Trace)
//         .init();

//     error!("This is an error!");
//     warn!("This is a warn!");
//     info!("This is an info!");
//     debug!("This is a debug!");
//     trace!("This is a trace!");

//     let is_sudo = sudo::escalate_if_needed().unwrap();
//     if is_sudo != RunningAs::Root {
//         println!("This application requires root privileges to install dependencies");
//         std::process::exit(1);
//     }

//     let user = whoami::username();
//     println!("Running as user: {}", user);

//     // let mut d = Docker::new();
//     // let is_installed = d.is_installed().await.unwrap();

//     // println!("is_installed: {:?}", is_installed);
//     // println!("things: {:?}", d);

//     // if ! matches!(is_installed, InstallationStatus::FullyInstalled) {
//     //     println!("Installing docker");
//     //     d.install(None).await.unwrap();
//     // }

//     // println!("is_installed: {:?}", d.is_installed().await.unwrap());
//     // println!("things: {:?}", d);
// }
