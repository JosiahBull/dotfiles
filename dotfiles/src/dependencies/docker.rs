use std::{
    process::Command,
    rc::{Rc, Weak},
    sync::RwLock,
};

use log::{debug, trace, warn};

use super::{
    DependencyError, DependencyInfo, DependencyInstallable, Installable,
    InstallationStatus, Dependency,
};
use crate::{OperatingSystem, CURRENT_USER, OPERATING_SYSTEM};

#[derive(Debug)]
pub struct Docker {
    current_version: Option<String>,
    repo_available: bool,
    docker_installed: bool,
    docker_service_enabled: bool,
    docker_service_running: bool,
    user_in_docker_group: bool,

    // Dependency graph
    self_ref: RwLock<Option<Rc<Docker>>>,
    parents: RwLock<Vec<Weak<dyn Dependency>>>,
    children: RwLock<Vec<Rc<dyn Dependency>>>,
    is_enabled: RwLock<bool>,
}

impl Docker {
    pub fn new() -> Rc<Self> {
        let mut res = Rc::new(Self {
            current_version: None,
            repo_available: false,
            docker_installed: false,
            docker_service_enabled: false,
            docker_service_running: false,
            user_in_docker_group: false,

            self_ref: RwLock::new(None),
            parents: RwLock::new(Vec::new()),
            children: RwLock::new(Vec::new()),
            is_enabled: RwLock::new(false),
        });

        // set self reference
        // let ptr = Arc::into_raw(res.clone());
        *res.self_ref.write().unwrap() = Some(res.clone());

        res
    }
}

impl DependencyInfo for Docker {
    fn name(&self) -> &'static str {
        "Docker"
    }
}

impl DependencyInstallable for Docker {
    /// Check if this package is installable on the current system.
    fn installable(&self) -> Result<Installable, DependencyError> {
        todo!()
    }

    /// Check if the dependency is installed on the current system.
    /// Updates internal state to reflect the current status.
    /// Validates:
    /// - Docker is installed
    /// - Docker systemctl service is enabled
    /// - Docker systemctl service is running
    /// - Current user is in the docker group
    fn is_installed(&mut self) -> Result<InstallationStatus, DependencyError> {
        // check if repo is available
        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu2204
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu1804
            | OperatingSystem::PopOS2104 => {
                debug!("Checking if docker repo is available - ubuntu");
                // check if docker repo is available
                let output = Command::new("sh")
                    .arg("-c")
                    .arg("apt-cache policy docker-ce")
                    .output()?;
                let stdout = String::from_utf8(output.stdout)?;

                if !output.status.success()
                    || stdout.is_empty()
                    || !stdout.contains("Candidate: 5:")
                {
                    trace!("Docker repo is not available");
                    self.repo_available = false;
                    return Ok(InstallationStatus::NotInstalled);
                } else {
                    trace!("Docker repo is available");
                    self.repo_available = true;
                }
            }
            OperatingSystem::Fedora38 | OperatingSystem::Rocky8 | OperatingSystem::Rocky9 => {
                debug!("Checking if docker repo is available - fedora38");
                // check if docker repo is available
                let output = Command::new("sh")
                    .arg("-c")
                    .arg("dnf list docker-ce")
                    .output()?;
                let stdout = String::from_utf8(output.stdout)?;

                if !output.status.success()
                    || stdout.is_empty()
                    || !stdout.contains("docker-ce.x86_64")
                {
                    trace!("Docker repo is not available");
                    self.repo_available = false;
                    return Ok(InstallationStatus::NotInstalled);
                } else {
                    trace!("Docker repo is available");
                    self.repo_available = true;
                }
            }
            _ => return Err(DependencyError::UnsupportedOperatingSystem),
        }

        // Check if docker is installed
        debug!("Checking if docker is installed");
        let output = Command::new("sh")
            .arg("-c")
            .arg("docker --version")
            .output()?;
        let stdout = String::from_utf8(output.stdout).unwrap();

        if !output.status.success() || stdout.is_empty() || !stdout.starts_with("Docker version") {
            trace!("Docker is not installed");
            self.docker_installed = false;
            return Ok(InstallationStatus::NotInstalled);
        } else {
            trace!("Docker is installed");
            self.docker_installed = true;
            self.current_version = Some(
                stdout.split(' ').collect::<Vec<&str>>()[2]
                    .trim()
                    .replace(',', ""),
            );
        }

        // Check if docker systemctl service is enabled
        debug!("Checking if docker systemctl service is enabled");
        let output = Command::new("sh")
            .arg("-c")
            .arg("systemctl is-enabled docker")
            .output()?;

        if !output.status.success() || String::from_utf8(output.stdout).unwrap().trim() != "enabled"
        {
            trace!("Docker systemctl service is not enabled");
            self.docker_service_enabled = false;
        } else {
            trace!("Docker systemctl service is enabled");
            self.docker_service_enabled = true;
        }

        // Check if docker systemctl service is running
        debug!("Checking if docker systemctl service is running");
        let output = Command::new("sh")
            .arg("-c")
            .arg("systemctl is-active docker")
            .output()?;

        if !output.status.success() || String::from_utf8(output.stdout).unwrap().trim() != "active"
        {
            trace!("Docker systemctl service is not running");
            self.docker_service_running = false;
        } else {
            trace!("Docker systemctl service is running");
            self.docker_service_running = true;
        }

        // Check if current user is in the docker group
        debug!("Checking if current user is in the docker group");
        // sh -c "sudo su -c 'sh -c groups' $USER"
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("sudo su -c 'sh -c groups' {}", *CURRENT_USER))
            .output()?;

        if !output.status.success()
            || !String::from_utf8(output.stdout)
                .unwrap()
                .trim()
                .split(' ')
                .any(|x| x == "docker")
        {
            trace!("Current user is not in the docker group");
            self.user_in_docker_group = false;
        } else {
            trace!("Current user is in the docker group");
            self.user_in_docker_group = true;
        }

        // check if any of the above are false
        if !self.repo_available
            || !self.docker_installed
            || !self.docker_service_enabled
            || !self.docker_service_running
            || !self.user_in_docker_group
        {
            debug!("Docker is partially installed");
            return Ok(InstallationStatus::PartialInstall);
        }

        debug!("Docker is fully installed");
        Ok(InstallationStatus::FullyInstalled)
    }

    /// Install the dependency.
    fn install(&mut self, version: Option<&str>) -> Result<(), DependencyError> {
        debug!("Installing docker using OS {:?}", *OPERATING_SYSTEM);

        if version.is_some() {
            // emit warning that custom version is not supported
            warn!("Custom version of docker is not supported. Installing latest version.")
        }

        match *OPERATING_SYSTEM {
            OperatingSystem::Ubuntu2204
            | OperatingSystem::Ubuntu2004
            | OperatingSystem::Ubuntu1804
            | OperatingSystem::PopOS2104 => {
                debug!("Installing docker - ubuntu");
                // if already fully installed, return
                if self.repo_available
                    && self.docker_installed
                    && self.docker_service_enabled
                    && self.docker_service_running
                    && self.user_in_docker_group
                {
                    return Ok(());
                }

                // if repo is not available, add it
                if !self.repo_available {
                    debug!("Adding docker repo");

                    // check ca-certificates, curl, gnupg are installed
                    trace!("Checking if ca-certificates, curl, gnupg are installed");
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg("apt-get install -y apt-transport-https ca-certificates curl gnupg")
                        .output()?;

                    if !output.status.success() {
                        // XXX: add logging of output
                        return Err(DependencyError::DependencyFailed(
                            "Missing or unable to install ca-certificates, curl, gnupg".to_string(),
                        ));
                    }

                    // add docker gpg key
                    // sudo install -m 0755 -d /etc/apt/keyrings
                    // curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
                    // sudo chmod a+r /etc/apt/keyrings/docker.gpg
                    trace!("Adding docker gpg key");
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg("install -m 0755 -d /etc/apt/keyrings")
                        .output()?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed(
                            "Unable to create /etc/apt/keyrings directory".to_string(),
                        ));
                    }

                    trace!("Adding docker gpg key");
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg("curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg")
                        .output()
                        ?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed(
                            "Unable to add docker gpg key".to_string(),
                        ));
                    }

                    trace!("Adding docker gpg key");
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg("chmod a+r /etc/apt/keyrings/docker.gpg")
                        .output()?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed(
                            "Unable to chmod docker gpg key".to_string(),
                        ));
                    }

                    // add docker repo
                    // echo \
                    // "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
                    // "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
                    trace!("Adding docker repo");
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg("echo \"deb [arch=\"$(dpkg --print-architecture)\" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \"$(. /etc/os-release && echo \"$VERSION_CODENAME\")\" stable\" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null")
                        .output()
                        ?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed(
                            "Unable to add docker repo".to_string(),
                        ));
                    }

                    // update apt
                    // sudo apt-get update
                    trace!("Updating apt");
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg("apt-get update")
                        .output()?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed(
                            "Unable to update apt".to_string(),
                        ));
                    }

                    self.repo_available = true;
                }

                // if docker is not installed, install it
                if !self.docker_installed {
                    debug!("Installing docker");

                    // sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
                    trace!("Installing docker");
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg("apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin")
                        .output()
                        ?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed(
                            "Unable to install docker".to_string(),
                        ));
                    }

                    self.docker_installed = true;
                }

                // if docker service is not enabled, enable it
                if !self.docker_service_enabled {
                    debug!("Enabling docker service");

                    // sudo systemctl enable docker.service
                    trace!("Enabling docker service");
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg("systemctl enable docker.service")
                        .output()?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed(
                            "Unable to enable docker service".to_string(),
                        ));
                    }

                    self.docker_service_enabled = true;
                }

                // if docker service is not running, start it
                if !self.docker_service_running {
                    debug!("Starting docker service");

                    // sudo systemctl start docker.service
                    trace!("Starting docker service");
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg("systemctl start docker.service")
                        .output()?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed(
                            "Unable to start docker service".to_string(),
                        ));
                    }

                    self.docker_service_running = true;
                }

                // if user is not in docker group, add them
                if !self.user_in_docker_group {
                    debug!("Adding user to docker group");

                    // sudo usermod -aG docker $USER
                    trace!("Adding user to docker group");
                    let output = Command::new("sh")
                        .arg("-c")
                        .arg(format!("usermod -aG docker {}", *CURRENT_USER))
                        .output()?;

                    if !output.status.success() {
                        return Err(DependencyError::DependencyFailed(
                            "Unable to add user to docker group".to_string(),
                        ));
                    }

                    self.user_in_docker_group = true;
                }

                Ok(())
            }
            OperatingSystem::Fedora38 => {
                todo!()
            }
            OperatingSystem::Rocky8 => {
                todo!()
            }
            OperatingSystem::Rocky9 => {
                todo!()
            }
        }
    }

    /// Uninstall the dependency.
    fn uninstall(&mut self) -> Result<(), DependencyError> {
        unimplemented!("Docker uninstall not implemented");
    }
}

impl Dependency for Docker {
    /// Get a list of all dependencies that this application requires
    // fn dependencies<'b>(&'b self) -> &'b[&'b dyn DependencyGraph] {
    //     self.children.read().unwrap().as_slice()
    // }
    fn dependencies(&self) -> Vec<Rc<dyn Dependency>> {
        self.children.read().unwrap().clone()
    }

    // /// Get a list of dependants that require this application
    // fn dependants<'b>(&'b self) -> &'b[&'b dyn DependencyGraph<'a>] {
    //     self.parents.read().unwrap().as_slice()
    // }
    fn dependants(&self) -> Vec<Weak<dyn Dependency>> {
        self.parents.read().unwrap().clone()
    }

    fn add_dependency(&self, dependency: Rc<dyn Dependency>) {
        let self_ref = self.self_ref.read().unwrap().clone().unwrap();
        let self_ref = Rc::downgrade(&self_ref);
        dependency.add_dependant(self_ref);
        self.children.write().unwrap().push(dependency);
    }

    fn add_dependant(&self, dependant: Weak<dyn Dependency>) {
        self.parents.write().unwrap().push(dependant);
    }

    /// Enable or disable this dependency
    fn set_enabled(&self, enabled: bool) {
        *self.is_enabled.write().unwrap() = enabled;
    }

    /// Check if this dependency is enabled
    fn is_enabled(&self) -> bool {
        // check if this dependency is enabled
        let enabled = *self.is_enabled.read().unwrap();
        if !enabled {
            return false;
        }

        // check if any children are disabled
        for child in self.children.read().unwrap().iter() {
            if !child.is_enabled() {
                return false;
            }
        }

        true
    }
}
