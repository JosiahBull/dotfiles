use std::sync::RwLock;

use singleton_derive::Singleton;

use super::{DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus};
use crate::{
    command::DCommand,
    dependencies::{
        apt_transport_https::AptTransportHttps, ca_certificates::CaCertificates, curl::Curl,
        gnupg::Gnupg,
    },
    OperatingSystem, CURRENT_USER, OPERATING_SYSTEM,
};

#[derive(Default, Debug, Singleton)]
pub struct Docker {
    // current_version: RwLock<Option<String>>,
    repo_available: RwLock<bool>,
    docker_installed: RwLock<bool>,
    docker_service_enabled: RwLock<bool>,
    docker_service_running: RwLock<bool>,
    user_in_docker_group: RwLock<bool>,
}

impl DependencyInfo for Docker {
    fn name(&self) -> &'static str {
        "Docker"
    }

    fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu2204
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu1804
            | OperatingSystem::PopOS2104 => vec![
                Curl::singleton(),
                Gnupg::singleton(),
                AptTransportHttps::singleton(),
                CaCertificates::singleton(),
            ],
            _ => unimplemented!("Docker is not supported on this platform"),
        }
    }
}

impl DependencyInstallable for Docker {
    /// Check if the dependency is installed on the current system.
    /// Updates internal state to reflect the current status.
    /// Validates:
    /// - Docker is installed
    /// - Docker systemctl service is enabled
    /// - Docker systemctl service is running
    /// - Current user is in the docker group
    fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
        // check if repo is available
        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu2204
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu1804 => {
                // check if docker repo is available
                let res = DCommand::new("apt-cache", &["policy", "docker-ce"]).run()?;
                *self.repo_available.write().unwrap() = res.stdout.contains("Candidate: 5:");
            }
            OperatingSystem::Fedora38 | OperatingSystem::Rocky8 | OperatingSystem::Rocky9 => {
                // check if docker repo is available
                let res = DCommand::new("dnf", &["list", "docker-ce"]).run()?;
                *self.repo_available.write().unwrap() = res.stdout.contains("docker-ce.x86_64");
            }
            _ => return Err(DependencyError::UnsupportedOperatingSystem),
        }

        if !*self.repo_available.read().unwrap() {
            return Ok(InstallationStatus::NotInstalled);
        }

        // Check if docker is installed
        DCommand::new("docker", &["--version"]).run()?;
        *self.docker_installed.write().unwrap() = true;

        // Check if docker systemctl service is enabled
        let res = DCommand::new("systemctl", &["is-enabled", "docker"]).run()?;
        *self.docker_service_enabled.write().unwrap() = res.stdout == "enabled";

        // Check if docker systemctl service is running
        let res = DCommand::new("systemctl", &["is-active", "docker"]).run()?;
        *self.docker_service_running.write().unwrap() = res.stdout == "active";

        // Check if current user is in the docker group
        let res = DCommand::new("groups", &[&*CURRENT_USER]).run()?;
        *self.user_in_docker_group.write().unwrap() = res.stdout.contains("docker");

        match (
            *self.repo_available.read().unwrap(),
            *self.docker_installed.read().unwrap(),
            *self.docker_service_enabled.read().unwrap(),
            *self.docker_service_running.read().unwrap(),
            *self.user_in_docker_group.read().unwrap(),
        ) {
            (false, false, false, false, false) => Ok(InstallationStatus::NotInstalled),
            (true, true, true, true, true) => Ok(InstallationStatus::FullyInstalled),
            _ => Ok(InstallationStatus::PartialInstall),
        }
    }

    /// Install the dependency.
    fn install(&self) -> Result<(), DependencyError> {
        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu2204
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu1804
            | OperatingSystem::PopOS2104 => {
                // if already fully installed, return
                if *self.repo_available.read().unwrap()
                    && *self.docker_installed.read().unwrap()
                    && *self.docker_service_enabled.read().unwrap()
                    && *self.docker_service_running.read().unwrap()
                    && *self.user_in_docker_group.read().unwrap()
                {
                    return Ok(());
                }

                // if repo is not available, add it
                if !*self.repo_available.read().unwrap() {
                    // add docker gpg key
                    // XXX: should check if key already exists first
                    // sudo install -m 0755 -d /etc/apt/keyrings
                    // curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
                    // sudo chmod a+r /etc/apt/keyrings/docker.gpg
                    DCommand::new("install", &["-m", "0755", "-d", "/etc/apt/keyrings"]).run()?;
                    // XXX: Piping is an anti-pattern, multi-stage these commands
                    DCommand::new(
                        "curl",
                        &[
                            "-fsSL",
                            "https://download.docker.com/linux/ubuntu/gpg",
                            "|",
                            "sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg",
                        ],
                    )
                    .run()?;
                    DCommand::new("chmod", &["a+r", "/etc/apt/keyrings/docker.gpg"]).run()?;

                    // add docker repo
                    // echo "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
                    DCommand::new("deb", &["[arch=\"$(dpkg --print-architecture)\" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \"$(. /etc/os-release && echo \"$VERSION_CODENAME\")\" stable\" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null"]).run()?;
                    DCommand::new("apt-get", &["update"]).run()?;

                    *self.repo_available.write().unwrap() = true;
                }

                // if docker is not installed, install it
                if !*self.docker_installed.read().unwrap() {
                    // sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
                    DCommand::new(
                        "apt-get",
                        &[
                            "install",
                            "-y",
                            "docker-ce",
                            "docker-ce-cli",
                            "containerd.io",
                            "docker-buildx-plugin",
                            "docker-compose-plugin",
                        ],
                    )
                    .run()?;
                    *self.docker_installed.write().unwrap() = true;
                }

                // if docker service is not enabled, enable it
                if !*self.docker_service_enabled.read().unwrap() {
                    // sudo systemctl enable docker.service
                    DCommand::new("systemctl", &["enable", "docker.service"]).run()?;
                    *self.docker_service_enabled.write().unwrap() = true;
                }

                // if docker service is not running, start it
                if !*self.docker_service_running.read().unwrap() {
                    // sudo systemctl start docker.service
                    DCommand::new("systemctl", &["start", "docker.service"]).run()?;
                    *self.docker_service_running.write().unwrap() = true;
                }

                // if user is not in docker group, add them
                if !*self.user_in_docker_group.read().unwrap() {
                    // sudo usermod -aG docker $USER
                    DCommand::new("usermod", &["-aG", "docker", &*CURRENT_USER]).run()?;
                    *self.user_in_docker_group.write().unwrap() = true;
                }

                Ok(())
            }
            _ => return Err(DependencyError::UnsupportedOperatingSystem),
        }
    }
}
