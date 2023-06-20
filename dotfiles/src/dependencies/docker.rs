use async_trait::async_trait;
use log::{warn, trace, debug};

use super::{Dependency, DependencyError, InstallationStatus};
use crate::{OPERATING_SYSTEM, OperatingSystem, CURRENT_USER};

#[derive(Debug)]
pub struct Docker {
    current_version: Option<String>,
    repo_available: bool,
    docker_installed: bool,
    docker_service_enabled: bool,
    docker_service_running: bool,
    user_in_docker_group: bool,
}

impl Docker {
    pub fn new() -> Self {
        Self {
            current_version: None,
            repo_available: false,
            docker_installed: false,
            docker_service_enabled: false,
            docker_service_running: false,
            user_in_docker_group: false,
        }
    }
}

#[async_trait]
impl Dependency for Docker {

    /// Validates:
    /// - Docker is installed
    /// - Docker systemctl service is enabled
    /// - Docker systemctl service is running
    /// - Current user is in the docker group
    async fn is_installed(&mut self) -> Result<InstallationStatus, DependencyError> {
        // check if repo is available
        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu2204 | OperatingSystem::Ubuntu2004 | OperatingSystem::Ubuntu1804 | OperatingSystem::PopOS2104 => {
                debug!("Checking if docker repo is available - ubuntu");
                // check if docker repo is available
                let output = tokio::process::Command::new("sh")
                    .arg("-c")
                    .arg("apt-cache policy docker-ce")
                    .output()
                    .await?;
                let stdout = String::from_utf8(output.stdout)?;

                if !output.status.success() || stdout.is_empty() || !stdout.contains("Candidate: 5:") {
                    trace!("Docker repo is not available");
                    self.repo_available = false;
                    return Ok(InstallationStatus::NotInstalled);
                } else {
                    trace!("Docker repo is available");
                    self.repo_available = true;
                }
            },
            OperatingSystem::Fedora38 | OperatingSystem::Rocky8 | OperatingSystem::Rocky9 => {
                debug!("Checking if docker repo is available - fedora38");
                // check if docker repo is available
                let output = tokio::process::Command::new("sh")
                    .arg("-c")
                    .arg("dnf list docker-ce")
                    .output()
                    .await?;
                let stdout = String::from_utf8(output.stdout)?;

                if !output.status.success() || stdout.is_empty() || !stdout.contains("docker-ce.x86_64") {
                    trace!("Docker repo is not available");
                    self.repo_available = false;
                    return Ok(InstallationStatus::NotInstalled);
                } else {
                    trace!("Docker repo is available");
                    self.repo_available = true;
                }
            },
            _ => return Err(DependencyError::UnsupportedOperatingSystem)
        }

        // Check if docker is installed
        debug!("Checking if docker is installed");
        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg("docker --version")
            .output()
            .await?;
        let stdout = String::from_utf8(output.stdout).unwrap();

        if !output.status.success() || stdout.is_empty() || !stdout.starts_with("Docker version") {
            trace!("Docker is not installed");
            self.docker_installed = false;
            return Ok(InstallationStatus::NotInstalled);
        } else {
            trace!("Docker is installed");
            self.docker_installed = true;
            self.current_version = Some(stdout.split(' ').collect::<Vec<&str>>()[2].trim().replace(',', ""));
        }

        // Check if docker systemctl service is enabled
        debug!("Checking if docker systemctl service is enabled");
        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg("systemctl is-enabled docker")
            .output()
            .await?;

        if !output.status.success() || String::from_utf8(output.stdout).unwrap().trim() != "enabled" {
            trace!("Docker systemctl service is not enabled");
            self.docker_service_enabled = false;
        } else {
            trace!("Docker systemctl service is enabled");
            self.docker_service_enabled = true;
        }

        // Check if docker systemctl service is running
        debug!("Checking if docker systemctl service is running");
        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg("systemctl is-active docker")
            .output()
            .await?;

        if !output.status.success() || String::from_utf8(output.stdout).unwrap().trim() != "active" {
            trace!("Docker systemctl service is not running");
            self.docker_service_running = false;
        } else {
            trace!("Docker systemctl service is running");
            self.docker_service_running = true;
        }

        // Check if current user is in the docker group
        debug!("Checking if current user is in the docker group");
        // sh -c "sudo su -c 'sh -c groups' $USER"
        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(format!("sudo su -c 'sh -c groups' {}", *CURRENT_USER))
            .output()
            .await?;

        if !output.status.success() || !String::from_utf8(output.stdout).unwrap().trim().split(' ').any(|x| x == "docker") {
            trace!("Current user is not in the docker group");
            self.user_in_docker_group = false;
        } else {
            trace!("Current user is in the docker group");
            self.user_in_docker_group = true;
        }

        // check if any of the above are false
        if !self.repo_available || !self.docker_installed || !self.docker_service_enabled || !self.docker_service_running || !self.user_in_docker_group {
            debug!("Docker is partially installed");
            return Ok(InstallationStatus::PartialInstall);
        }

        debug!("Docker is fully installed");
        Ok(InstallationStatus::FullyInstalled)
    }

    async fn install(&mut self, version: Option<&str>) -> Result<(), DependencyError> {
        debug!("Installing docker using OS {:?}", *OPERATING_SYSTEM);

        if version.is_some() {
            // emit warning that custom version is not supported
            warn!("Custom version of docker is not supported. Installing latest version.")
        }

        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu2204 | OperatingSystem::Ubuntu2004 | OperatingSystem::Ubuntu1804 | OperatingSystem::PopOS2104 => {
                debug!("Installing docker - ubuntu");
                // if already fully installed, return
                if self.repo_available && self.docker_installed && self.docker_service_enabled && self.docker_service_running && self.user_in_docker_group {
                    return Ok(())
                }

                // if repo is not available, add it
                if !self.repo_available {
                    debug!("Adding docker repo");

                    // check ca-certificates, curl, gnupg are installed
                    trace!("Checking if ca-certificates, curl, gnupg are installed");
                    let output = tokio::process::Command::new("sh")
                        .arg("-c")
                        .arg("apt-get install -y apt-transport-https ca-certificates curl gnupg")
                        .output()
                        .await?;

                    if !output.status.success() {
                        // XXX: add logging of output
                        return Err(DependencyError::DependencyFailed("Missing or unable to install ca-certificates, curl, gnupg".to_string()))
                    }

                    // add docker gpg key
                    // sudo install -m 0755 -d /etc/apt/keyrings
                    // curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
                    // sudo chmod a+r /etc/apt/keyrings/docker.gpg
                    trace!("Adding docker gpg key");
                    let output = tokio::process::Command::new("sh")
                        .arg("-c")
                        .arg("install -m 0755 -d /etc/apt/keyrings")
                        .output()
                        .await?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed("Unable to create /etc/apt/keyrings directory".to_string()))
                    }

                    trace!("Adding docker gpg key");
                    let output = tokio::process::Command::new("sh")
                        .arg("-c")
                        .arg("curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg")
                        .output()
                        .await?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed("Unable to add docker gpg key".to_string()))
                    }

                    trace!("Adding docker gpg key");
                    let output = tokio::process::Command::new("sh")
                        .arg("-c")
                        .arg("chmod a+r /etc/apt/keyrings/docker.gpg")
                        .output()
                        .await?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed("Unable to chmod docker gpg key".to_string()))
                    }

                    // add docker repo
                    // echo \
                    // "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
                    // "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
                    trace!("Adding docker repo");
                    let output = tokio::process::Command::new("sh")
                        .arg("-c")
                        .arg("echo \"deb [arch=\"$(dpkg --print-architecture)\" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \"$(. /etc/os-release && echo \"$VERSION_CODENAME\")\" stable\" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null")
                        .output()
                        .await?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed("Unable to add docker repo".to_string()))
                    }

                    // update apt
                    // sudo apt-get update
                    trace!("Updating apt");
                    let output = tokio::process::Command::new("sh")
                        .arg("-c")
                        .arg("apt-get update")
                        .output()
                        .await?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed("Unable to update apt".to_string()))
                    }

                    self.repo_available = true;
                }

                // if docker is not installed, install it
                if !self.docker_installed {
                    debug!("Installing docker");

                    // sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
                    trace!("Installing docker");
                    let output = tokio::process::Command::new("sh")
                        .arg("-c")
                        .arg("apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin")
                        .output()
                        .await?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed("Unable to install docker".to_string()))
                    }

                    self.docker_installed = true;
                }

                // if docker service is not enabled, enable it
                if !self.docker_service_enabled {
                    debug!("Enabling docker service");

                    // sudo systemctl enable docker.service
                    trace!("Enabling docker service");
                    let output = tokio::process::Command::new("sh")
                        .arg("-c")
                        .arg("systemctl enable docker.service")
                        .output()
                        .await?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed("Unable to enable docker service".to_string()))
                    }

                    self.docker_service_enabled = true;
                }

                // if docker service is not running, start it
                if !self.docker_service_running {
                    debug!("Starting docker service");

                    // sudo systemctl start docker.service
                    trace!("Starting docker service");
                    let output = tokio::process::Command::new("sh")
                        .arg("-c")
                        .arg("systemctl start docker.service")
                        .output()
                        .await?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed("Unable to start docker service".to_string()))
                    }

                    self.docker_service_running = true;
                }

                // if user is not in docker group, add them
                if !self.user_in_docker_group {
                    debug!("Adding user to docker group");

                    // sudo usermod -aG docker $USER
                    trace!("Adding user to docker group");
                    let output = tokio::process::Command::new("sh")
                        .arg("-c")
                        .arg(format!("usermod -aG docker {}", *CURRENT_USER))
                        .output()
                        .await?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed("Unable to add user to docker group".to_string()))
                    }

                    self.user_in_docker_group = true;
                }

                Ok(())
            },
            OperatingSystem::Fedora38 => {
                todo!()
            },
            OperatingSystem::Rocky8 => {
                todo!()
            },
            OperatingSystem::Rocky9 => {
                todo!()
            },
        }
    }

    async fn uninstall(&mut self) -> Result<(), DependencyError> {
        unimplemented!("Docker uninstall not implemented");
    }
}