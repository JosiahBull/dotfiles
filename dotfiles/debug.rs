#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod app {
    use std::{
        cell::RefCell, collections::{HashMap, HashSet},
        error, rc::{Rc, Weak},
    };
    use crate::dependencies::DependencyInfo;
    /// Application result type.
    pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
    /// Application.
    pub struct App {
        /// Is the application running?
        pub running: bool,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for App {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "App",
                "running",
                &&self.running,
            )
        }
    }
    impl Default for App {
        fn default() -> Self {
            Self { running: true }
        }
    }
    impl App {
        /// Constructs a new instance of [`App`].
        pub fn new() -> Self {
            Self::default()
        }
        /// Handles the tick event of the terminal.
        pub fn tick(&self) {}
        /// Set running to false to quit the application.
        pub fn quit(&mut self) {
            self.running = false;
        }
    }
}
mod dependencies {
    use std::{string::FromUtf8Error, fs::metadata};
    pub mod apt_transport_https {
        use std::process::Command;
        use singleton_derive::Singleton;
        use super::{
            DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
        };
        use crate::{OperatingSystem, OPERATING_SYSTEM};
        pub struct AptTransportHttps {
            apt_transport_https_available: bool,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AptTransportHttps {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AptTransportHttps",
                    "apt_transport_https_available",
                    &&self.apt_transport_https_available,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AptTransportHttps {
            #[inline]
            fn default() -> AptTransportHttps {
                AptTransportHttps {
                    apt_transport_https_available: ::core::default::Default::default(),
                }
            }
        }
        static APT_TRANSPORT_HTTPS: std::sync::OnceLock<
            &'static mut AptTransportHttps,
        > = std::sync::OnceLock::new();
        impl AptTransportHttps {
            pub fn singleton() -> &'static mut Self {
                use std::sync::OnceLock;
                APT_TRANSPORT_HTTPS
                    .get_or_init(|| {
                        ::std::boxed::Box::leak(::std::boxed::Box::new(Self::default()))
                    })
            }
        }
        impl DependencyInfo for AptTransportHttps {
            fn name(&self) -> &'static str {
                "apt-transport-https"
            }
        }
        impl DependencyInstallable for AptTransportHttps {
            fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
                let output = Command::new("which")
                    .arg("apt-transport-https")
                    .spawn()?
                    .wait_with_output()?;
                if output.status.success() {
                    return Ok(InstallationStatus::FullyInstalled);
                }
                self.apt_transport_https_available = false;
                Ok(InstallationStatus::NotInstalled)
            }
            fn install(&self, _: Option<&str>) -> Result<(), DependencyError> {
                if self.apt_transport_https_available {
                    return Ok(());
                }
                match *OPERATING_SYSTEM {
                    OperatingSystem::Ubuntu1804
                    | OperatingSystem::Ubuntu2004
                    | OperatingSystem::Ubuntu2204 => {
                        let output = Command::new("sudo")
                            .arg("apt-get")
                            .arg("update")
                            .spawn()?
                            .wait_with_output()?;
                        if !output.status.success() {
                            return Err(
                                DependencyError::DependencyFailed(
                                    String::from("apt-get update"),
                                ),
                            );
                        }
                        let output = Command::new("sudo")
                            .arg("apt-get")
                            .arg("install")
                            .arg("apt-transport-https")
                            .spawn()?
                            .wait_with_output()?;
                        if !output.status.success() {
                            return Err(
                                DependencyError::DependencyFailed(
                                    String::from("apt-get install apt-transport-https"),
                                ),
                            );
                        }
                        self.apt_transport_https_available = true;
                    }
                    _ => return Err(DependencyError::UnsupportedOperatingSystem),
                }
                Ok(())
            }
        }
    }
    pub mod ca_certificates {
        use std::process::Command;
        use singleton_derive::Singleton;
        use super::{
            DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
        };
        use crate::{OperatingSystem, OPERATING_SYSTEM};
        pub struct CaCertificates {
            ca_certificates_available: bool,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CaCertificates {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "CaCertificates",
                    "ca_certificates_available",
                    &&self.ca_certificates_available,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for CaCertificates {
            #[inline]
            fn default() -> CaCertificates {
                CaCertificates {
                    ca_certificates_available: ::core::default::Default::default(),
                }
            }
        }
        static CA_CERTIFICATES: std::sync::OnceLock<&'static mut CaCertificates> = std::sync::OnceLock::new();
        impl CaCertificates {
            pub fn singleton() -> &'static mut Self {
                use std::sync::OnceLock;
                CA_CERTIFICATES
                    .get_or_init(|| {
                        ::std::boxed::Box::leak(::std::boxed::Box::new(Self::default()))
                    })
            }
        }
        impl DependencyInfo for CaCertificates {
            fn name(&self) -> &'static str {
                "ca-certificates"
            }
        }
        impl DependencyInstallable for CaCertificates {
            fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
                let output = Command::new("which")
                    .arg("update-ca-certificates")
                    .spawn()?
                    .wait_with_output()?;
                if output.status.success() {
                    return Ok(InstallationStatus::FullyInstalled);
                }
                self.ca_certificates_available = false;
                Ok(InstallationStatus::NotInstalled)
            }
            fn install(&self, _: Option<&str>) -> Result<(), DependencyError> {
                if self.ca_certificates_available {
                    return Ok(());
                }
                match *OPERATING_SYSTEM {
                    OperatingSystem::Ubuntu1804
                    | OperatingSystem::Ubuntu2004
                    | OperatingSystem::Ubuntu2204 => {
                        let output = Command::new("sudo")
                            .arg("apt")
                            .arg("update")
                            .spawn()?
                            .wait_with_output()?;
                        if !output.status.success() {
                            return Err(
                                DependencyError::DependencyFailed(
                                    String::from("apt update"),
                                ),
                            );
                        }
                        let output = Command::new("sudo")
                            .arg("apt")
                            .arg("install")
                            .arg("-y")
                            .arg("ca-certificates")
                            .spawn()?
                            .wait_with_output()?;
                        if !output.status.success() {
                            return Err(
                                DependencyError::DependencyFailed(
                                    String::from("apt install ca-certificates"),
                                ),
                            );
                        }
                        let output = Command::new("sudo")
                            .arg("update-ca-certificates")
                            .spawn()?
                            .wait_with_output()?;
                        if !output.status.success() {
                            return Err(
                                DependencyError::DependencyFailed(
                                    String::from("update-ca-certificates"),
                                ),
                            );
                        }
                    }
                    _ => return Err(DependencyError::UnsupportedOperatingSystem),
                }
                Ok(())
            }
        }
    }
    pub mod curl {
        use singleton_derive::Singleton;
        use crate::{OperatingSystem, OPERATING_SYSTEM};
        use super::{
            DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
        };
        pub struct Curl {
            curl_available: bool,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Curl {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Curl",
                    "curl_available",
                    &&self.curl_available,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Curl {
            #[inline]
            fn default() -> Curl {
                Curl {
                    curl_available: ::core::default::Default::default(),
                }
            }
        }
        static CURL: std::sync::OnceLock<&'static mut Curl> = std::sync::OnceLock::new();
        impl Curl {
            pub fn singleton() -> &'static mut Self {
                use std::sync::OnceLock;
                CURL.get_or_init(|| {
                    ::std::boxed::Box::leak(::std::boxed::Box::new(Self::default()))
                })
            }
        }
        impl DependencyInfo for Curl {
            fn name(&self) -> &'static str {
                "curl"
            }
        }
        impl DependencyInstallable for Curl {
            fn is_installed(
                &mut self,
            ) -> Result<super::InstallationStatus, super::DependencyError> {
                let output = std::process::Command::new("which")
                    .arg("curl")
                    .spawn()?
                    .wait_with_output()?;
                if output.status.success() {
                    return Ok(super::InstallationStatus::FullyInstalled);
                }
                self.curl_available = false;
                Ok(InstallationStatus::NotInstalled)
            }
            fn install(
                &mut self,
                _: Option<&str>,
            ) -> Result<(), super::DependencyError> {
                if self.curl_available {
                    return Ok(());
                }
                match *OPERATING_SYSTEM {
                    OperatingSystem::Ubuntu1804
                    | OperatingSystem::Ubuntu2004
                    | OperatingSystem::Ubuntu2204 => {
                        let output = std::process::Command::new("sudo")
                            .arg("apt-get")
                            .arg("update")
                            .spawn()?
                            .wait_with_output()?;
                        if !output.status.success() {
                            return Err(
                                DependencyError::DependencyFailed(
                                    String::from("apt-get update"),
                                ),
                            );
                        }
                        let output = std::process::Command::new("sudo")
                            .arg("apt-get")
                            .arg("install")
                            .arg("-y")
                            .arg("curl")
                            .spawn()?
                            .wait_with_output()?;
                        if !output.status.success() {
                            return Err(
                                DependencyError::DependencyFailed(
                                    String::from("apt-get install curl"),
                                ),
                            );
                        }
                    }
                    _ => return Err(DependencyError::UnsupportedOperatingSystem),
                }
                Ok(())
            }
        }
    }
    pub mod gnupg {
        use std::process::Command;
        use singleton_derive::Singleton;
        use super::{
            DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
        };
        use crate::{OperatingSystem, OPERATING_SYSTEM};
        pub struct Gnupg {
            gpg_available: bool,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Gnupg {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Gnupg",
                    "gpg_available",
                    &&self.gpg_available,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Gnupg {
            #[inline]
            fn default() -> Gnupg {
                Gnupg {
                    gpg_available: ::core::default::Default::default(),
                }
            }
        }
        static GNUPG: std::sync::OnceLock<&'static mut Gnupg> = std::sync::OnceLock::new();
        impl Gnupg {
            pub fn singleton() -> &'static mut Self {
                use std::sync::OnceLock;
                GNUPG
                    .get_or_init(|| {
                        ::std::boxed::Box::leak(::std::boxed::Box::new(Self::default()))
                    })
            }
        }
        impl DependencyInfo for Gnupg {
            fn name(&self) -> &'static str {
                "GnuPG"
            }
        }
        impl DependencyInstallable for Gnupg {
            fn is_installed(
                &mut self,
            ) -> Result<super::InstallationStatus, super::DependencyError> {
                let output = Command::new("which")
                    .arg("gpg")
                    .spawn()?
                    .wait_with_output()?;
                if output.status.success() {
                    return Ok(super::InstallationStatus::FullyInstalled);
                }
                self.gpg_available = false;
                Ok(InstallationStatus::NotInstalled)
            }
            fn install(
                &mut self,
                _: Option<&str>,
            ) -> Result<(), super::DependencyError> {
                if self.gpg_available {
                    return Ok(());
                }
                match *OPERATING_SYSTEM {
                    OperatingSystem::Ubuntu1804
                    | OperatingSystem::Ubuntu2004
                    | OperatingSystem::Ubuntu2204 => {
                        let output = Command::new("sudo")
                            .arg("apt-get")
                            .arg("update")
                            .spawn()?
                            .wait_with_output()?;
                        if !output.status.success() {
                            return Err(
                                DependencyError::DependencyFailed(
                                    String::from("apt-get update"),
                                ),
                            );
                        }
                        let output = Command::new("sudo")
                            .arg("apt-get")
                            .arg("install")
                            .arg("-y")
                            .arg("gnupg")
                            .spawn()?
                            .wait_with_output()?;
                        if !output.status.success() {
                            return Err(
                                DependencyError::DependencyFailed(
                                    String::from("apt-get install gnupg"),
                                ),
                            );
                        }
                        self.gpg_available = true;
                    }
                    _ => return Err(DependencyError::UnsupportedOperatingSystem),
                }
                Ok(())
            }
        }
    }
    pub mod docker {
        use std::process::Command;
        use log::{debug, trace, warn};
        use singleton_derive::Singleton;
        use super::{
            DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
        };
        use crate::{
            dependencies::{
                apt_transport_https::AptTransportHttps, ca_certificates::CaCertificates,
                curl::Curl, gnupg::Gnupg,
            },
            OperatingSystem, CURRENT_USER, OPERATING_SYSTEM,
        };
        pub struct Docker {
            current_version: Option<String>,
            repo_available: bool,
            docker_installed: bool,
            docker_service_enabled: bool,
            docker_service_running: bool,
            user_in_docker_group: bool,
        }
        #[automatically_derived]
        impl ::core::default::Default for Docker {
            #[inline]
            fn default() -> Docker {
                Docker {
                    current_version: ::core::default::Default::default(),
                    repo_available: ::core::default::Default::default(),
                    docker_installed: ::core::default::Default::default(),
                    docker_service_enabled: ::core::default::Default::default(),
                    docker_service_running: ::core::default::Default::default(),
                    user_in_docker_group: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Docker {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "current_version",
                    "repo_available",
                    "docker_installed",
                    "docker_service_enabled",
                    "docker_service_running",
                    "user_in_docker_group",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.current_version,
                    &self.repo_available,
                    &self.docker_installed,
                    &self.docker_service_enabled,
                    &self.docker_service_running,
                    &&self.user_in_docker_group,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Docker",
                    names,
                    values,
                )
            }
        }
        static DOCKER: std::sync::OnceLock<&'static mut Docker> = std::sync::OnceLock::new();
        impl Docker {
            pub fn singleton() -> &'static mut Self {
                use std::sync::OnceLock;
                DOCKER
                    .get_or_init(|| {
                        ::std::boxed::Box::leak(::std::boxed::Box::new(Self::default()))
                    })
            }
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
                    | OperatingSystem::PopOS2104 => {
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                Curl::singleton(),
                                Gnupg::singleton(),
                                AptTransportHttps::singleton(),
                                CaCertificates::singleton(),
                            ]),
                        )
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "not implemented: {0}",
                                format_args!("Docker is not supported on this platform"),
                            ),
                        );
                    }
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
                match *OPERATING_SYSTEM {
                    OperatingSystem::Ubuntu2204
                    | OperatingSystem::Ubuntu2004
                    | OperatingSystem::Ubuntu1804
                    | OperatingSystem::PopOS2104 => {
                        {
                            let lvl = ::log::Level::Debug;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api_log(
                                    format_args!(
                                        "Checking if docker repo is available - ubuntu",
                                    ),
                                    lvl,
                                    &(
                                        "dotfiles::dependencies::docker",
                                        "dotfiles::dependencies::docker",
                                        "src/dependencies/docker.rs",
                                        61u32,
                                    ),
                                    ::log::__private_api::Option::None,
                                );
                            }
                        };
                        let output = Command::new("sh")
                            .arg("-c")
                            .arg("apt-cache policy docker-ce")
                            .output()?;
                        let stdout = String::from_utf8(output.stdout)?;
                        if !output.status.success() || stdout.is_empty()
                            || !stdout.contains("Candidate: 5:")
                        {
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Docker repo is not available"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            73u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            self.repo_available = false;
                            return Ok(InstallationStatus::NotInstalled);
                        } else {
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Docker repo is available"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            77u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            self.repo_available = true;
                        }
                    }
                    OperatingSystem::Fedora38
                    | OperatingSystem::Rocky8
                    | OperatingSystem::Rocky9 => {
                        {
                            let lvl = ::log::Level::Debug;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api_log(
                                    format_args!(
                                        "Checking if docker repo is available - fedora38",
                                    ),
                                    lvl,
                                    &(
                                        "dotfiles::dependencies::docker",
                                        "dotfiles::dependencies::docker",
                                        "src/dependencies/docker.rs",
                                        82u32,
                                    ),
                                    ::log::__private_api::Option::None,
                                );
                            }
                        };
                        let output = Command::new("sh")
                            .arg("-c")
                            .arg("dnf list docker-ce")
                            .output()?;
                        let stdout = String::from_utf8(output.stdout)?;
                        if !output.status.success() || stdout.is_empty()
                            || !stdout.contains("docker-ce.x86_64")
                        {
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Docker repo is not available"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            94u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            self.repo_available = false;
                            return Ok(InstallationStatus::NotInstalled);
                        } else {
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Docker repo is available"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            98u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            self.repo_available = true;
                        }
                    }
                }
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            format_args!("Checking if docker is installed"),
                            lvl,
                            &(
                                "dotfiles::dependencies::docker",
                                "dotfiles::dependencies::docker",
                                "src/dependencies/docker.rs",
                                105u32,
                            ),
                            ::log::__private_api::Option::None,
                        );
                    }
                };
                let output = Command::new("sh")
                    .arg("-c")
                    .arg("docker --version")
                    .output()?;
                let stdout = String::from_utf8(output.stdout).unwrap();
                if !output.status.success() || stdout.is_empty()
                    || !stdout.starts_with("Docker version")
                {
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                format_args!("Docker is not installed"),
                                lvl,
                                &(
                                    "dotfiles::dependencies::docker",
                                    "dotfiles::dependencies::docker",
                                    "src/dependencies/docker.rs",
                                    113u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    self.docker_installed = false;
                    return Ok(InstallationStatus::NotInstalled);
                } else {
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                format_args!("Docker is installed"),
                                lvl,
                                &(
                                    "dotfiles::dependencies::docker",
                                    "dotfiles::dependencies::docker",
                                    "src/dependencies/docker.rs",
                                    117u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    self.docker_installed = true;
                    self
                        .current_version = Some(
                        stdout
                            .split(' ')
                            .collect::<Vec<&str>>()[2]
                            .trim()
                            .replace(',', ""),
                    );
                }
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            format_args!(
                                "Checking if docker systemctl service is enabled",
                            ),
                            lvl,
                            &(
                                "dotfiles::dependencies::docker",
                                "dotfiles::dependencies::docker",
                                "src/dependencies/docker.rs",
                                127u32,
                            ),
                            ::log::__private_api::Option::None,
                        );
                    }
                };
                let output = Command::new("sh")
                    .arg("-c")
                    .arg("systemctl is-enabled docker")
                    .output()?;
                if !output.status.success()
                    || String::from_utf8(output.stdout).unwrap().trim() != "enabled"
                {
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                format_args!("Docker systemctl service is not enabled"),
                                lvl,
                                &(
                                    "dotfiles::dependencies::docker",
                                    "dotfiles::dependencies::docker",
                                    "src/dependencies/docker.rs",
                                    135u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    self.docker_service_enabled = false;
                } else {
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                format_args!("Docker systemctl service is enabled"),
                                lvl,
                                &(
                                    "dotfiles::dependencies::docker",
                                    "dotfiles::dependencies::docker",
                                    "src/dependencies/docker.rs",
                                    138u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    self.docker_service_enabled = true;
                }
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            format_args!(
                                "Checking if docker systemctl service is running",
                            ),
                            lvl,
                            &(
                                "dotfiles::dependencies::docker",
                                "dotfiles::dependencies::docker",
                                "src/dependencies/docker.rs",
                                143u32,
                            ),
                            ::log::__private_api::Option::None,
                        );
                    }
                };
                let output = Command::new("sh")
                    .arg("-c")
                    .arg("systemctl is-active docker")
                    .output()?;
                if !output.status.success()
                    || String::from_utf8(output.stdout).unwrap().trim() != "active"
                {
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                format_args!("Docker systemctl service is not running"),
                                lvl,
                                &(
                                    "dotfiles::dependencies::docker",
                                    "dotfiles::dependencies::docker",
                                    "src/dependencies/docker.rs",
                                    151u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    self.docker_service_running = false;
                } else {
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                format_args!("Docker systemctl service is running"),
                                lvl,
                                &(
                                    "dotfiles::dependencies::docker",
                                    "dotfiles::dependencies::docker",
                                    "src/dependencies/docker.rs",
                                    154u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    self.docker_service_running = true;
                }
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            format_args!(
                                "Checking if current user is in the docker group",
                            ),
                            lvl,
                            &(
                                "dotfiles::dependencies::docker",
                                "dotfiles::dependencies::docker",
                                "src/dependencies/docker.rs",
                                159u32,
                            ),
                            ::log::__private_api::Option::None,
                        );
                    }
                };
                let output = Command::new("sh")
                    .arg("-c")
                    .arg({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "sudo su -c \'sh -c groups\' {0}",
                                *CURRENT_USER,
                            ),
                        );
                        res
                    })
                    .output()?;
                if !output.status.success()
                    || !String::from_utf8(output.stdout)
                        .unwrap()
                        .trim()
                        .split(' ')
                        .any(|x| x == "docker")
                {
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                format_args!("Current user is not in the docker group"),
                                lvl,
                                &(
                                    "dotfiles::dependencies::docker",
                                    "dotfiles::dependencies::docker",
                                    "src/dependencies/docker.rs",
                                    173u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    self.user_in_docker_group = false;
                } else {
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                format_args!("Current user is in the docker group"),
                                lvl,
                                &(
                                    "dotfiles::dependencies::docker",
                                    "dotfiles::dependencies::docker",
                                    "src/dependencies/docker.rs",
                                    176u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    self.user_in_docker_group = true;
                }
                if !self.repo_available || !self.docker_installed
                    || !self.docker_service_enabled || !self.docker_service_running
                    || !self.user_in_docker_group
                {
                    {
                        let lvl = ::log::Level::Debug;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                format_args!("Docker is partially installed"),
                                lvl,
                                &(
                                    "dotfiles::dependencies::docker",
                                    "dotfiles::dependencies::docker",
                                    "src/dependencies/docker.rs",
                                    187u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    return Ok(InstallationStatus::PartialInstall);
                }
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            format_args!("Docker is fully installed"),
                            lvl,
                            &(
                                "dotfiles::dependencies::docker",
                                "dotfiles::dependencies::docker",
                                "src/dependencies/docker.rs",
                                191u32,
                            ),
                            ::log::__private_api::Option::None,
                        );
                    }
                };
                Ok(InstallationStatus::FullyInstalled)
            }
            /// Install the dependency.
            fn install(&self, version: Option<&str>) -> Result<(), DependencyError> {
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            format_args!(
                                "Installing docker using OS {0:?}",
                                *OPERATING_SYSTEM,
                            ),
                            lvl,
                            &(
                                "dotfiles::dependencies::docker",
                                "dotfiles::dependencies::docker",
                                "src/dependencies/docker.rs",
                                197u32,
                            ),
                            ::log::__private_api::Option::None,
                        );
                    }
                };
                if version.is_some() {
                    {
                        let lvl = ::log::Level::Warn;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                format_args!(
                                    "Custom version of docker is not supported. Installing latest version.",
                                ),
                                lvl,
                                &(
                                    "dotfiles::dependencies::docker",
                                    "dotfiles::dependencies::docker",
                                    "src/dependencies/docker.rs",
                                    201u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    }
                }
                match *OPERATING_SYSTEM {
                    OperatingSystem::Ubuntu2204
                    | OperatingSystem::Ubuntu2004
                    | OperatingSystem::Ubuntu1804
                    | OperatingSystem::PopOS2104 => {
                        {
                            let lvl = ::log::Level::Debug;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api_log(
                                    format_args!("Installing docker - ubuntu"),
                                    lvl,
                                    &(
                                        "dotfiles::dependencies::docker",
                                        "dotfiles::dependencies::docker",
                                        "src/dependencies/docker.rs",
                                        209u32,
                                    ),
                                    ::log::__private_api::Option::None,
                                );
                            }
                        };
                        if self.repo_available && self.docker_installed
                            && self.docker_service_enabled && self.docker_service_running
                            && self.user_in_docker_group
                        {
                            return Ok(());
                        }
                        if !self.repo_available {
                            {
                                let lvl = ::log::Level::Debug;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Adding docker repo"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            222u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Adding docker gpg key"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            244u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            let output = Command::new("sh")
                                .arg("-c")
                                .arg("install -m 0755 -d /etc/apt/keyrings")
                                .output()?;
                            if !output.status.success() {
                                return Err(
                                    DependencyError::DependencyFailed(
                                        "Unable to create /etc/apt/keyrings directory".to_string(),
                                    ),
                                );
                            }
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Adding docker gpg key"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            256u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            let output = Command::new("sh")
                                .arg("-c")
                                .arg(
                                    "curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg",
                                )
                                .output()?;
                            if !output.status.success() {
                                return Err(
                                    DependencyError::DependencyFailed(
                                        "Unable to add docker gpg key".to_string(),
                                    ),
                                );
                            }
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Adding docker gpg key"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            269u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            let output = Command::new("sh")
                                .arg("-c")
                                .arg("chmod a+r /etc/apt/keyrings/docker.gpg")
                                .output()?;
                            if !output.status.success() {
                                return Err(
                                    DependencyError::DependencyFailed(
                                        "Unable to chmod docker gpg key".to_string(),
                                    ),
                                );
                            }
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Adding docker repo"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            285u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            let output = Command::new("sh")
                                .arg("-c")
                                .arg(
                                    "echo \"deb [arch=\"$(dpkg --print-architecture)\" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \"$(. /etc/os-release && echo \"$VERSION_CODENAME\")\" stable\" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null",
                                )
                                .output()?;
                            if !output.status.success() {
                                return Err(
                                    DependencyError::DependencyFailed(
                                        "Unable to add docker repo".to_string(),
                                    ),
                                );
                            }
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Updating apt"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            300u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            let output = Command::new("sh")
                                .arg("-c")
                                .arg("apt-get update")
                                .output()?;
                            if !output.status.success() {
                                return Err(
                                    DependencyError::DependencyFailed(
                                        "Unable to update apt".to_string(),
                                    ),
                                );
                            }
                            self.repo_available = true;
                        }
                        if !self.docker_installed {
                            {
                                let lvl = ::log::Level::Debug;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Installing docker"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            317u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Installing docker"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            320u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            let output = Command::new("sh")
                                .arg("-c")
                                .arg(
                                    "apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin",
                                )
                                .output()?;
                            if !output.status.success() {
                                return Err(
                                    DependencyError::DependencyFailed(
                                        "Unable to install docker".to_string(),
                                    ),
                                );
                            }
                            self.docker_installed = true;
                        }
                        if !self.docker_service_enabled {
                            {
                                let lvl = ::log::Level::Debug;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Enabling docker service"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            338u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Enabling docker service"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            341u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            let output = Command::new("sh")
                                .arg("-c")
                                .arg("systemctl enable docker.service")
                                .output()?;
                            if !output.status.success() {
                                return Err(
                                    DependencyError::DependencyFailed(
                                        "Unable to enable docker service".to_string(),
                                    ),
                                );
                            }
                            self.docker_service_enabled = true;
                        }
                        if !self.docker_service_running {
                            {
                                let lvl = ::log::Level::Debug;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Starting docker service"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            358u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Starting docker service"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            361u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            let output = Command::new("sh")
                                .arg("-c")
                                .arg("systemctl start docker.service")
                                .output()?;
                            if !output.status.success() {
                                return Err(
                                    DependencyError::DependencyFailed(
                                        "Unable to start docker service".to_string(),
                                    ),
                                );
                            }
                            self.docker_service_running = true;
                        }
                        if !self.user_in_docker_group {
                            {
                                let lvl = ::log::Level::Debug;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Adding user to docker group"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            378u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            {
                                let lvl = ::log::Level::Trace;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api_log(
                                        format_args!("Adding user to docker group"),
                                        lvl,
                                        &(
                                            "dotfiles::dependencies::docker",
                                            "dotfiles::dependencies::docker",
                                            "src/dependencies/docker.rs",
                                            381u32,
                                        ),
                                        ::log::__private_api::Option::None,
                                    );
                                }
                            };
                            let output = Command::new("sh")
                                .arg("-c")
                                .arg({
                                    let res = ::alloc::fmt::format(
                                        format_args!("usermod -aG docker {0}", *CURRENT_USER),
                                    );
                                    res
                                })
                                .output()?;
                            if !output.status.success() {
                                return Err(
                                    DependencyError::DependencyFailed(
                                        "Unable to add user to docker group".to_string(),
                                    ),
                                );
                            }
                            self.user_in_docker_group = true;
                        }
                        Ok(())
                    }
                    OperatingSystem::Fedora38 => {
                        ::core::panicking::panic("not yet implemented")
                    }
                    OperatingSystem::Rocky8 => {
                        ::core::panicking::panic("not yet implemented")
                    }
                    OperatingSystem::Rocky9 => {
                        ::core::panicking::panic("not yet implemented")
                    }
                }
            }
        }
    }
    pub mod git {
        use std::process::Command;
        use singleton_derive::Singleton;
        use super::{
            DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
        };
        use crate::{OperatingSystem, OPERATING_SYSTEM};
        pub struct Git {
            git_available: bool,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Git {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Git",
                    "git_available",
                    &&self.git_available,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Git {
            #[inline]
            fn default() -> Git {
                Git {
                    git_available: ::core::default::Default::default(),
                }
            }
        }
        static GIT: std::sync::OnceLock<&'static mut Git> = std::sync::OnceLock::new();
        impl Git {
            pub fn singleton() -> &'static mut Self {
                use std::sync::OnceLock;
                GIT.get_or_init(|| {
                    ::std::boxed::Box::leak(::std::boxed::Box::new(Self::default()))
                })
            }
        }
        impl DependencyInfo for Git {
            fn name(&self) -> &'static str {
                "git"
            }
        }
        impl DependencyInstallable for Git {
            fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
                let output = Command::new("which")
                    .arg("git")
                    .spawn()?
                    .wait_with_output()?;
                if output.status.success() {
                    return Ok(InstallationStatus::FullyInstalled);
                }
                self.git_available = false;
                Ok(InstallationStatus::NotInstalled)
            }
            fn install(&self, _: Option<&str>) -> Result<(), DependencyError> {
                if self.git_available {
                    return Ok(());
                }
                match *OPERATING_SYSTEM {
                    OperatingSystem::Ubuntu1804
                    | OperatingSystem::Ubuntu2004
                    | OperatingSystem::Ubuntu2204 => {
                        let output = Command::new("sudo")
                            .arg("apt-get")
                            .arg("update")
                            .spawn()?
                            .wait_with_output()?;
                        if !output.status.success() {
                            return Err(
                                DependencyError::DependencyFailed(
                                    String::from("apt-get update"),
                                ),
                            );
                        }
                        let output = Command::new("sudo")
                            .arg("apt-get")
                            .arg("install")
                            .arg("git")
                            .spawn()?
                            .wait_with_output()?;
                        if !output.status.success() {
                            return Err(
                                DependencyError::DependencyFailed(
                                    String::from("apt-get install git"),
                                ),
                            );
                        }
                    }
                    _ => return Err(DependencyError::UnsupportedOperatingSystem),
                }
                Ok(())
            }
        }
    }
    pub mod tmux {
        use std::process::Command;
        use singleton_derive::Singleton;
        use super::{
            DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
        };
        use crate::{OperatingSystem, OPERATING_SYSTEM};
        pub struct Tmux {
            tmux_available: bool,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Tmux {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Tmux",
                    "tmux_available",
                    &&self.tmux_available,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Tmux {
            #[inline]
            fn default() -> Tmux {
                Tmux {
                    tmux_available: ::core::default::Default::default(),
                }
            }
        }
        static TMUX: std::sync::OnceLock<&'static mut Tmux> = std::sync::OnceLock::new();
        impl Tmux {
            pub fn singleton() -> &'static mut Self {
                use std::sync::OnceLock;
                TMUX.get_or_init(|| {
                    ::std::boxed::Box::leak(::std::boxed::Box::new(Self::default()))
                })
            }
        }
        impl DependencyInfo for Tmux {
            fn name(&self) -> &'static str {
                "tmux"
            }
        }
        impl DependencyInstallable for Tmux {
            fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
                let output = Command::new("which")
                    .arg("tmux")
                    .spawn()?
                    .wait_with_output()?;
                if output.status.success() {
                    return Ok(InstallationStatus::FullyInstalled);
                }
                self.tmux_available = false;
                Ok(InstallationStatus::NotInstalled)
            }
            fn install(&self, _: Option<&str>) -> Result<(), DependencyError> {
                if self.tmux_available {
                    return Ok(());
                }
                match *OPERATING_SYSTEM {
                    OperatingSystem::Ubuntu1804
                    | OperatingSystem::Ubuntu2004
                    | OperatingSystem::Ubuntu2204 => {
                        let output = Command::new("sudo")
                            .arg("apt-get")
                            .arg("update")
                            .spawn()?
                            .wait_with_output()?;
                        if !output.status.success() {
                            return Err(
                                DependencyError::DependencyFailed(
                                    String::from("apt-get update"),
                                ),
                            );
                        }
                        let output = Command::new("sudo")
                            .arg("apt-get")
                            .arg("install")
                            .arg("tmux")
                            .spawn()?
                            .wait_with_output()?;
                        if !output.status.success() {
                            return Err(
                                DependencyError::DependencyFailed(
                                    String::from("apt-get install tmux"),
                                ),
                            );
                        }
                    }
                    _ => {
                        return Err(DependencyError::UnsupportedOperatingSystem);
                    }
                }
                Ok(())
            }
        }
    }
    pub mod zsh {
        use std::process::Command;
        use log::trace;
        use singleton_derive::Singleton;
        use super::{
            DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
        };
        use crate::{
            dependencies::{tmux::Tmux, git::Git},
            OperatingSystem, OPERATING_SYSTEM,
        };
        pub struct Zsh {
            zsh_base_installed: bool,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Zsh {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Zsh",
                    "zsh_base_installed",
                    &&self.zsh_base_installed,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Zsh {
            #[inline]
            fn default() -> Zsh {
                Zsh {
                    zsh_base_installed: ::core::default::Default::default(),
                }
            }
        }
        static ZSH: std::sync::OnceLock<&'static mut Zsh> = std::sync::OnceLock::new();
        impl Zsh {
            pub fn singleton() -> &'static mut Self {
                use std::sync::OnceLock;
                ZSH.get_or_init(|| {
                    ::std::boxed::Box::leak(::std::boxed::Box::new(Self::default()))
                })
            }
        }
        impl DependencyInfo for Zsh {
            fn name(&self) -> &'static str {
                "zsh"
            }
            fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([Tmux::singleton(), Git::singleton()]),
                )
            }
        }
        impl DependencyInstallable for Zsh {
            fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
                let output = Command::new("zsh").arg("--version").output()?;
                let stdout = String::from_utf8(output.stdout)?;
                self
                    .zsh_base_installed = output.status.success() && !stdout.is_empty()
                    && stdout.contains("zsh");
                {
                    let lvl = ::log::Level::Trace;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            format_args!(
                                "zsh installation status is {0}",
                                self.zsh_base_installed,
                            ),
                            lvl,
                            &(
                                "dotfiles::dependencies::zsh",
                                "dotfiles::dependencies::zsh",
                                "src/dependencies/zsh.rs",
                                35u32,
                            ),
                            ::log::__private_api::Option::None,
                        );
                    }
                };
                match self.zsh_base_installed {
                    true => Ok(InstallationStatus::FullyInstalled),
                    false => Ok(InstallationStatus::NotInstalled),
                }
            }
            fn install(&self, _: Option<&str>) -> Result<(), DependencyError> {
                if !self.zsh_base_installed {
                    match *OPERATING_SYSTEM {
                        OperatingSystem::Ubuntu1804
                        | OperatingSystem::Ubuntu2004
                        | OperatingSystem::Ubuntu2204
                        | OperatingSystem::PopOS2104 => {
                            let output = Command::new("sudo")
                                .arg("apt-get")
                                .arg("update")
                                .output()?;
                            if !output.status.success() {
                                return Err(
                                    DependencyError::DependencyFailed(
                                        String::from("apt-get update"),
                                    ),
                                );
                            }
                            let output = Command::new("sudo")
                                .arg("apt-get")
                                .arg("install")
                                .arg("-y")
                                .arg("zsh")
                                .output()?;
                            if !output.status.success() {
                                return Err(
                                    DependencyError::DependencyFailed(
                                        String::from("apt-get install zsh"),
                                    ),
                                );
                            }
                            self.zsh_base_installed = true;
                        }
                        OperatingSystem::Fedora38 => {
                            let output = Command::new("sudo")
                                .arg("dnf")
                                .arg("install")
                                .arg("-y")
                                .arg("zsh")
                                .output()?;
                            if !output.status.success() {
                                return Err(
                                    DependencyError::DependencyFailed(
                                        String::from("dnf install zsh"),
                                    ),
                                );
                            }
                            self.zsh_base_installed = true;
                        }
                        _ => return Err(DependencyError::UnsupportedOperatingSystem),
                    }
                }
                Ok(())
            }
        }
    }
    pub mod zsh_autosuggestions {}
    pub mod zsh_syntax_highlighting {}
    pub mod zshrc {
        use std::fs::{metadata, read_to_string};
        use singleton_derive::Singleton;
        use super::{
            DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
            rename_bak_file, ConfigStatus,
        };
        use crate::{HOME_DIR, dependencies::zsh::Zsh};
        const ZSH_CONFIG_BASE: &str = "# shellcheck disable=SC1090,SC2034,SC1091,SC2148,SC2296,SC2296\n\nalias nzrc=\"nano ~/.zshrc\"\nalias szrc=\"source ~/.zshrc\"\nalias nsrc=\"nano ~/.ssh/config\"\n\nif [[ -r \"${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh\" ]]; then\n  source \"${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh\"\nfi\n\nexport ZSH=\"$HOME/.oh-my-zsh\"\n\nZSH_THEME=\"powerlevel10k/powerlevel10k\"\n\nPROMPT_EOL_MARK=\"\"\nHYPHEN_INSENSITIVE=\"true\"\nDISABLE_AUTO_UPDATE=\"true\"\nENABLE_CORRECTION=\"true\"\nCOMPLETION_WAITING_DOTS=\"true\"\n\nZSH_AUTOSUGGEST_HIGHLIGHT_STYLE=\'fg=#999\'\nZSH_AUTOSUGGEST_STRATEGY=(completion)\n\nplugins=(\n  grc\n  cp\n  urltools\n  safe-paste\n  universalarchive\n  sudo\n  rsync\n  zsh-syntax-highlighting\n  zsh-autosuggestions\n  # git\n  # tmux\n)\n\nsource \"$ZSH/oh-my-zsh.sh\"\n\n[[ ! -f ~/.p10k.zsh ]] || source ~/.p10k.zsh\nZSH_AUTOSUGGEST_USE_ASYNC=\"true\"\n\nsetopt nocorrectall; setopt correct\n\nsource \"$HOME/.zsh_aliases\"\n";
        pub struct Zshrc {
            zshrc_available: ConfigStatus,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Zshrc {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Zshrc",
                    "zshrc_available",
                    &&self.zshrc_available,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Zshrc {
            #[inline]
            fn default() -> Zshrc {
                Zshrc {
                    zshrc_available: ::core::default::Default::default(),
                }
            }
        }
        static ZSHRC: std::sync::OnceLock<&'static mut Zshrc> = std::sync::OnceLock::new();
        impl Zshrc {
            pub fn singleton() -> &'static mut Self {
                use std::sync::OnceLock;
                ZSHRC
                    .get_or_init(|| {
                        ::std::boxed::Box::leak(::std::boxed::Box::new(Self::default()))
                    })
            }
        }
        impl DependencyInfo for Zshrc {
            fn name(&self) -> &'static str {
                "zshrc"
            }
            fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([Zsh::singleton()]),
                )
            }
        }
        impl DependencyInstallable for Zshrc {
            fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
                let zshrc_path = {
                    let res = ::alloc::fmt::format(
                        format_args!("{0}/.zshrc", *HOME_DIR),
                    );
                    res
                };
                let is_present = metadata(zshrc_path.clone()).is_ok();
                let zshrc_contents = read_to_string(zshrc_path.clone())?;
                let is_correct = zshrc_contents
                    .lines()
                    .take(33)
                    .eq(ZSH_CONFIG_BASE.lines().take(33));
                self
                    .zshrc_available = match (is_present, is_correct) {
                    (false, _) => ConfigStatus::NotPresent,
                    (true, false) => ConfigStatus::PresentIncorrect,
                    (true, true) => ConfigStatus::PresentCorrect,
                };
                if match self.zshrc_available {
                    ConfigStatus::PresentCorrect => true,
                    _ => false,
                } {
                    Ok(InstallationStatus::FullyInstalled)
                } else {
                    Ok(InstallationStatus::NotInstalled)
                }
            }
            fn install(&self, _: Option<&str>) -> Result<(), DependencyError> {
                if match self.zshrc_available {
                    ConfigStatus::PresentCorrect => true,
                    _ => false,
                } {
                    return Ok(());
                }
                let zshrc_path = {
                    let res = ::alloc::fmt::format(
                        format_args!("{0}/.zshrc", *HOME_DIR),
                    );
                    res
                };
                rename_bak_file(&zshrc_path)?;
                std::fs::write(zshrc_path, ZSH_CONFIG_BASE)?;
                Ok(())
            }
        }
    }
    pub mod ohmyzsh {
        use std::{fs::metadata, process::Command};
        use lazy_static::lazy_static;
        use log::trace;
        use singleton_derive::Singleton;
        use super::{
            DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
        };
        use crate::{
            dependencies::{git::Git, zsh::Zsh},
            HOME_DIR,
        };
        const OH_MY_ZSH_GITHUB_URL: &str = "https://github.com/ohmyzsh/ohmyzsh";
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct OH_MY_ZSH_PATH {
            __private_field: (),
        }
        #[doc(hidden)]
        static OH_MY_ZSH_PATH: OH_MY_ZSH_PATH = OH_MY_ZSH_PATH {
            __private_field: (),
        };
        impl ::lazy_static::__Deref for OH_MY_ZSH_PATH {
            type Target = String;
            fn deref(&self) -> &String {
                #[inline(always)]
                fn __static_ref_initialize() -> String {
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("{0}/.oh-my-zsh", *HOME_DIR),
                        );
                        res
                    }
                }
                #[inline(always)]
                fn __stability() -> &'static String {
                    static LAZY: ::lazy_static::lazy::Lazy<String> = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for OH_MY_ZSH_PATH {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
        pub struct OhMyZsh {
            ohmyzsh_installed: bool,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for OhMyZsh {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "OhMyZsh",
                    "ohmyzsh_installed",
                    &&self.ohmyzsh_installed,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for OhMyZsh {
            #[inline]
            fn default() -> OhMyZsh {
                OhMyZsh {
                    ohmyzsh_installed: ::core::default::Default::default(),
                }
            }
        }
        static OH_MY_ZSH: std::sync::OnceLock<&'static mut OhMyZsh> = std::sync::OnceLock::new();
        impl OhMyZsh {
            pub fn singleton() -> &'static mut Self {
                use std::sync::OnceLock;
                OH_MY_ZSH
                    .get_or_init(|| {
                        ::std::boxed::Box::leak(::std::boxed::Box::new(Self::default()))
                    })
            }
        }
        impl DependencyInfo for OhMyZsh {
            fn name(&self) -> &'static str {
                "oh-my-zsh"
            }
            fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([Git::singleton(), Zsh::singleton()]),
                )
            }
        }
        impl DependencyInstallable for OhMyZsh {
            fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
                let is_present = metadata(&*OH_MY_ZSH_PATH).is_ok();
                self.ohmyzsh_installed = is_present;
                if !is_present {
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                format_args!("oh-my-zsh is not installed"),
                                lvl,
                                &(
                                    "dotfiles::dependencies::ohmyzsh",
                                    "dotfiles::dependencies::ohmyzsh",
                                    "src/dependencies/ohmyzsh.rs",
                                    42u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    Ok(InstallationStatus::NotInstalled)
                } else {
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                format_args!("oh-my-zsh is installed"),
                                lvl,
                                &(
                                    "dotfiles::dependencies::ohmyzsh",
                                    "dotfiles::dependencies::ohmyzsh",
                                    "src/dependencies/ohmyzsh.rs",
                                    45u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    Ok(InstallationStatus::FullyInstalled)
                }
            }
            fn install(&self, _: Option<&str>) -> Result<(), DependencyError> {
                if self.ohmyzsh_installed {
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api_log(
                                format_args!("oh-my-zsh is already installed"),
                                lvl,
                                &(
                                    "dotfiles::dependencies::ohmyzsh",
                                    "dotfiles::dependencies::ohmyzsh",
                                    "src/dependencies/ohmyzsh.rs",
                                    52u32,
                                ),
                                ::log::__private_api::Option::None,
                            );
                        }
                    };
                    return Ok(());
                }
                {
                    let lvl = ::log::Level::Trace;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api_log(
                            format_args!("Installing oh-my-zsh"),
                            lvl,
                            &(
                                "dotfiles::dependencies::ohmyzsh",
                                "dotfiles::dependencies::ohmyzsh",
                                "src/dependencies/ohmyzsh.rs",
                                56u32,
                            ),
                            ::log::__private_api::Option::None,
                        );
                    }
                };
                let output = Command::new("git")
                    .arg("clone")
                    .arg("--depth=1")
                    .arg(OH_MY_ZSH_GITHUB_URL)
                    .arg(&*OH_MY_ZSH_PATH)
                    .output()?;
                if !output.status.success() {
                    let stderr = String::from_utf8(output.stderr)?;
                    return Err(
                        DependencyError::DependencyFailed({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "oh-my-zsh failed to install due to error {0}",
                                    stderr,
                                ),
                            );
                            res
                        }),
                    );
                }
                self.ohmyzsh_installed = true;
                Ok(())
            }
        }
    }
    pub mod powerlevel10k {
        use std::{
            fs::{metadata, read_to_string},
            process::Command,
        };
        use singleton_derive::Singleton;
        use super::{
            DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
            rename_bak_file, ConfigStatus,
        };
        use crate::{HOME_DIR, dependencies::{zsh::Zsh, ohmyzsh::OhMyZsh}};
        use lazy_static::lazy_static;
        const POWER_LEVEL_10K_GITHUB_URL: &str = "https://gitee.com/romkatv/powerlevel10k.git";
        const POWER_LEVEL_10K_CONFIG_BASE: &str = "# Generated by Powerlevel10k configuration wizard on 2021-10-28 at 00:50 NZDT.\n# Based on romkatv/powerlevel10k/config/p10k-rainbow.zsh, checksum 19497.\n# Wizard options: powerline, rainbow, unicode, angled separators, sharp heads,\n# flat tails, 2 lines, dotted, full frame, dark-ornaments, sparse, fluent,\n# transient_prompt, instant_prompt=verbose.\n# Type `p10k configure` to generate another config.\n#\n# Config for Powerlevel10k with powerline prompt style with colorful background.\n# Type `p10k configure` to generate your own config based on it.\n#\n# Tip: Looking for a nice color? Here\'s a one-liner to print colormap.\n#\n#   for i in {0..255}; do print -Pn \"%K{$i}  %k%F{$i}${(l:3::0:)i}%f \" ${${(M)$((i%6)):#3}:+$\'\\n\'}; done\n\n# Temporarily change options.\n\'builtin\' \'local\' \'-a\' \'p10k_config_opts\'\n[[ ! -o \'aliases\'         ]] || p10k_config_opts+=(\'aliases\')\n[[ ! -o \'sh_glob\'         ]] || p10k_config_opts+=(\'sh_glob\')\n[[ ! -o \'no_brace_expand\' ]] || p10k_config_opts+=(\'no_brace_expand\')\n\'builtin\' \'setopt\' \'no_aliases\' \'no_sh_glob\' \'brace_expand\'\n\n() {\n  emulate -L zsh -o extended_glob\n\n  # Unset all configuration options. This allows you to apply configuration changes without\n  # restarting zsh. Edit ~/.p10k.zsh and type `source ~/.p10k.zsh`.\n  unset -m \'(POWERLEVEL9K_*|DEFAULT_USER)~POWERLEVEL9K_GITSTATUS_DIR\'\n\n  # Zsh >= 5.1 is required.\n  autoload -Uz is-at-least && is-at-least 5.1 || return\n\n  # The list of segments shown on the left. Fill it with the most important segments.\n  typeset -g POWERLEVEL9K_LEFT_PROMPT_ELEMENTS=(\n    # =========================[ Line #1 ]=========================\n    # os_icon               # os identifier\n    dir                     # current directory\n    vcs                     # git status\n    # =========================[ Line #2 ]=========================\n    newline                 # \\n\n    # prompt_char           # prompt symbol\n  )\n\n  # The list of segments shown on the right. Fill it with less important segments.\n  # Right prompt on the last prompt line (where you are typing your commands) gets\n  # automatically hidden when the input line reaches it. Right prompt above the\n  # last prompt line gets hidden if it would overlap with left prompt.\n  typeset -g POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS=(\n    # =========================[ Line #1 ]=========================\n    status                  # exit code of the last command\n    command_execution_time  # duration of the last command\n    background_jobs         # presence of background jobs\n    direnv                  # direnv status (https://direnv.net/)\n    asdf                    # asdf version manager (https://github.com/asdf-vm/asdf)\n    virtualenv              # python virtual environment (https://docs.python.org/3/library/venv.html)\n    anaconda                # conda environment (https://conda.io/)\n    pyenv                   # python environment (https://github.com/pyenv/pyenv)\n    goenv                   # go environment (https://github.com/syndbg/goenv)\n    nodenv                  # node.js version from nodenv (https://github.com/nodenv/nodenv)\n    nvm                     # node.js version from nvm (https://github.com/nvm-sh/nvm)\n    nodeenv                 # node.js environment (https://github.com/ekalinin/nodeenv)\n    # node_version          # node.js version\n    # go_version            # go version (https://golang.org)\n    # rust_version          # rustc version (https://www.rust-lang.org)\n    # dotnet_version        # .NET version (https://dotnet.microsoft.com)\n    # php_version           # php version (https://www.php.net/)\n    # laravel_version       # laravel php framework version (https://laravel.com/)\n    # java_version          # java version (https://www.java.com/)\n    # package               # name@version from package.json (https://docs.npmjs.com/files/package.json)\n    rbenv                   # ruby version from rbenv (https://github.com/rbenv/rbenv)\n    rvm                     # ruby version from rvm (https://rvm.io)\n    fvm                     # flutter version management (https://github.com/leoafarias/fvm)\n    luaenv                  # lua version from luaenv (https://github.com/cehoffman/luaenv)\n    jenv                    # java version from jenv (https://github.com/jenv/jenv)\n    plenv                   # perl version from plenv (https://github.com/tokuhirom/plenv)\n    phpenv                  # php version from phpenv (https://github.com/phpenv/phpenv)\n    scalaenv                # scala version from scalaenv (https://github.com/scalaenv/scalaenv)\n    haskell_stack           # haskell version from stack (https://haskellstack.org/)\n    kubecontext             # current kubernetes context (https://kubernetes.io/)\n    terraform               # terraform workspace (https://www.terraform.io)\n    # terraform_version     # terraform version (https://www.terraform.io)\n    aws                     # aws profile (https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-profiles.html)\n    aws_eb_env              # aws elastic beanstalk environment (https://aws.amazon.com/elasticbeanstalk/)\n    azure                   # azure account name (https://docs.microsoft.com/en-us/cli/azure)\n    gcloud                  # google cloud cli account and project (https://cloud.google.com/)\n    google_app_cred         # google application credentials (https://cloud.google.com/docs/authentication/production)\n    toolbox                 # toolbox name (https://github.com/containers/toolbox)\n    context                 # user@hostname\n    nordvpn                 # nordvpn connection status, linux only (https://nordvpn.com/)\n    ranger                  # ranger shell (https://github.com/ranger/ranger)\n    nnn                     # nnn shell (https://github.com/jarun/nnn)\n    xplr                    # xplr shell (https://github.com/sayanarijit/xplr)\n    vim_shell               # vim shell indicator (:sh)\n    midnight_commander      # midnight commander shell (https://midnight-commander.org/)\n    nix_shell               # nix shell (https://nixos.org/nixos/nix-pills/developing-with-nix-shell.html)\n    vi_mode                 # vi mode (you don\'t need this if you\'ve enabled prompt_char)\n    # vpn_ip                # virtual private network indicator\n    # load                  # CPU load\n    # disk_usage            # disk usage\n    # ram                   # free RAM\n    # swap                  # used swap\n    todo                    # todo items (https://github.com/todotxt/todo.txt-cli)\n    timewarrior             # timewarrior tracking status (https://timewarrior.net/)\n    taskwarrior             # taskwarrior task count (https://taskwarrior.org/)\n    # time                  # current time\n    # =========================[ Line #2 ]=========================\n    newline\n    # ip                    # ip address and bandwidth usage for a specified network interface\n    # public_ip             # public IP address\n    # proxy                 # system-wide http/https/ftp proxy\n    # battery               # internal battery\n    # wifi                  # wifi speed\n    # example               # example user-defined segment (see prompt_example function below)\n  )\n\n  # Defines character set used by powerlevel10k. It\'s best to let `p10k configure` set it for you.\n  typeset -g POWERLEVEL9K_MODE=powerline\n  # When set to `moderate`, some icons will have an extra space after them. This is meant to avoid\n  # icon overlap when using non-monospace fonts. When set to `none`, spaces are not added.\n  typeset -g POWERLEVEL9K_ICON_PADDING=none\n\n  # When set to true, icons appear before content on both sides of the prompt. When set\n  # to false, icons go after content. If empty or not set, icons go before content in the left\n  # prompt and after content in the right prompt.\n  #\n  # You can also override it for a specific segment:\n  #\n  #   POWERLEVEL9K_STATUS_ICON_BEFORE_CONTENT=false\n  #\n  # Or for a specific segment in specific state:\n  #\n  #   POWERLEVEL9K_DIR_NOT_WRITABLE_ICON_BEFORE_CONTENT=false\n  typeset -g POWERLEVEL9K_ICON_BEFORE_CONTENT=\n\n  # Add an empty line before each prompt.\n  typeset -g POWERLEVEL9K_PROMPT_ADD_NEWLINE=true\n\n  # Connect left prompt lines with these symbols. You\'ll probably want to use the same color\n  # as POWERLEVEL9K_MULTILINE_FIRST_PROMPT_GAP_FOREGROUND below.\n  typeset -g POWERLEVEL9K_MULTILINE_FIRST_PROMPT_PREFIX=\'%240F\u{256d}\u{2500}\'\n  typeset -g POWERLEVEL9K_MULTILINE_NEWLINE_PROMPT_PREFIX=\'%240F\u{251c}\u{2500}\'\n  typeset -g POWERLEVEL9K_MULTILINE_LAST_PROMPT_PREFIX=\'%240F\u{2570}\u{2500}\'\n  # Connect right prompt lines with these symbols.\n  typeset -g POWERLEVEL9K_MULTILINE_FIRST_PROMPT_SUFFIX=\'%240F\u{2500}\u{256e}\'\n  typeset -g POWERLEVEL9K_MULTILINE_NEWLINE_PROMPT_SUFFIX=\'%240F\u{2500}\u{2524}\'\n  typeset -g POWERLEVEL9K_MULTILINE_LAST_PROMPT_SUFFIX=\'%240F\u{2500}\u{256f}\'\n\n  # Filler between left and right prompt on the first prompt line. You can set it to \' \', \'\u{b7}\' or\n  # \'\u{2500}\'. The last two make it easier to see the alignment between left and right prompt and to\n  # separate prompt from command output. You might want to set POWERLEVEL9K_PROMPT_ADD_NEWLINE=false\n  # for more compact prompt if using using this option.\n  typeset -g POWERLEVEL9K_MULTILINE_FIRST_PROMPT_GAP_CHAR=\'\u{b7}\'\n  typeset -g POWERLEVEL9K_MULTILINE_FIRST_PROMPT_GAP_BACKGROUND=\n  typeset -g POWERLEVEL9K_MULTILINE_NEWLINE_PROMPT_GAP_BACKGROUND=\n  if [[ $POWERLEVEL9K_MULTILINE_FIRST_PROMPT_GAP_CHAR != \' \' ]]; then\n    # The color of the filler. You\'ll probably want to match the color of POWERLEVEL9K_MULTILINE\n    # ornaments defined above.\n    typeset -g POWERLEVEL9K_MULTILINE_FIRST_PROMPT_GAP_FOREGROUND=240\n    # Start filler from the edge of the screen if there are no left segments on the first line.\n    typeset -g POWERLEVEL9K_EMPTY_LINE_LEFT_PROMPT_FIRST_SEGMENT_END_SYMBOL=\'%{%}\'\n    # End filler on the edge of the screen if there are no right segments on the first line.\n    typeset -g POWERLEVEL9K_EMPTY_LINE_RIGHT_PROMPT_FIRST_SEGMENT_START_SYMBOL=\'%{%}\'\n  fi\n\n  # Separator between same-color segments on the left.\n  typeset -g POWERLEVEL9K_LEFT_SUBSEGMENT_SEPARATOR=\'\\uE0B1\'\n  # Separator between same-color segments on the right.\n  typeset -g POWERLEVEL9K_RIGHT_SUBSEGMENT_SEPARATOR=\'\\uE0B3\'\n  # Separator between different-color segments on the left.\n  typeset -g POWERLEVEL9K_LEFT_SEGMENT_SEPARATOR=\'\\uE0B0\'\n  # Separator between different-color segments on the right.\n  typeset -g POWERLEVEL9K_RIGHT_SEGMENT_SEPARATOR=\'\\uE0B2\'\n  # The right end of left prompt.\n  typeset -g POWERLEVEL9K_LEFT_PROMPT_LAST_SEGMENT_END_SYMBOL=\'\\uE0B0\'\n  # The left end of right prompt.\n  typeset -g POWERLEVEL9K_RIGHT_PROMPT_FIRST_SEGMENT_START_SYMBOL=\'\\uE0B2\'\n  # The left end of left prompt.\n  typeset -g POWERLEVEL9K_LEFT_PROMPT_FIRST_SEGMENT_START_SYMBOL=\'\'\n  # The right end of right prompt.\n  typeset -g POWERLEVEL9K_RIGHT_PROMPT_LAST_SEGMENT_END_SYMBOL=\'\'\n  # Left prompt terminator for lines without any segments.\n  typeset -g POWERLEVEL9K_EMPTY_LINE_LEFT_PROMPT_LAST_SEGMENT_END_SYMBOL=\n\n  #################################[ os_icon: os identifier ]##################################\n  # OS identifier color.\n  typeset -g POWERLEVEL9K_OS_ICON_FOREGROUND=232\n  typeset -g POWERLEVEL9K_OS_ICON_BACKGROUND=7\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_OS_ICON_CONTENT_EXPANSION=\'\u{2b50}\'\n\n  ################################[ prompt_char: prompt symbol ]################################\n  # Transparent background.\n  typeset -g POWERLEVEL9K_PROMPT_CHAR_BACKGROUND=\n  # Green prompt symbol if the last command succeeded.\n  typeset -g POWERLEVEL9K_PROMPT_CHAR_OK_{VIINS,VICMD,VIVIS,VIOWR}_FOREGROUND=76\n  # Red prompt symbol if the last command failed.\n  typeset -g POWERLEVEL9K_PROMPT_CHAR_ERROR_{VIINS,VICMD,VIVIS,VIOWR}_FOREGROUND=196\n  # Default prompt symbol.\n  typeset -g POWERLEVEL9K_PROMPT_CHAR_{OK,ERROR}_VIINS_CONTENT_EXPANSION=\'\u{276f}\'\n  # Prompt symbol in command vi mode.\n  typeset -g POWERLEVEL9K_PROMPT_CHAR_{OK,ERROR}_VICMD_CONTENT_EXPANSION=\'\u{276e}\'\n  # Prompt symbol in visual vi mode.\n  typeset -g POWERLEVEL9K_PROMPT_CHAR_{OK,ERROR}_VIVIS_CONTENT_EXPANSION=\'V\'\n  # Prompt symbol in overwrite vi mode.\n  typeset -g POWERLEVEL9K_PROMPT_CHAR_{OK,ERROR}_VIOWR_CONTENT_EXPANSION=\'\u{25b6}\'\n  typeset -g POWERLEVEL9K_PROMPT_CHAR_OVERWRITE_STATE=true\n  # No line terminator if prompt_char is the last segment.\n  typeset -g POWERLEVEL9K_PROMPT_CHAR_LEFT_PROMPT_LAST_SEGMENT_END_SYMBOL=\n  # No line introducer if prompt_char is the first segment.\n  typeset -g POWERLEVEL9K_PROMPT_CHAR_LEFT_PROMPT_FIRST_SEGMENT_START_SYMBOL=\n  # No surrounding whitespace.\n  typeset -g POWERLEVEL9K_PROMPT_CHAR_LEFT_{LEFT,RIGHT}_WHITESPACE=\n\n  ##################################[ dir: current directory ]##################################\n  # Current directory background color.\n  typeset -g POWERLEVEL9K_DIR_BACKGROUND=4\n  # Default current directory foreground color.\n  typeset -g POWERLEVEL9K_DIR_FOREGROUND=254\n  # If directory is too long, shorten some of its segments to the shortest possible unique\n  # prefix. The shortened directory can be tab-completed to the original.\n  typeset -g POWERLEVEL9K_SHORTEN_STRATEGY=truncate_to_unique\n  # Replace removed segment suffixes with this symbol.\n  typeset -g POWERLEVEL9K_SHORTEN_DELIMITER=\n  # Color of the shortened directory segments.\n  typeset -g POWERLEVEL9K_DIR_SHORTENED_FOREGROUND=250\n  # Color of the anchor directory segments. Anchor segments are never shortened. The first\n  # segment is always an anchor.\n  typeset -g POWERLEVEL9K_DIR_ANCHOR_FOREGROUND=255\n  # Display anchor directory segments in bold.\n  typeset -g POWERLEVEL9K_DIR_ANCHOR_BOLD=true\n  # Don\'t shorten directories that contain any of these files. They are anchors.\n  local anchor_files=(\n    .bzr\n    .citc\n    .git\n    .hg\n    .node-version\n    .python-version\n    .go-version\n    .ruby-version\n    .lua-version\n    .java-version\n    .perl-version\n    .php-version\n    .tool-version\n    .shorten_folder_marker\n    .svn\n    .terraform\n    CVS\n    Cargo.toml\n    composer.json\n    go.mod\n    package.json\n    stack.yaml\n  )\n  typeset -g POWERLEVEL9K_SHORTEN_FOLDER_MARKER=\"(${(j:|:)anchor_files})\"\n  # If set to \"first\" (\"last\"), remove everything before the first (last) subdirectory that contains\n  # files matching $POWERLEVEL9K_SHORTEN_FOLDER_MARKER. For example, when the current directory is\n  # /foo/bar/git_repo/nested_git_repo/baz, prompt will display git_repo/nested_git_repo/baz (first)\n  # or nested_git_repo/baz (last). This assumes that git_repo and nested_git_repo contain markers\n  # and other directories don\'t.\n  #\n  # Optionally, \"first\" and \"last\" can be followed by \":<offset>\" where <offset> is an integer.\n  # This moves the truncation point to the right (positive offset) or to the left (negative offset)\n  # relative to the marker. Plain \"first\" and \"last\" are equivalent to \"first:0\" and \"last:0\"\n  # respectively.\n  typeset -g POWERLEVEL9K_DIR_TRUNCATE_BEFORE_MARKER=false\n  # Don\'t shorten this many last directory segments. They are anchors.\n  typeset -g POWERLEVEL9K_SHORTEN_DIR_LENGTH=1\n  # Shorten directory if it\'s longer than this even if there is space for it. The value can\n  # be either absolute (e.g., \'80\') or a percentage of terminal width (e.g, \'50%\'). If empty,\n  # directory will be shortened only when prompt doesn\'t fit or when other parameters demand it\n  # (see POWERLEVEL9K_DIR_MIN_COMMAND_COLUMNS and POWERLEVEL9K_DIR_MIN_COMMAND_COLUMNS_PCT below).\n  # If set to `0`, directory will always be shortened to its minimum length.\n  typeset -g POWERLEVEL9K_DIR_MAX_LENGTH=80\n  # When `dir` segment is on the last prompt line, try to shorten it enough to leave at least this\n  # many columns for typing commands.\n  typeset -g POWERLEVEL9K_DIR_MIN_COMMAND_COLUMNS=40\n  # When `dir` segment is on the last prompt line, try to shorten it enough to leave at least\n  # COLUMNS * POWERLEVEL9K_DIR_MIN_COMMAND_COLUMNS_PCT * 0.01 columns for typing commands.\n  typeset -g POWERLEVEL9K_DIR_MIN_COMMAND_COLUMNS_PCT=50\n  # If set to true, embed a hyperlink into the directory. Useful for quickly\n  # opening a directory in the file manager simply by clicking the link.\n  # Can also be handy when the directory is shortened, as it allows you to see\n  # the full directory that was used in previous commands.\n  typeset -g POWERLEVEL9K_DIR_HYPERLINK=false\n\n  # Enable special styling for non-writable and non-existent directories. See POWERLEVEL9K_LOCK_ICON\n  # and POWERLEVEL9K_DIR_CLASSES below.\n  typeset -g POWERLEVEL9K_DIR_SHOW_WRITABLE=v3\n\n  # The default icon shown next to non-writable and non-existent directories when\n  # POWERLEVEL9K_DIR_SHOW_WRITABLE is set to v3.\n  typeset -g POWERLEVEL9K_LOCK_ICON=\'\u{2205}\'\n\n  # POWERLEVEL9K_DIR_CLASSES allows you to specify custom icons and colors for different\n  # directories. It must be an array with 3 * N elements. Each triplet consists of:\n  #\n  #   1. A pattern against which the current directory ($PWD) is matched. Matching is done with\n  #      extended_glob option enabled.\n  #   2. Directory class for the purpose of styling.\n  #   3. An empty string.\n  #\n  # Triplets are tried in order. The first triplet whose pattern matches $PWD wins.\n  #\n  # If POWERLEVEL9K_DIR_SHOW_WRITABLE is set to v3, non-writable and non-existent directories\n  # acquire class suffix _NOT_WRITABLE and NON_EXISTENT respectively.\n  #\n  # For example, given these settings:\n  #\n  #   typeset -g POWERLEVEL9K_DIR_CLASSES=(\n  #     \'~/work(|/*)\'  WORK     \'\'\n  #     \'~(|/*)\'       HOME     \'\'\n  #     \'*\'            DEFAULT  \'\')\n  #\n  # Whenever the current directory is ~/work or a subdirectory of ~/work, it gets styled with one\n  # of the following classes depending on its writability and existence: WORK, WORK_NOT_WRITABLE or\n  # WORK_NON_EXISTENT.\n  #\n  # Simply assigning classes to directories doesn\'t have any visible effects. It merely gives you an\n  # option to define custom colors and icons for different directory classes.\n  #\n  #   # Styling for WORK.\n  #   typeset -g POWERLEVEL9K_DIR_WORK_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  #   typeset -g POWERLEVEL9K_DIR_WORK_BACKGROUND=4\n  #   typeset -g POWERLEVEL9K_DIR_WORK_FOREGROUND=254\n  #   typeset -g POWERLEVEL9K_DIR_WORK_SHORTENED_FOREGROUND=250\n  #   typeset -g POWERLEVEL9K_DIR_WORK_ANCHOR_FOREGROUND=255\n  #\n  #   # Styling for WORK_NOT_WRITABLE.\n  #   typeset -g POWERLEVEL9K_DIR_WORK_NOT_WRITABLE_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  #   typeset -g POWERLEVEL9K_DIR_WORK_NOT_WRITABLE_BACKGROUND=4\n  #   typeset -g POWERLEVEL9K_DIR_WORK_NOT_WRITABLE_FOREGROUND=254\n  #   typeset -g POWERLEVEL9K_DIR_WORK_NOT_WRITABLE_SHORTENED_FOREGROUND=250\n  #   typeset -g POWERLEVEL9K_DIR_WORK_NOT_WRITABLE_ANCHOR_FOREGROUND=255\n  #\n  #   # Styling for WORK_NON_EXISTENT.\n  #   typeset -g POWERLEVEL9K_DIR_WORK_NON_EXISTENT_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  #   typeset -g POWERLEVEL9K_DIR_WORK_NON_EXISTENT_BACKGROUND=4\n  #   typeset -g POWERLEVEL9K_DIR_WORK_NON_EXISTENT_FOREGROUND=254\n  #   typeset -g POWERLEVEL9K_DIR_WORK_NON_EXISTENT_SHORTENED_FOREGROUND=250\n  #   typeset -g POWERLEVEL9K_DIR_WORK_NON_EXISTENT_ANCHOR_FOREGROUND=255\n  #\n  # If a styling parameter isn\'t explicitly defined for some class, it falls back to the classless\n  # parameter. For example, if POWERLEVEL9K_DIR_WORK_NOT_WRITABLE_FOREGROUND is not set, it falls\n  # back to POWERLEVEL9K_DIR_FOREGROUND.\n  #\n  typeset -g POWERLEVEL9K_DIR_CLASSES=()\n\n  # Custom prefix.\n  # typeset -g POWERLEVEL9K_DIR_PREFIX=\'in \'\n\n  #####################################[ vcs: git status ]######################################\n  # Version control system colors.\n  typeset -g POWERLEVEL9K_VCS_CLEAN_BACKGROUND=2\n  typeset -g POWERLEVEL9K_VCS_MODIFIED_BACKGROUND=3\n  typeset -g POWERLEVEL9K_VCS_UNTRACKED_BACKGROUND=2\n  typeset -g POWERLEVEL9K_VCS_CONFLICTED_BACKGROUND=3\n  typeset -g POWERLEVEL9K_VCS_LOADING_BACKGROUND=8\n\n  # Branch icon. Set this parameter to \'\\uF126 \' for the popular Powerline branch icon.\n  typeset -g POWERLEVEL9K_VCS_BRANCH_ICON=\n\n  # Untracked files icon. It\'s really a question mark, your font isn\'t broken.\n  # Change the value of this parameter to show a different icon.\n  typeset -g POWERLEVEL9K_VCS_UNTRACKED_ICON=\'?\'\n\n  # Formatter for Git status.\n  #\n  # Example output: master wip \u{21e3}42\u{21e1}42 *42 merge ~42 +42 !42 ?42.\n  #\n  # You can edit the function to customize how Git status looks.\n  #\n  # VCS_STATUS_* parameters are set by gitstatus plugin. See reference:\n  # https://github.com/romkatv/gitstatus/blob/master/gitstatus.plugin.zsh.\n  function my_git_formatter() {\n    emulate -L zsh\n\n    if [[ -n $P9K_CONTENT ]]; then\n      # If P9K_CONTENT is not empty, use it. It\'s either \"loading\" or from vcs_info (not from\n      # gitstatus plugin). VCS_STATUS_* parameters are not available in this case.\n      typeset -g my_git_format=$P9K_CONTENT\n      return\n    fi\n\n    # Styling for different parts of Git status.\n    local       meta=\'%7F\' # white foreground\n    local      clean=\'%0F\' # black foreground\n    local   modified=\'%0F\' # black foreground\n    local  untracked=\'%0F\' # black foreground\n    local conflicted=\'%1F\' # red foreground\n\n    local res\n\n    if [[ -n $VCS_STATUS_LOCAL_BRANCH ]]; then\n      local branch=${(V)VCS_STATUS_LOCAL_BRANCH}\n      # If local branch name is at most 32 characters long, show it in full.\n      # Otherwise show the first 12 \u{2026} the last 12.\n      # Tip: To always show local branch name in full without truncation, delete the next line.\n      (( $#branch > 32 )) && branch[13,-13]=\"\u{2026}\"  # <-- this line\n      res+=\"${clean}${(g::)POWERLEVEL9K_VCS_BRANCH_ICON}${branch//\\%/%%}\"\n    fi\n\n    if [[ -n $VCS_STATUS_TAG\n          # Show tag only if not on a branch.\n          # Tip: To always show tag, delete the next line.\n          && -z $VCS_STATUS_LOCAL_BRANCH  # <-- this line\n        ]]; then\n      local tag=${(V)VCS_STATUS_TAG}\n      # If tag name is at most 32 characters long, show it in full.\n      # Otherwise show the first 12 \u{2026} the last 12.\n      # Tip: To always show tag name in full without truncation, delete the next line.\n      (( $#tag > 32 )) && tag[13,-13]=\"\u{2026}\"  # <-- this line\n      res+=\"${meta}#${clean}${tag//\\%/%%}\"\n    fi\n\n    # Display the current Git commit if there is no branch and no tag.\n    # Tip: To always display the current Git commit, delete the next line.\n    [[ -z $VCS_STATUS_LOCAL_BRANCH && -z $VCS_STATUS_TAG ]] &&  # <-- this line\n      res+=\"${meta}@${clean}${VCS_STATUS_COMMIT[1,8]}\"\n\n    # Show tracking branch name if it differs from local branch.\n    if [[ -n ${VCS_STATUS_REMOTE_BRANCH:#$VCS_STATUS_LOCAL_BRANCH} ]]; then\n      res+=\"${meta}:${clean}${(V)VCS_STATUS_REMOTE_BRANCH//\\%/%%}\"\n    fi\n\n    # Display \"wip\" if the latest commit\'s summary contains \"wip\" or \"WIP\".\n    if [[ $VCS_STATUS_COMMIT_SUMMARY == (|*[^[:alnum:]])(wip|WIP)(|[^[:alnum:]]*) ]]; then\n      res+=\" ${modified}wip\"\n    fi\n\n    # \u{21e3}42 if behind the remote.\n    (( VCS_STATUS_COMMITS_BEHIND )) && res+=\" ${clean}\u{21e3}${VCS_STATUS_COMMITS_BEHIND}\"\n    # \u{21e1}42 if ahead of the remote; no leading space if also behind the remote: \u{21e3}42\u{21e1}42.\n    (( VCS_STATUS_COMMITS_AHEAD && !VCS_STATUS_COMMITS_BEHIND )) && res+=\" \"\n    (( VCS_STATUS_COMMITS_AHEAD  )) && res+=\"${clean}\u{21e1}${VCS_STATUS_COMMITS_AHEAD}\"\n    # \u{21e0}42 if behind the push remote.\n    (( VCS_STATUS_PUSH_COMMITS_BEHIND )) && res+=\" ${clean}\u{21e0}${VCS_STATUS_PUSH_COMMITS_BEHIND}\"\n    (( VCS_STATUS_PUSH_COMMITS_AHEAD && !VCS_STATUS_PUSH_COMMITS_BEHIND )) && res+=\" \"\n    # \u{21e2}42 if ahead of the push remote; no leading space if also behind: \u{21e0}42\u{21e2}42.\n    (( VCS_STATUS_PUSH_COMMITS_AHEAD  )) && res+=\"${clean}\u{21e2}${VCS_STATUS_PUSH_COMMITS_AHEAD}\"\n    # *42 if have stashes.\n    (( VCS_STATUS_STASHES        )) && res+=\" ${clean}*${VCS_STATUS_STASHES}\"\n    # \'merge\' if the repo is in an unusual state.\n    [[ -n $VCS_STATUS_ACTION     ]] && res+=\" ${conflicted}${VCS_STATUS_ACTION}\"\n    # ~42 if have merge conflicts.\n    (( VCS_STATUS_NUM_CONFLICTED )) && res+=\" ${conflicted}~${VCS_STATUS_NUM_CONFLICTED}\"\n    # +42 if have staged changes.\n    (( VCS_STATUS_NUM_STAGED     )) && res+=\" ${modified}+${VCS_STATUS_NUM_STAGED}\"\n    # !42 if have unstaged changes.\n    (( VCS_STATUS_NUM_UNSTAGED   )) && res+=\" ${modified}!${VCS_STATUS_NUM_UNSTAGED}\"\n    # ?42 if have untracked files. It\'s really a question mark, your font isn\'t broken.\n    # See POWERLEVEL9K_VCS_UNTRACKED_ICON above if you want to use a different icon.\n    # Remove the next line if you don\'t want to see untracked files at all.\n    (( VCS_STATUS_NUM_UNTRACKED  )) && res+=\" ${untracked}${(g::)POWERLEVEL9K_VCS_UNTRACKED_ICON}${VCS_STATUS_NUM_UNTRACKED}\"\n    # \"\u{2500}\" if the number of unstaged files is unknown. This can happen due to\n    # POWERLEVEL9K_VCS_MAX_INDEX_SIZE_DIRTY (see below) being set to a non-negative number lower\n    # than the number of files in the Git index, or due to bash.showDirtyState being set to false\n    # in the repository config. The number of staged and untracked files may also be unknown\n    # in this case.\n    (( VCS_STATUS_HAS_UNSTAGED == -1 )) && res+=\" ${modified}\u{2500}\"\n\n    typeset -g my_git_format=$res\n  }\n  functions -M my_git_formatter 2>/dev/null\n\n  # Don\'t count the number of unstaged, untracked and conflicted files in Git repositories with\n  # more than this many files in the index. Negative value means infinity.\n  #\n  # If you are working in Git repositories with tens of millions of files and seeing performance\n  # sagging, try setting POWERLEVEL9K_VCS_MAX_INDEX_SIZE_DIRTY to a number lower than the output\n  # of `git ls-files | wc -l`. Alternatively, add `bash.showDirtyState = false` to the repository\'s\n  # config: `git config bash.showDirtyState false`.\n  typeset -g POWERLEVEL9K_VCS_MAX_INDEX_SIZE_DIRTY=-1\n\n  # Don\'t show Git status in prompt for repositories whose workdir matches this pattern.\n  # For example, if set to \'~\', the Git repository at $HOME/.git will be ignored.\n  # Multiple patterns can be combined with \'|\': \'~(|/foo)|/bar/baz/*\'.\n  typeset -g POWERLEVEL9K_VCS_DISABLED_WORKDIR_PATTERN=\'~\'\n\n  # Disable the default Git status formatting.\n  typeset -g POWERLEVEL9K_VCS_DISABLE_GITSTATUS_FORMATTING=true\n  # Install our own Git status formatter.\n  typeset -g POWERLEVEL9K_VCS_CONTENT_EXPANSION=\'${$((my_git_formatter()))+${my_git_format}}\'\n  # Enable counters for staged, unstaged, etc.\n  typeset -g POWERLEVEL9K_VCS_{STAGED,UNSTAGED,UNTRACKED,CONFLICTED,COMMITS_AHEAD,COMMITS_BEHIND}_MAX_NUM=-1\n\n  # Custom icon.\n  typeset -g POWERLEVEL9K_VCS_VISUAL_IDENTIFIER_EXPANSION=\n  # Custom prefix.\n  typeset -g POWERLEVEL9K_VCS_PREFIX=\'on \'\n\n  # Show status of repositories of these types. You can add svn and/or hg if you are\n  # using them. If you do, your prompt may become slow even when your current directory\n  # isn\'t in an svn or hg reposotiry.\n  typeset -g POWERLEVEL9K_VCS_BACKENDS=(git)\n\n  ##########################[ status: exit code of the last command ]###########################\n  # Enable OK_PIPE, ERROR_PIPE and ERROR_SIGNAL status states to allow us to enable, disable and\n  # style them independently from the regular OK and ERROR state.\n  typeset -g POWERLEVEL9K_STATUS_EXTENDED_STATES=true\n\n  # Status on success. No content, just an icon. No need to show it if prompt_char is enabled as\n  # it will signify success by turning green.\n  typeset -g POWERLEVEL9K_STATUS_OK=true\n  typeset -g POWERLEVEL9K_STATUS_OK_VISUAL_IDENTIFIER_EXPANSION=\'\u{2714}\'\n  typeset -g POWERLEVEL9K_STATUS_OK_FOREGROUND=2\n  typeset -g POWERLEVEL9K_STATUS_OK_BACKGROUND=0\n\n  # Status when some part of a pipe command fails but the overall exit status is zero. It may look\n  # like this: 1|0.\n  typeset -g POWERLEVEL9K_STATUS_OK_PIPE=true\n  typeset -g POWERLEVEL9K_STATUS_OK_PIPE_VISUAL_IDENTIFIER_EXPANSION=\'\u{2714}\'\n  typeset -g POWERLEVEL9K_STATUS_OK_PIPE_FOREGROUND=2\n  typeset -g POWERLEVEL9K_STATUS_OK_PIPE_BACKGROUND=0\n\n  # Status when it\'s just an error code (e.g., \'1\'). No need to show it if prompt_char is enabled as\n  # it will signify error by turning red.\n  typeset -g POWERLEVEL9K_STATUS_ERROR=true\n  typeset -g POWERLEVEL9K_STATUS_ERROR_VISUAL_IDENTIFIER_EXPANSION=\'\u{2718}\'\n  typeset -g POWERLEVEL9K_STATUS_ERROR_FOREGROUND=3\n  typeset -g POWERLEVEL9K_STATUS_ERROR_BACKGROUND=1\n\n  # Status when the last command was terminated by a signal.\n  typeset -g POWERLEVEL9K_STATUS_ERROR_SIGNAL=true\n  # Use terse signal names: \"INT\" instead of \"SIGINT(2)\".\n  typeset -g POWERLEVEL9K_STATUS_VERBOSE_SIGNAME=false\n  typeset -g POWERLEVEL9K_STATUS_ERROR_SIGNAL_VISUAL_IDENTIFIER_EXPANSION=\'\u{2718}\'\n  typeset -g POWERLEVEL9K_STATUS_ERROR_SIGNAL_FOREGROUND=3\n  typeset -g POWERLEVEL9K_STATUS_ERROR_SIGNAL_BACKGROUND=1\n\n  # Status when some part of a pipe command fails and the overall exit status is also non-zero.\n  # It may look like this: 1|0.\n  typeset -g POWERLEVEL9K_STATUS_ERROR_PIPE=true\n  typeset -g POWERLEVEL9K_STATUS_ERROR_PIPE_VISUAL_IDENTIFIER_EXPANSION=\'\u{2718}\'\n  typeset -g POWERLEVEL9K_STATUS_ERROR_PIPE_FOREGROUND=3\n  typeset -g POWERLEVEL9K_STATUS_ERROR_PIPE_BACKGROUND=1\n\n  ###################[ command_execution_time: duration of the last command ]###################\n  # Execution time color.\n  typeset -g POWERLEVEL9K_COMMAND_EXECUTION_TIME_FOREGROUND=0\n  typeset -g POWERLEVEL9K_COMMAND_EXECUTION_TIME_BACKGROUND=3\n  # Show duration of the last command if takes at least this many seconds.\n  typeset -g POWERLEVEL9K_COMMAND_EXECUTION_TIME_THRESHOLD=3\n  # Show this many fractional digits. Zero means round to seconds.\n  typeset -g POWERLEVEL9K_COMMAND_EXECUTION_TIME_PRECISION=0\n  # Duration format: 1d 2h 3m 4s.\n  typeset -g POWERLEVEL9K_COMMAND_EXECUTION_TIME_FORMAT=\'d h m s\'\n  # Custom icon.\n  typeset -g POWERLEVEL9K_COMMAND_EXECUTION_TIME_VISUAL_IDENTIFIER_EXPANSION=\n  # Custom prefix.\n  typeset -g POWERLEVEL9K_COMMAND_EXECUTION_TIME_PREFIX=\'took \'\n\n  #######################[ background_jobs: presence of background jobs ]#######################\n  # Background jobs color.\n  typeset -g POWERLEVEL9K_BACKGROUND_JOBS_FOREGROUND=6\n  typeset -g POWERLEVEL9K_BACKGROUND_JOBS_BACKGROUND=0\n  # Don\'t show the number of background jobs.\n  typeset -g POWERLEVEL9K_BACKGROUND_JOBS_VERBOSE=false\n  # Custom icon.\n  typeset -g POWERLEVEL9K_BACKGROUND_JOBS_VISUAL_IDENTIFIER_EXPANSION=\'\u{2261}\'\n\n  #######################[ direnv: direnv status (https://direnv.net/) ]########################\n  # Direnv color.\n  typeset -g POWERLEVEL9K_DIRENV_FOREGROUND=3\n  typeset -g POWERLEVEL9K_DIRENV_BACKGROUND=0\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_DIRENV_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ###############[ asdf: asdf version manager (https://github.com/asdf-vm/asdf) ]###############\n  # Default asdf color. Only used to display tools for which there is no color override (see below).\n  # Tip:  Override these parameters for ${TOOL} with POWERLEVEL9K_ASDF_${TOOL}_FOREGROUND and\n  # POWERLEVEL9K_ASDF_${TOOL}_BACKGROUND.\n  typeset -g POWERLEVEL9K_ASDF_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ASDF_BACKGROUND=7\n\n  # There are four parameters that can be used to hide asdf tools. Each parameter describes\n  # conditions under which a tool gets hidden. Parameters can hide tools but not unhide them. If at\n  # least one parameter decides to hide a tool, that tool gets hidden. If no parameter decides to\n  # hide a tool, it gets shown.\n  #\n  # Special note on the difference between POWERLEVEL9K_ASDF_SOURCES and\n  # POWERLEVEL9K_ASDF_PROMPT_ALWAYS_SHOW. Consider the effect of the following commands:\n  #\n  #   asdf local  python 3.8.1\n  #   asdf global python 3.8.1\n  #\n  # After running both commands the current python version is 3.8.1 and its source is \"local\" as\n  # it takes precedence over \"global\". If POWERLEVEL9K_ASDF_PROMPT_ALWAYS_SHOW is set to false,\n  # it\'ll hide python version in this case because 3.8.1 is the same as the global version.\n  # POWERLEVEL9K_ASDF_SOURCES will hide python version only if the value of this parameter doesn\'t\n  # contain \"local\".\n\n  # Hide tool versions that don\'t come from one of these sources.\n  #\n  # Available sources:\n  #\n  # - shell   `asdf current` says \"set by ASDF_${TOOL}_VERSION environment variable\"\n  # - local   `asdf current` says \"set by /some/not/home/directory/file\"\n  # - global  `asdf current` says \"set by /home/username/file\"\n  #\n  # Note: If this parameter is set to (shell local global), it won\'t hide tools.\n  # Tip:  Override this parameter for ${TOOL} with POWERLEVEL9K_ASDF_${TOOL}_SOURCES.\n  typeset -g POWERLEVEL9K_ASDF_SOURCES=(shell local global)\n\n  # If set to false, hide tool versions that are the same as global.\n  #\n  # Note: The name of this parameter doesn\'t reflect its meaning at all.\n  # Note: If this parameter is set to true, it won\'t hide tools.\n  # Tip:  Override this parameter for ${TOOL} with POWERLEVEL9K_ASDF_${TOOL}_PROMPT_ALWAYS_SHOW.\n  typeset -g POWERLEVEL9K_ASDF_PROMPT_ALWAYS_SHOW=false\n\n  # If set to false, hide tool versions that are equal to \"system\".\n  #\n  # Note: If this parameter is set to true, it won\'t hide tools.\n  # Tip: Override this parameter for ${TOOL} with POWERLEVEL9K_ASDF_${TOOL}_SHOW_SYSTEM.\n  typeset -g POWERLEVEL9K_ASDF_SHOW_SYSTEM=true\n\n  # If set to non-empty value, hide tools unless there is a file matching the specified file pattern\n  # in the current directory, or its parent directory, or its grandparent directory, and so on.\n  #\n  # Note: If this parameter is set to empty value, it won\'t hide tools.\n  # Note: SHOW_ON_UPGLOB isn\'t specific to asdf. It works with all prompt segments.\n  # Tip: Override this parameter for ${TOOL} with POWERLEVEL9K_ASDF_${TOOL}_SHOW_ON_UPGLOB.\n  #\n  # Example: Hide nodejs version when there is no package.json and no *.js files in the current\n  # directory, in `..`, in `../..` and so on.\n  #\n  #   typeset -g POWERLEVEL9K_ASDF_NODEJS_SHOW_ON_UPGLOB=\'*.js|package.json\'\n  typeset -g POWERLEVEL9K_ASDF_SHOW_ON_UPGLOB=\n\n  # Ruby version from asdf.\n  typeset -g POWERLEVEL9K_ASDF_RUBY_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ASDF_RUBY_BACKGROUND=1\n  # typeset -g POWERLEVEL9K_ASDF_RUBY_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # typeset -g POWERLEVEL9K_ASDF_RUBY_SHOW_ON_UPGLOB=\'*.foo|*.bar\'\n\n  # Python version from asdf.\n  typeset -g POWERLEVEL9K_ASDF_PYTHON_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ASDF_PYTHON_BACKGROUND=4\n  # typeset -g POWERLEVEL9K_ASDF_PYTHON_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # typeset -g POWERLEVEL9K_ASDF_PYTHON_SHOW_ON_UPGLOB=\'*.foo|*.bar\'\n\n  # Go version from asdf.\n  typeset -g POWERLEVEL9K_ASDF_GOLANG_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ASDF_GOLANG_BACKGROUND=4\n  # typeset -g POWERLEVEL9K_ASDF_GOLANG_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # typeset -g POWERLEVEL9K_ASDF_GOLANG_SHOW_ON_UPGLOB=\'*.foo|*.bar\'\n\n  # Node.js version from asdf.\n  typeset -g POWERLEVEL9K_ASDF_NODEJS_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ASDF_NODEJS_BACKGROUND=2\n  # typeset -g POWERLEVEL9K_ASDF_NODEJS_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # typeset -g POWERLEVEL9K_ASDF_NODEJS_SHOW_ON_UPGLOB=\'*.foo|*.bar\'\n\n  # Rust version from asdf.\n  typeset -g POWERLEVEL9K_ASDF_RUST_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ASDF_RUST_BACKGROUND=208\n  # typeset -g POWERLEVEL9K_ASDF_RUST_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # typeset -g POWERLEVEL9K_ASDF_RUST_SHOW_ON_UPGLOB=\'*.foo|*.bar\'\n\n  # .NET Core version from asdf.\n  typeset -g POWERLEVEL9K_ASDF_DOTNET_CORE_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ASDF_DOTNET_CORE_BACKGROUND=5\n  # typeset -g POWERLEVEL9K_ASDF_DOTNET_CORE_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # typeset -g POWERLEVEL9K_ASDF_DOTNET_CORE_SHOW_ON_UPGLOB=\'*.foo|*.bar\'\n\n  # Flutter version from asdf.\n  typeset -g POWERLEVEL9K_ASDF_FLUTTER_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ASDF_FLUTTER_BACKGROUND=4\n  # typeset -g POWERLEVEL9K_ASDF_FLUTTER_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # typeset -g POWERLEVEL9K_ASDF_FLUTTER_SHOW_ON_UPGLOB=\'*.foo|*.bar\'\n\n  # Lua version from asdf.\n  typeset -g POWERLEVEL9K_ASDF_LUA_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ASDF_LUA_BACKGROUND=4\n  # typeset -g POWERLEVEL9K_ASDF_LUA_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # typeset -g POWERLEVEL9K_ASDF_LUA_SHOW_ON_UPGLOB=\'*.foo|*.bar\'\n\n  # Java version from asdf.\n  typeset -g POWERLEVEL9K_ASDF_JAVA_FOREGROUND=1\n  typeset -g POWERLEVEL9K_ASDF_JAVA_BACKGROUND=7\n  # typeset -g POWERLEVEL9K_ASDF_JAVA_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # typeset -g POWERLEVEL9K_ASDF_JAVA_SHOW_ON_UPGLOB=\'*.foo|*.bar\'\n\n  # Perl version from asdf.\n  typeset -g POWERLEVEL9K_ASDF_PERL_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ASDF_PERL_BACKGROUND=4\n  # typeset -g POWERLEVEL9K_ASDF_PERL_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # typeset -g POWERLEVEL9K_ASDF_PERL_SHOW_ON_UPGLOB=\'*.foo|*.bar\'\n\n  # Erlang version from asdf.\n  typeset -g POWERLEVEL9K_ASDF_ERLANG_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ASDF_ERLANG_BACKGROUND=1\n  # typeset -g POWERLEVEL9K_ASDF_ERLANG_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # typeset -g POWERLEVEL9K_ASDF_ERLANG_SHOW_ON_UPGLOB=\'*.foo|*.bar\'\n\n  # Elixir version from asdf.\n  typeset -g POWERLEVEL9K_ASDF_ELIXIR_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ASDF_ELIXIR_BACKGROUND=5\n  # typeset -g POWERLEVEL9K_ASDF_ELIXIR_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # typeset -g POWERLEVEL9K_ASDF_ELIXIR_SHOW_ON_UPGLOB=\'*.foo|*.bar\'\n\n  # Postgres version from asdf.\n  typeset -g POWERLEVEL9K_ASDF_POSTGRES_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ASDF_POSTGRES_BACKGROUND=6\n  # typeset -g POWERLEVEL9K_ASDF_POSTGRES_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # typeset -g POWERLEVEL9K_ASDF_POSTGRES_SHOW_ON_UPGLOB=\'*.foo|*.bar\'\n\n  # PHP version from asdf.\n  typeset -g POWERLEVEL9K_ASDF_PHP_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ASDF_PHP_BACKGROUND=5\n  # typeset -g POWERLEVEL9K_ASDF_PHP_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # typeset -g POWERLEVEL9K_ASDF_PHP_SHOW_ON_UPGLOB=\'*.foo|*.bar\'\n\n  # Haskell version from asdf.\n  typeset -g POWERLEVEL9K_ASDF_HASKELL_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ASDF_HASKELL_BACKGROUND=3\n  # typeset -g POWERLEVEL9K_ASDF_HASKELL_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # typeset -g POWERLEVEL9K_ASDF_HASKELL_SHOW_ON_UPGLOB=\'*.foo|*.bar\'\n\n  # Julia version from asdf.\n  typeset -g POWERLEVEL9K_ASDF_JULIA_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ASDF_JULIA_BACKGROUND=2\n  # typeset -g POWERLEVEL9K_ASDF_JULIA_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # typeset -g POWERLEVEL9K_ASDF_JULIA_SHOW_ON_UPGLOB=\'*.foo|*.bar\'\n\n  ##########[ nordvpn: nordvpn connection status, linux only (https://nordvpn.com/) ]###########\n  # NordVPN connection indicator color.\n  typeset -g POWERLEVEL9K_NORDVPN_FOREGROUND=7\n  typeset -g POWERLEVEL9K_NORDVPN_BACKGROUND=4\n  # Hide NordVPN connection indicator when not connected.\n  typeset -g POWERLEVEL9K_NORDVPN_{DISCONNECTED,CONNECTING,DISCONNECTING}_CONTENT_EXPANSION=\n  typeset -g POWERLEVEL9K_NORDVPN_{DISCONNECTED,CONNECTING,DISCONNECTING}_VISUAL_IDENTIFIER_EXPANSION=\n  # Custom icon.\n  typeset -g POWERLEVEL9K_NORDVPN_VISUAL_IDENTIFIER_EXPANSION=\'nord\'\n\n  #################[ ranger: ranger shell (https://github.com/ranger/ranger) ]##################\n  # Ranger shell color.\n  typeset -g POWERLEVEL9K_RANGER_FOREGROUND=3\n  typeset -g POWERLEVEL9K_RANGER_BACKGROUND=0\n  # Custom icon.\n  typeset -g POWERLEVEL9K_RANGER_VISUAL_IDENTIFIER_EXPANSION=\'\u{25b2}\'\n\n  ######################[ nnn: nnn shell (https://github.com/jarun/nnn) ]#######################\n  # Nnn shell color.\n  typeset -g POWERLEVEL9K_NNN_FOREGROUND=0\n  typeset -g POWERLEVEL9K_NNN_BACKGROUND=6\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_NNN_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ##################[ xplr: xplr shell (https://github.com/sayanarijit/xplr) ]##################\n  # xplr shell color.\n  typeset -g POWERLEVEL9K_XPLR_FOREGROUND=0\n  typeset -g POWERLEVEL9K_XPLR_BACKGROUND=6\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_XPLR_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ###########################[ vim_shell: vim shell indicator (:sh) ]###########################\n  # Vim shell indicator color.\n  typeset -g POWERLEVEL9K_VIM_SHELL_FOREGROUND=0\n  typeset -g POWERLEVEL9K_VIM_SHELL_BACKGROUND=2\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_VIM_SHELL_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ######[ midnight_commander: midnight commander shell (https://midnight-commander.org/) ]######\n  # Midnight Commander shell color.\n  typeset -g POWERLEVEL9K_MIDNIGHT_COMMANDER_FOREGROUND=3\n  typeset -g POWERLEVEL9K_MIDNIGHT_COMMANDER_BACKGROUND=0\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_MIDNIGHT_COMMANDER_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  #[ nix_shell: nix shell (https://nixos.org/nixos/nix-pills/developing-with-nix-shell.html) ]##\n  # Nix shell color.\n  typeset -g POWERLEVEL9K_NIX_SHELL_FOREGROUND=0\n  typeset -g POWERLEVEL9K_NIX_SHELL_BACKGROUND=4\n\n  # Tip: If you want to see just the icon without \"pure\" and \"impure\", uncomment the next line.\n  # typeset -g POWERLEVEL9K_NIX_SHELL_CONTENT_EXPANSION=\n\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_NIX_SHELL_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ##################################[ disk_usage: disk usage ]##################################\n  # Colors for different levels of disk usage.\n  typeset -g POWERLEVEL9K_DISK_USAGE_NORMAL_FOREGROUND=3\n  typeset -g POWERLEVEL9K_DISK_USAGE_NORMAL_BACKGROUND=0\n  typeset -g POWERLEVEL9K_DISK_USAGE_WARNING_FOREGROUND=0\n  typeset -g POWERLEVEL9K_DISK_USAGE_WARNING_BACKGROUND=3\n  typeset -g POWERLEVEL9K_DISK_USAGE_CRITICAL_FOREGROUND=7\n  typeset -g POWERLEVEL9K_DISK_USAGE_CRITICAL_BACKGROUND=1\n  # Thresholds for different levels of disk usage (percentage points).\n  typeset -g POWERLEVEL9K_DISK_USAGE_WARNING_LEVEL=90\n  typeset -g POWERLEVEL9K_DISK_USAGE_CRITICAL_LEVEL=95\n  # If set to true, hide disk usage when below $POWERLEVEL9K_DISK_USAGE_WARNING_LEVEL percent.\n  typeset -g POWERLEVEL9K_DISK_USAGE_ONLY_WARNING=false\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_DISK_USAGE_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ###########[ vi_mode: vi mode (you don\'t need this if you\'ve enabled prompt_char) ]###########\n  # Foreground color.\n  typeset -g POWERLEVEL9K_VI_MODE_FOREGROUND=0\n  # Text and color for normal (a.k.a. command) vi mode.\n  typeset -g POWERLEVEL9K_VI_COMMAND_MODE_STRING=NORMAL\n  typeset -g POWERLEVEL9K_VI_MODE_NORMAL_BACKGROUND=2\n  # Text and color for visual vi mode.\n  typeset -g POWERLEVEL9K_VI_VISUAL_MODE_STRING=VISUAL\n  typeset -g POWERLEVEL9K_VI_MODE_VISUAL_BACKGROUND=4\n  # Text and color for overtype (a.k.a. overwrite and replace) vi mode.\n  typeset -g POWERLEVEL9K_VI_OVERWRITE_MODE_STRING=OVERTYPE\n  typeset -g POWERLEVEL9K_VI_MODE_OVERWRITE_BACKGROUND=3\n  # Text and color for insert vi mode.\n  typeset -g POWERLEVEL9K_VI_INSERT_MODE_STRING=\n  typeset -g POWERLEVEL9K_VI_MODE_INSERT_FOREGROUND=8\n\n  ######################################[ ram: free RAM ]#######################################\n  # RAM color.\n  typeset -g POWERLEVEL9K_RAM_FOREGROUND=0\n  typeset -g POWERLEVEL9K_RAM_BACKGROUND=3\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_RAM_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  #####################################[ swap: used swap ]######################################\n  # Swap color.\n  typeset -g POWERLEVEL9K_SWAP_FOREGROUND=0\n  typeset -g POWERLEVEL9K_SWAP_BACKGROUND=3\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_SWAP_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ######################################[ load: CPU load ]######################################\n  # Show average CPU load over this many last minutes. Valid values are 1, 5 and 15.\n  typeset -g POWERLEVEL9K_LOAD_WHICH=5\n  # Load color when load is under 50%.\n  typeset -g POWERLEVEL9K_LOAD_NORMAL_FOREGROUND=0\n  typeset -g POWERLEVEL9K_LOAD_NORMAL_BACKGROUND=2\n  # Load color when load is between 50% and 70%.\n  typeset -g POWERLEVEL9K_LOAD_WARNING_FOREGROUND=0\n  typeset -g POWERLEVEL9K_LOAD_WARNING_BACKGROUND=3\n  # Load color when load is over 70%.\n  typeset -g POWERLEVEL9K_LOAD_CRITICAL_FOREGROUND=0\n  typeset -g POWERLEVEL9K_LOAD_CRITICAL_BACKGROUND=1\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_LOAD_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ################[ todo: todo items (https://github.com/todotxt/todo.txt-cli) ]################\n  # Todo color.\n  typeset -g POWERLEVEL9K_TODO_FOREGROUND=0\n  typeset -g POWERLEVEL9K_TODO_BACKGROUND=8\n  # Hide todo when the total number of tasks is zero.\n  typeset -g POWERLEVEL9K_TODO_HIDE_ZERO_TOTAL=true\n  # Hide todo when the number of tasks after filtering is zero.\n  typeset -g POWERLEVEL9K_TODO_HIDE_ZERO_FILTERED=false\n\n  # Todo format. The following parameters are available within the expansion.\n  #\n  # - P9K_TODO_TOTAL_TASK_COUNT     The total number of tasks.\n  # - P9K_TODO_FILTERED_TASK_COUNT  The number of tasks after filtering.\n  #\n  # These variables correspond to the last line of the output of `todo.sh -p ls`:\n  #\n  #   TODO: 24 of 42 tasks shown\n  #\n  # Here 24 is P9K_TODO_FILTERED_TASK_COUNT and 42 is P9K_TODO_TOTAL_TASK_COUNT.\n  #\n  # typeset -g POWERLEVEL9K_TODO_CONTENT_EXPANSION=\'$P9K_TODO_FILTERED_TASK_COUNT\'\n\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_TODO_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ###########[ timewarrior: timewarrior tracking status (https://timewarrior.net/) ]############\n  # Timewarrior color.\n  typeset -g POWERLEVEL9K_TIMEWARRIOR_FOREGROUND=255\n  typeset -g POWERLEVEL9K_TIMEWARRIOR_BACKGROUND=8\n\n  # If the tracked task is longer than 24 characters, truncate and append \"\u{2026}\".\n  # Tip: To always display tasks without truncation, delete the following parameter.\n  # Tip: To hide task names and display just the icon when time tracking is enabled, set the\n  # value of the following parameter to \"\".\n  typeset -g POWERLEVEL9K_TIMEWARRIOR_CONTENT_EXPANSION=\'${P9K_CONTENT:0:24}${${P9K_CONTENT:24}:+\u{2026}}\'\n\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_TIMEWARRIOR_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ##############[ taskwarrior: taskwarrior task count (https://taskwarrior.org/) ]##############\n  # Taskwarrior color.\n  typeset -g POWERLEVEL9K_TASKWARRIOR_FOREGROUND=0\n  typeset -g POWERLEVEL9K_TASKWARRIOR_BACKGROUND=6\n\n  # Taskwarrior segment format. The following parameters are available within the expansion.\n  #\n  # - P9K_TASKWARRIOR_PENDING_COUNT   The number of pending tasks: `task +PENDING count`.\n  # - P9K_TASKWARRIOR_OVERDUE_COUNT   The number of overdue tasks: `task +OVERDUE count`.\n  #\n  # Zero values are represented as empty parameters.\n  #\n  # The default format:\n  #\n  #   \'${P9K_TASKWARRIOR_OVERDUE_COUNT:+\"!$P9K_TASKWARRIOR_OVERDUE_COUNT/\"}$P9K_TASKWARRIOR_PENDING_COUNT\'\n  #\n  # typeset -g POWERLEVEL9K_TASKWARRIOR_CONTENT_EXPANSION=\'$P9K_TASKWARRIOR_PENDING_COUNT\'\n\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_TASKWARRIOR_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ##################################[ context: user@hostname ]##################################\n  # Context color when running with privileges.\n  typeset -g POWERLEVEL9K_CONTEXT_ROOT_FOREGROUND=1\n  typeset -g POWERLEVEL9K_CONTEXT_ROOT_BACKGROUND=0\n  # Context color in SSH without privileges.\n  typeset -g POWERLEVEL9K_CONTEXT_{REMOTE,REMOTE_SUDO}_FOREGROUND=3\n  typeset -g POWERLEVEL9K_CONTEXT_{REMOTE,REMOTE_SUDO}_BACKGROUND=0\n  # Default context color (no privileges, no SSH).\n  typeset -g POWERLEVEL9K_CONTEXT_FOREGROUND=3\n  typeset -g POWERLEVEL9K_CONTEXT_BACKGROUND=0\n\n  # Context format when running with privileges: user@hostname.\n  typeset -g POWERLEVEL9K_CONTEXT_ROOT_TEMPLATE=\'%n@%m\'\n  # Context format when in SSH without privileges: user@hostname.\n  typeset -g POWERLEVEL9K_CONTEXT_{REMOTE,REMOTE_SUDO}_TEMPLATE=\'%n@%m\'\n  # Default context format (no privileges, no SSH): user@hostname.\n  typeset -g POWERLEVEL9K_CONTEXT_TEMPLATE=\'%n@%m\'\n\n  # Don\'t show context unless running with privileges or in SSH.\n  # Tip: Remove the next line to always show context.\n  typeset -g POWERLEVEL9K_CONTEXT_{DEFAULT,SUDO}_{CONTENT,VISUAL_IDENTIFIER}_EXPANSION=\n\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_CONTEXT_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # Custom prefix.\n  typeset -g POWERLEVEL9K_CONTEXT_PREFIX=\'with \'\n\n  ###[ virtualenv: python virtual environment (https://docs.python.org/3/library/venv.html) ]###\n  # Python virtual environment color.\n  typeset -g POWERLEVEL9K_VIRTUALENV_FOREGROUND=0\n  typeset -g POWERLEVEL9K_VIRTUALENV_BACKGROUND=4\n  # Don\'t show Python version next to the virtual environment name.\n  typeset -g POWERLEVEL9K_VIRTUALENV_SHOW_PYTHON_VERSION=false\n  # If set to \"false\", won\'t show virtualenv if pyenv is already shown.\n  # If set to \"if-different\", won\'t show virtualenv if it\'s the same as pyenv.\n  typeset -g POWERLEVEL9K_VIRTUALENV_SHOW_WITH_PYENV=false\n  # Separate environment name from Python version only with a space.\n  typeset -g POWERLEVEL9K_VIRTUALENV_{LEFT,RIGHT}_DELIMITER=\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_VIRTUALENV_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  #####################[ anaconda: conda environment (https://conda.io/) ]######################\n  # Anaconda environment color.\n  typeset -g POWERLEVEL9K_ANACONDA_FOREGROUND=0\n  typeset -g POWERLEVEL9K_ANACONDA_BACKGROUND=4\n\n  # Anaconda segment format. The following parameters are available within the expansion.\n  #\n  # - CONDA_PREFIX                 Absolute path to the active Anaconda/Miniconda environment.\n  # - CONDA_DEFAULT_ENV            Name of the active Anaconda/Miniconda environment.\n  # - CONDA_PROMPT_MODIFIER        Configurable prompt modifier (see below).\n  # - P9K_ANACONDA_PYTHON_VERSION  Current python version (python --version).\n  #\n  # CONDA_PROMPT_MODIFIER can be configured with the following command:\n  #\n  #   conda config --set env_prompt \'({default_env}) \'\n  #\n  # The last argument is a Python format string that can use the following variables:\n  #\n  # - prefix       The same as CONDA_PREFIX.\n  # - default_env  The same as CONDA_DEFAULT_ENV.\n  # - name         The last segment of CONDA_PREFIX.\n  # - stacked_env  Comma-separated list of names in the environment stack. The first element is\n  #                always the same as default_env.\n  #\n  # Note: \'({default_env}) \' is the default value of env_prompt.\n  #\n  # The default value of POWERLEVEL9K_ANACONDA_CONTENT_EXPANSION expands to $CONDA_PROMPT_MODIFIER\n  # without the surrounding parentheses, or to the last path component of CONDA_PREFIX if the former\n  # is empty.\n  typeset -g POWERLEVEL9K_ANACONDA_CONTENT_EXPANSION=\'${${${${CONDA_PROMPT_MODIFIER#\\(}% }%\\)}:-${CONDA_PREFIX:t}}\'\n\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_ANACONDA_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ################[ pyenv: python environment (https://github.com/pyenv/pyenv) ]################\n  # Pyenv color.\n  typeset -g POWERLEVEL9K_PYENV_FOREGROUND=0\n  typeset -g POWERLEVEL9K_PYENV_BACKGROUND=4\n  # Hide python version if it doesn\'t come from one of these sources.\n  typeset -g POWERLEVEL9K_PYENV_SOURCES=(shell local global)\n  # If set to false, hide python version if it\'s the same as global:\n  # $(pyenv version-name) == $(pyenv global).\n  typeset -g POWERLEVEL9K_PYENV_PROMPT_ALWAYS_SHOW=false\n  # If set to false, hide python version if it\'s equal to \"system\".\n  typeset -g POWERLEVEL9K_PYENV_SHOW_SYSTEM=true\n\n  # Pyenv segment format. The following parameters are available within the expansion.\n  #\n  # - P9K_CONTENT                Current pyenv environment (pyenv version-name).\n  # - P9K_PYENV_PYTHON_VERSION   Current python version (python --version).\n  #\n  # The default format has the following logic:\n  #\n  # 1. Display just \"$P9K_CONTENT\" if it\'s equal to \"$P9K_PYENV_PYTHON_VERSION\" or\n  #    starts with \"$P9K_PYENV_PYTHON_VERSION/\".\n  # 2. Otherwise display \"$P9K_CONTENT $P9K_PYENV_PYTHON_VERSION\".\n  typeset -g POWERLEVEL9K_PYENV_CONTENT_EXPANSION=\'${P9K_CONTENT}${${P9K_CONTENT:#$P9K_PYENV_PYTHON_VERSION(|/*)}:+ $P9K_PYENV_PYTHON_VERSION}\'\n\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_PYENV_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ################[ goenv: go environment (https://github.com/syndbg/goenv) ]################\n  # Goenv color.\n  typeset -g POWERLEVEL9K_GOENV_FOREGROUND=0\n  typeset -g POWERLEVEL9K_GOENV_BACKGROUND=4\n  # Hide go version if it doesn\'t come from one of these sources.\n  typeset -g POWERLEVEL9K_GOENV_SOURCES=(shell local global)\n  # If set to false, hide go version if it\'s the same as global:\n  # $(goenv version-name) == $(goenv global).\n  typeset -g POWERLEVEL9K_GOENV_PROMPT_ALWAYS_SHOW=false\n  # If set to false, hide go version if it\'s equal to \"system\".\n  typeset -g POWERLEVEL9K_GOENV_SHOW_SYSTEM=true\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_GOENV_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ##########[ nodenv: node.js version from nodenv (https://github.com/nodenv/nodenv) ]##########\n  # Nodenv color.\n  typeset -g POWERLEVEL9K_NODENV_FOREGROUND=2\n  typeset -g POWERLEVEL9K_NODENV_BACKGROUND=0\n  # Hide node version if it doesn\'t come from one of these sources.\n  typeset -g POWERLEVEL9K_NODENV_SOURCES=(shell local global)\n  # If set to false, hide node version if it\'s the same as global:\n  # $(nodenv version-name) == $(nodenv global).\n  typeset -g POWERLEVEL9K_NODENV_PROMPT_ALWAYS_SHOW=false\n  # If set to false, hide node version if it\'s equal to \"system\".\n  typeset -g POWERLEVEL9K_NODENV_SHOW_SYSTEM=true\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_NODENV_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ##############[ nvm: node.js version from nvm (https://github.com/nvm-sh/nvm) ]###############\n  # Nvm color.\n  typeset -g POWERLEVEL9K_NVM_FOREGROUND=0\n  typeset -g POWERLEVEL9K_NVM_BACKGROUND=5\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_NVM_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ############[ nodeenv: node.js environment (https://github.com/ekalinin/nodeenv) ]############\n  # Nodeenv color.\n  typeset -g POWERLEVEL9K_NODEENV_FOREGROUND=2\n  typeset -g POWERLEVEL9K_NODEENV_BACKGROUND=0\n  # Don\'t show Node version next to the environment name.\n  typeset -g POWERLEVEL9K_NODEENV_SHOW_NODE_VERSION=false\n  # Separate environment name from Node version only with a space.\n  typeset -g POWERLEVEL9K_NODEENV_{LEFT,RIGHT}_DELIMITER=\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_NODEENV_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ##############################[ node_version: node.js version ]###############################\n  # Node version color.\n  typeset -g POWERLEVEL9K_NODE_VERSION_FOREGROUND=7\n  typeset -g POWERLEVEL9K_NODE_VERSION_BACKGROUND=2\n  # Show node version only when in a directory tree containing package.json.\n  typeset -g POWERLEVEL9K_NODE_VERSION_PROJECT_ONLY=true\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_NODE_VERSION_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  #######################[ go_version: go version (https://golang.org) ]########################\n  # Go version color.\n  typeset -g POWERLEVEL9K_GO_VERSION_FOREGROUND=255\n  typeset -g POWERLEVEL9K_GO_VERSION_BACKGROUND=2\n  # Show go version only when in a go project subdirectory.\n  typeset -g POWERLEVEL9K_GO_VERSION_PROJECT_ONLY=true\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_GO_VERSION_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  #################[ rust_version: rustc version (https://www.rust-lang.org) ]##################\n  # Rust version color.\n  typeset -g POWERLEVEL9K_RUST_VERSION_FOREGROUND=0\n  typeset -g POWERLEVEL9K_RUST_VERSION_BACKGROUND=208\n  # Show rust version only when in a rust project subdirectory.\n  typeset -g POWERLEVEL9K_RUST_VERSION_PROJECT_ONLY=true\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_RUST_VERSION_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ###############[ dotnet_version: .NET version (https://dotnet.microsoft.com) ]################\n  # .NET version color.\n  typeset -g POWERLEVEL9K_DOTNET_VERSION_FOREGROUND=7\n  typeset -g POWERLEVEL9K_DOTNET_VERSION_BACKGROUND=5\n  # Show .NET version only when in a .NET project subdirectory.\n  typeset -g POWERLEVEL9K_DOTNET_VERSION_PROJECT_ONLY=true\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_DOTNET_VERSION_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  #####################[ php_version: php version (https://www.php.net/) ]######################\n  # PHP version color.\n  typeset -g POWERLEVEL9K_PHP_VERSION_FOREGROUND=0\n  typeset -g POWERLEVEL9K_PHP_VERSION_BACKGROUND=5\n  # Show PHP version only when in a PHP project subdirectory.\n  typeset -g POWERLEVEL9K_PHP_VERSION_PROJECT_ONLY=true\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_PHP_VERSION_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ##########[ laravel_version: laravel php framework version (https://laravel.com/) ]###########\n  # Laravel version color.\n  typeset -g POWERLEVEL9K_LARAVEL_VERSION_FOREGROUND=1\n  typeset -g POWERLEVEL9K_LARAVEL_VERSION_BACKGROUND=7\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_LARAVEL_VERSION_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  #############[ rbenv: ruby version from rbenv (https://github.com/rbenv/rbenv) ]##############\n  # Rbenv color.\n  typeset -g POWERLEVEL9K_RBENV_FOREGROUND=0\n  typeset -g POWERLEVEL9K_RBENV_BACKGROUND=1\n  # Hide ruby version if it doesn\'t come from one of these sources.\n  typeset -g POWERLEVEL9K_RBENV_SOURCES=(shell local global)\n  # If set to false, hide ruby version if it\'s the same as global:\n  # $(rbenv version-name) == $(rbenv global).\n  typeset -g POWERLEVEL9K_RBENV_PROMPT_ALWAYS_SHOW=false\n  # If set to false, hide ruby version if it\'s equal to \"system\".\n  typeset -g POWERLEVEL9K_RBENV_SHOW_SYSTEM=true\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_RBENV_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ####################[ java_version: java version (https://www.java.com/) ]####################\n  # Java version color.\n  typeset -g POWERLEVEL9K_JAVA_VERSION_FOREGROUND=1\n  typeset -g POWERLEVEL9K_JAVA_VERSION_BACKGROUND=7\n  # Show java version only when in a java project subdirectory.\n  typeset -g POWERLEVEL9K_JAVA_VERSION_PROJECT_ONLY=true\n  # Show brief version.\n  typeset -g POWERLEVEL9K_JAVA_VERSION_FULL=false\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_JAVA_VERSION_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ###[ package: name@version from package.json (https://docs.npmjs.com/files/package.json) ]####\n  # Package color.\n  typeset -g POWERLEVEL9K_PACKAGE_FOREGROUND=0\n  typeset -g POWERLEVEL9K_PACKAGE_BACKGROUND=6\n\n  # Package format. The following parameters are available within the expansion.\n  #\n  # - P9K_PACKAGE_NAME     The value of `name` field in package.json.\n  # - P9K_PACKAGE_VERSION  The value of `version` field in package.json.\n  #\n  # typeset -g POWERLEVEL9K_PACKAGE_CONTENT_EXPANSION=\'${P9K_PACKAGE_NAME//\\%/%%}@${P9K_PACKAGE_VERSION//\\%/%%}\'\n\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_PACKAGE_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  #######################[ rvm: ruby version from rvm (https://rvm.io) ]########################\n  # Rvm color.\n  typeset -g POWERLEVEL9K_RVM_FOREGROUND=0\n  typeset -g POWERLEVEL9K_RVM_BACKGROUND=240\n  # Don\'t show @gemset at the end.\n  typeset -g POWERLEVEL9K_RVM_SHOW_GEMSET=false\n  # Don\'t show ruby- at the front.\n  typeset -g POWERLEVEL9K_RVM_SHOW_PREFIX=false\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_RVM_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ###########[ fvm: flutter version management (https://github.com/leoafarias/fvm) ]############\n  # Fvm color.\n  typeset -g POWERLEVEL9K_FVM_FOREGROUND=0\n  typeset -g POWERLEVEL9K_FVM_BACKGROUND=4\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_FVM_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ##########[ luaenv: lua version from luaenv (https://github.com/cehoffman/luaenv) ]###########\n  # Lua color.\n  typeset -g POWERLEVEL9K_LUAENV_FOREGROUND=0\n  typeset -g POWERLEVEL9K_LUAENV_BACKGROUND=4\n  # Hide lua version if it doesn\'t come from one of these sources.\n  typeset -g POWERLEVEL9K_LUAENV_SOURCES=(shell local global)\n  # If set to false, hide lua version if it\'s the same as global:\n  # $(luaenv version-name) == $(luaenv global).\n  typeset -g POWERLEVEL9K_LUAENV_PROMPT_ALWAYS_SHOW=false\n  # If set to false, hide lua version if it\'s equal to \"system\".\n  typeset -g POWERLEVEL9K_LUAENV_SHOW_SYSTEM=true\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_LUAENV_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ###############[ jenv: java version from jenv (https://github.com/jenv/jenv) ]################\n  # Java color.\n  typeset -g POWERLEVEL9K_JENV_FOREGROUND=1\n  typeset -g POWERLEVEL9K_JENV_BACKGROUND=7\n  # Hide java version if it doesn\'t come from one of these sources.\n  typeset -g POWERLEVEL9K_JENV_SOURCES=(shell local global)\n  # If set to false, hide java version if it\'s the same as global:\n  # $(jenv version-name) == $(jenv global).\n  typeset -g POWERLEVEL9K_JENV_PROMPT_ALWAYS_SHOW=false\n  # If set to false, hide java version if it\'s equal to \"system\".\n  typeset -g POWERLEVEL9K_JENV_SHOW_SYSTEM=true\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_JENV_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ###########[ plenv: perl version from plenv (https://github.com/tokuhirom/plenv) ]############\n  # Perl color.\n  typeset -g POWERLEVEL9K_PLENV_FOREGROUND=0\n  typeset -g POWERLEVEL9K_PLENV_BACKGROUND=4\n  # Hide perl version if it doesn\'t come from one of these sources.\n  typeset -g POWERLEVEL9K_PLENV_SOURCES=(shell local global)\n  # If set to false, hide perl version if it\'s the same as global:\n  # $(plenv version-name) == $(plenv global).\n  typeset -g POWERLEVEL9K_PLENV_PROMPT_ALWAYS_SHOW=false\n  # If set to false, hide perl version if it\'s equal to \"system\".\n  typeset -g POWERLEVEL9K_PLENV_SHOW_SYSTEM=true\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_PLENV_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ############[ phpenv: php version from phpenv (https://github.com/phpenv/phpenv) ]############\n  # PHP color.\n  typeset -g POWERLEVEL9K_PHPENV_FOREGROUND=0\n  typeset -g POWERLEVEL9K_PHPENV_BACKGROUND=5\n  # Hide php version if it doesn\'t come from one of these sources.\n  typeset -g POWERLEVEL9K_PHPENV_SOURCES=(shell local global)\n  # If set to false, hide php version if it\'s the same as global:\n  # $(phpenv version-name) == $(phpenv global).\n  typeset -g POWERLEVEL9K_PHPENV_PROMPT_ALWAYS_SHOW=false\n  # If set to false, hide PHP version if it\'s equal to \"system\".\n  typeset -g POWERLEVEL9K_PHPENV_SHOW_SYSTEM=true\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_PHPENV_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  #######[ scalaenv: scala version from scalaenv (https://github.com/scalaenv/scalaenv) ]#######\n  # Scala color.\n  typeset -g POWERLEVEL9K_SCALAENV_FOREGROUND=0\n  typeset -g POWERLEVEL9K_SCALAENV_BACKGROUND=1\n  # Hide scala version if it doesn\'t come from one of these sources.\n  typeset -g POWERLEVEL9K_SCALAENV_SOURCES=(shell local global)\n  # If set to false, hide scala version if it\'s the same as global:\n  # $(scalaenv version-name) == $(scalaenv global).\n  typeset -g POWERLEVEL9K_SCALAENV_PROMPT_ALWAYS_SHOW=false\n  # If set to false, hide scala version if it\'s equal to \"system\".\n  typeset -g POWERLEVEL9K_SCALAENV_SHOW_SYSTEM=true\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_SCALAENV_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ##########[ haskell_stack: haskell version from stack (https://haskellstack.org/) ]###########\n  # Haskell color.\n  typeset -g POWERLEVEL9K_HASKELL_STACK_FOREGROUND=0\n  typeset -g POWERLEVEL9K_HASKELL_STACK_BACKGROUND=3\n\n  # Hide haskell version if it doesn\'t come from one of these sources.\n  #\n  #   shell:  version is set by STACK_YAML\n  #   local:  version is set by stack.yaml up the directory tree\n  #   global: version is set by the implicit global project (~/.stack/global-project/stack.yaml)\n  typeset -g POWERLEVEL9K_HASKELL_STACK_SOURCES=(shell local)\n  # If set to false, hide haskell version if it\'s the same as in the implicit global project.\n  typeset -g POWERLEVEL9K_HASKELL_STACK_ALWAYS_SHOW=true\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_HASKELL_STACK_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ################[ terraform: terraform workspace (https://www.terraform.io) ]#################\n  # Don\'t show terraform workspace if it\'s literally \"default\".\n  typeset -g POWERLEVEL9K_TERRAFORM_SHOW_DEFAULT=false\n  # POWERLEVEL9K_TERRAFORM_CLASSES is an array with even number of elements. The first element\n  # in each pair defines a pattern against which the current terraform workspace gets matched.\n  # More specifically, it\'s P9K_CONTENT prior to the application of context expansion (see below)\n  # that gets matched. If you unset all POWERLEVEL9K_TERRAFORM_*CONTENT_EXPANSION parameters,\n  # you\'ll see this value in your prompt. The second element of each pair in\n  # POWERLEVEL9K_TERRAFORM_CLASSES defines the workspace class. Patterns are tried in order. The\n  # first match wins.\n  #\n  # For example, given these settings:\n  #\n  #   typeset -g POWERLEVEL9K_TERRAFORM_CLASSES=(\n  #     \'*prod*\'  PROD\n  #     \'*test*\'  TEST\n  #     \'*\'       OTHER)\n  #\n  # If your current terraform workspace is \"project_test\", its class is TEST because \"project_test\"\n  # doesn\'t match the pattern \'*prod*\' but does match \'*test*\'.\n  #\n  # You can define different colors, icons and content expansions for different classes:\n  #\n  #   typeset -g POWERLEVEL9K_TERRAFORM_TEST_FOREGROUND=2\n  #   typeset -g POWERLEVEL9K_TERRAFORM_TEST_BACKGROUND=0\n  #   typeset -g POWERLEVEL9K_TERRAFORM_TEST_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  #   typeset -g POWERLEVEL9K_TERRAFORM_TEST_CONTENT_EXPANSION=\'> ${P9K_CONTENT} <\'\n  typeset -g POWERLEVEL9K_TERRAFORM_CLASSES=(\n      # \'*prod*\'  PROD    # These values are examples that are unlikely\n      # \'*test*\'  TEST    # to match your needs. Customize them as needed.\n      \'*\'         OTHER)\n  typeset -g POWERLEVEL9K_TERRAFORM_OTHER_FOREGROUND=4\n  typeset -g POWERLEVEL9K_TERRAFORM_OTHER_BACKGROUND=0\n  # typeset -g POWERLEVEL9K_TERRAFORM_OTHER_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  #############[ terraform_version: terraform version (https://www.terraform.io) ]##############\n  # Terraform version color.\n  typeset -g POWERLEVEL9K_TERRAFORM_VERSION_FOREGROUND=4\n  typeset -g POWERLEVEL9K_TERRAFORM_VERSION_BACKGROUND=0\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_TERRAFORM_VERSION_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ################[ terraform_version: It shows active terraform version (https://www.terraform.io) ]#################\n  typeset -g POWERLEVEL9K_TERRAFORM_VERSION_SHOW_ON_COMMAND=\'terraform|tf\'\n\n  #############[ kubecontext: current kubernetes context (https://kubernetes.io/) ]#############\n  # Show kubecontext only when the the command you are typing invokes one of these tools.\n  # Tip: Remove the next line to always show kubecontext.\n  typeset -g POWERLEVEL9K_KUBECONTEXT_SHOW_ON_COMMAND=\'kubectl|helm|kubens|kubectx|oc|istioctl|kogito|k9s|helmfile|flux|fluxctl|stern\'\n\n  # Kubernetes context classes for the purpose of using different colors, icons and expansions with\n  # different contexts.\n  #\n  # POWERLEVEL9K_KUBECONTEXT_CLASSES is an array with even number of elements. The first element\n  # in each pair defines a pattern against which the current kubernetes context gets matched.\n  # More specifically, it\'s P9K_CONTENT prior to the application of context expansion (see below)\n  # that gets matched. If you unset all POWERLEVEL9K_KUBECONTEXT_*CONTENT_EXPANSION parameters,\n  # you\'ll see this value in your prompt. The second element of each pair in\n  # POWERLEVEL9K_KUBECONTEXT_CLASSES defines the context class. Patterns are tried in order. The\n  # first match wins.\n  #\n  # For example, given these settings:\n  #\n  #   typeset -g POWERLEVEL9K_KUBECONTEXT_CLASSES=(\n  #     \'*prod*\'  PROD\n  #     \'*test*\'  TEST\n  #     \'*\'       DEFAULT)\n  #\n  # If your current kubernetes context is \"deathray-testing/default\", its class is TEST\n  # because \"deathray-testing/default\" doesn\'t match the pattern \'*prod*\' but does match \'*test*\'.\n  #\n  # You can define different colors, icons and content expansions for different classes:\n  #\n  #   typeset -g POWERLEVEL9K_KUBECONTEXT_TEST_FOREGROUND=0\n  #   typeset -g POWERLEVEL9K_KUBECONTEXT_TEST_BACKGROUND=2\n  #   typeset -g POWERLEVEL9K_KUBECONTEXT_TEST_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  #   typeset -g POWERLEVEL9K_KUBECONTEXT_TEST_CONTENT_EXPANSION=\'> ${P9K_CONTENT} <\'\n  typeset -g POWERLEVEL9K_KUBECONTEXT_CLASSES=(\n      # \'*prod*\'  PROD    # These values are examples that are unlikely\n      # \'*test*\'  TEST    # to match your needs. Customize them as needed.\n      \'*\'       DEFAULT)\n  typeset -g POWERLEVEL9K_KUBECONTEXT_DEFAULT_FOREGROUND=7\n  typeset -g POWERLEVEL9K_KUBECONTEXT_DEFAULT_BACKGROUND=5\n  typeset -g POWERLEVEL9K_KUBECONTEXT_DEFAULT_VISUAL_IDENTIFIER_EXPANSION=\'\u{25cb}\'\n\n  # Use POWERLEVEL9K_KUBECONTEXT_CONTENT_EXPANSION to specify the content displayed by kubecontext\n  # segment. Parameter expansions are very flexible and fast, too. See reference:\n  # http://zsh.sourceforge.net/Doc/Release/Expansion.html#Parameter-Expansion.\n  #\n  # Within the expansion the following parameters are always available:\n  #\n  # - P9K_CONTENT                The content that would\'ve been displayed if there was no content\n  #                              expansion defined.\n  # - P9K_KUBECONTEXT_NAME       The current context\'s name. Corresponds to column NAME in the\n  #                              output of `kubectl config get-contexts`.\n  # - P9K_KUBECONTEXT_CLUSTER    The current context\'s cluster. Corresponds to column CLUSTER in the\n  #                              output of `kubectl config get-contexts`.\n  # - P9K_KUBECONTEXT_NAMESPACE  The current context\'s namespace. Corresponds to column NAMESPACE\n  #                              in the output of `kubectl config get-contexts`. If there is no\n  #                              namespace, the parameter is set to \"default\".\n  # - P9K_KUBECONTEXT_USER       The current context\'s user. Corresponds to column AUTHINFO in the\n  #                              output of `kubectl config get-contexts`.\n  #\n  # If the context points to Google Kubernetes Engine (GKE) or Elastic Kubernetes Service (EKS),\n  # the following extra parameters are available:\n  #\n  # - P9K_KUBECONTEXT_CLOUD_NAME     Either \"gke\" or \"eks\".\n  # - P9K_KUBECONTEXT_CLOUD_ACCOUNT  Account/project ID.\n  # - P9K_KUBECONTEXT_CLOUD_ZONE     Availability zone.\n  # - P9K_KUBECONTEXT_CLOUD_CLUSTER  Cluster.\n  #\n  # P9K_KUBECONTEXT_CLOUD_* parameters are derived from P9K_KUBECONTEXT_CLUSTER. For example,\n  # if P9K_KUBECONTEXT_CLUSTER is \"gke_my-account_us-east1-a_my-cluster-01\":\n  #\n  #   - P9K_KUBECONTEXT_CLOUD_NAME=gke\n  #   - P9K_KUBECONTEXT_CLOUD_ACCOUNT=my-account\n  #   - P9K_KUBECONTEXT_CLOUD_ZONE=us-east1-a\n  #   - P9K_KUBECONTEXT_CLOUD_CLUSTER=my-cluster-01\n  #\n  # If P9K_KUBECONTEXT_CLUSTER is \"arn:aws:eks:us-east-1:123456789012:cluster/my-cluster-01\":\n  #\n  #   - P9K_KUBECONTEXT_CLOUD_NAME=eks\n  #   - P9K_KUBECONTEXT_CLOUD_ACCOUNT=123456789012\n  #   - P9K_KUBECONTEXT_CLOUD_ZONE=us-east-1\n  #   - P9K_KUBECONTEXT_CLOUD_CLUSTER=my-cluster-01\n  typeset -g POWERLEVEL9K_KUBECONTEXT_DEFAULT_CONTENT_EXPANSION=\n  # Show P9K_KUBECONTEXT_CLOUD_CLUSTER if it\'s not empty and fall back to P9K_KUBECONTEXT_NAME.\n  POWERLEVEL9K_KUBECONTEXT_DEFAULT_CONTENT_EXPANSION+=\'${P9K_KUBECONTEXT_CLOUD_CLUSTER:-${P9K_KUBECONTEXT_NAME}}\'\n  # Append the current context\'s namespace if it\'s not \"default\".\n  POWERLEVEL9K_KUBECONTEXT_DEFAULT_CONTENT_EXPANSION+=\'${${:-/$P9K_KUBECONTEXT_NAMESPACE}:#/default}\'\n\n  # Custom prefix.\n  typeset -g POWERLEVEL9K_KUBECONTEXT_PREFIX=\'at \'\n\n  #[ aws: aws profile (https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-profiles.html) ]#\n  # Show aws only when the the command you are typing invokes one of these tools.\n  # Tip: Remove the next line to always show aws.\n  typeset -g POWERLEVEL9K_AWS_SHOW_ON_COMMAND=\'aws|awless|terraform|pulumi|terragrunt\'\n\n  # POWERLEVEL9K_AWS_CLASSES is an array with even number of elements. The first element\n  # in each pair defines a pattern against which the current AWS profile gets matched.\n  # More specifically, it\'s P9K_CONTENT prior to the application of context expansion (see below)\n  # that gets matched. If you unset all POWERLEVEL9K_AWS_*CONTENT_EXPANSION parameters,\n  # you\'ll see this value in your prompt. The second element of each pair in\n  # POWERLEVEL9K_AWS_CLASSES defines the profile class. Patterns are tried in order. The\n  # first match wins.\n  #\n  # For example, given these settings:\n  #\n  #   typeset -g POWERLEVEL9K_AWS_CLASSES=(\n  #     \'*prod*\'  PROD\n  #     \'*test*\'  TEST\n  #     \'*\'       DEFAULT)\n  #\n  # If your current AWS profile is \"company_test\", its class is TEST\n  # because \"company_test\" doesn\'t match the pattern \'*prod*\' but does match \'*test*\'.\n  #\n  # You can define different colors, icons and content expansions for different classes:\n  #\n  #   typeset -g POWERLEVEL9K_AWS_TEST_FOREGROUND=28\n  #   typeset -g POWERLEVEL9K_AWS_TEST_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  #   typeset -g POWERLEVEL9K_AWS_TEST_CONTENT_EXPANSION=\'> ${P9K_CONTENT} <\'\n  typeset -g POWERLEVEL9K_AWS_CLASSES=(\n      # \'*prod*\'  PROD    # These values are examples that are unlikely\n      # \'*test*\'  TEST    # to match your needs. Customize them as needed.\n      \'*\'       DEFAULT)\n  typeset -g POWERLEVEL9K_AWS_DEFAULT_FOREGROUND=7\n  typeset -g POWERLEVEL9K_AWS_DEFAULT_BACKGROUND=1\n  # typeset -g POWERLEVEL9K_AWS_DEFAULT_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  # AWS segment format. The following parameters are available within the expansion.\n  #\n  # - P9K_AWS_PROFILE  The name of the current AWS profile.\n  # - P9K_AWS_REGION   The region associated with the current AWS profile.\n  typeset -g POWERLEVEL9K_AWS_CONTENT_EXPANSION=\'${P9K_AWS_PROFILE//\\%/%%}${P9K_AWS_REGION:+ ${P9K_AWS_REGION//\\%/%%}}\'\n\n  #[ aws_eb_env: aws elastic beanstalk environment (https://aws.amazon.com/elasticbeanstalk/) ]#\n  # AWS Elastic Beanstalk environment color.\n  typeset -g POWERLEVEL9K_AWS_EB_ENV_FOREGROUND=2\n  typeset -g POWERLEVEL9K_AWS_EB_ENV_BACKGROUND=0\n  # Custom icon.\n  typeset -g POWERLEVEL9K_AWS_EB_ENV_VISUAL_IDENTIFIER_EXPANSION=\'eb\'\n\n  ##########[ azure: azure account name (https://docs.microsoft.com/en-us/cli/azure) ]##########\n  # Show azure only when the the command you are typing invokes one of these tools.\n  # Tip: Remove the next line to always show azure.\n  typeset -g POWERLEVEL9K_AZURE_SHOW_ON_COMMAND=\'az|terraform|pulumi|terragrunt\'\n  # Azure account name color.\n  typeset -g POWERLEVEL9K_AZURE_FOREGROUND=7\n  typeset -g POWERLEVEL9K_AZURE_BACKGROUND=4\n  # Custom icon.\n  typeset -g POWERLEVEL9K_AZURE_VISUAL_IDENTIFIER_EXPANSION=\'az\'\n\n  ##########[ gcloud: google cloud account and project (https://cloud.google.com/) ]###########\n  # Show gcloud only when the the command you are typing invokes one of these tools.\n  # Tip: Remove the next line to always show gcloud.\n  typeset -g POWERLEVEL9K_GCLOUD_SHOW_ON_COMMAND=\'gcloud|gcs\'\n  # Google cloud color.\n  typeset -g POWERLEVEL9K_GCLOUD_FOREGROUND=7\n  typeset -g POWERLEVEL9K_GCLOUD_BACKGROUND=4\n\n  # Google cloud format. Change the value of POWERLEVEL9K_GCLOUD_PARTIAL_CONTENT_EXPANSION and/or\n  # POWERLEVEL9K_GCLOUD_COMPLETE_CONTENT_EXPANSION if the default is too verbose or not informative\n  # enough. You can use the following parameters in the expansions. Each of them corresponds to the\n  # output of `gcloud` tool.\n  #\n  #   Parameter                | Source\n  #   -------------------------|--------------------------------------------------------------------\n  #   P9K_GCLOUD_CONFIGURATION | gcloud config configurations list --format=\'value(name)\'\n  #   P9K_GCLOUD_ACCOUNT       | gcloud config get-value account\n  #   P9K_GCLOUD_PROJECT_ID    | gcloud config get-value project\n  #   P9K_GCLOUD_PROJECT_NAME  | gcloud projects describe $P9K_GCLOUD_PROJECT_ID --format=\'value(name)\'\n  #\n  # Note: ${VARIABLE//\\%/%%} expands to ${VARIABLE} with all occurrences of \'%\' replaced with \'%%\'.\n  #\n  # Obtaining project name requires sending a request to Google servers. This can take a long time\n  # and even fail. When project name is unknown, P9K_GCLOUD_PROJECT_NAME is not set and gcloud\n  # prompt segment is in state PARTIAL. When project name gets known, P9K_GCLOUD_PROJECT_NAME gets\n  # set and gcloud prompt segment transitions to state COMPLETE.\n  #\n  # You can customize the format, icon and colors of gcloud segment separately for states PARTIAL\n  # and COMPLETE. You can also hide gcloud in state PARTIAL by setting\n  # POWERLEVEL9K_GCLOUD_PARTIAL_VISUAL_IDENTIFIER_EXPANSION and\n  # POWERLEVEL9K_GCLOUD_PARTIAL_CONTENT_EXPANSION to empty.\n  typeset -g POWERLEVEL9K_GCLOUD_PARTIAL_CONTENT_EXPANSION=\'${P9K_GCLOUD_PROJECT_ID//\\%/%%}\'\n  typeset -g POWERLEVEL9K_GCLOUD_COMPLETE_CONTENT_EXPANSION=\'${P9K_GCLOUD_PROJECT_NAME//\\%/%%}\'\n\n  # Send a request to Google (by means of `gcloud projects describe ...`) to obtain project name\n  # this often. Negative value disables periodic polling. In this mode project name is retrieved\n  # only when the current configuration, account or project id changes.\n  typeset -g POWERLEVEL9K_GCLOUD_REFRESH_PROJECT_NAME_SECONDS=60\n\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_GCLOUD_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  #[ google_app_cred: google application credentials (https://cloud.google.com/docs/authentication/production) ]#\n  # Show google_app_cred only when the the command you are typing invokes one of these tools.\n  # Tip: Remove the next line to always show google_app_cred.\n  typeset -g POWERLEVEL9K_GOOGLE_APP_CRED_SHOW_ON_COMMAND=\'terraform|pulumi|terragrunt\'\n\n  # Google application credentials classes for the purpose of using different colors, icons and\n  # expansions with different credentials.\n  #\n  # POWERLEVEL9K_GOOGLE_APP_CRED_CLASSES is an array with even number of elements. The first\n  # element in each pair defines a pattern against which the current kubernetes context gets\n  # matched. More specifically, it\'s P9K_CONTENT prior to the application of context expansion\n  # (see below) that gets matched. If you unset all POWERLEVEL9K_GOOGLE_APP_CRED_*CONTENT_EXPANSION\n  # parameters, you\'ll see this value in your prompt. The second element of each pair in\n  # POWERLEVEL9K_GOOGLE_APP_CRED_CLASSES defines the context class. Patterns are tried in order.\n  # The first match wins.\n  #\n  # For example, given these settings:\n  #\n  #   typeset -g POWERLEVEL9K_GOOGLE_APP_CRED_CLASSES=(\n  #     \'*:*prod*:*\'  PROD\n  #     \'*:*test*:*\'  TEST\n  #     \'*\'           DEFAULT)\n  #\n  # If your current Google application credentials is \"service_account deathray-testing x@y.com\",\n  # its class is TEST because it doesn\'t match the pattern \'* *prod* *\' but does match \'* *test* *\'.\n  #\n  # You can define different colors, icons and content expansions for different classes:\n  #\n  #   typeset -g POWERLEVEL9K_GOOGLE_APP_CRED_TEST_FOREGROUND=28\n  #   typeset -g POWERLEVEL9K_GOOGLE_APP_CRED_TEST_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  #   typeset -g POWERLEVEL9K_GOOGLE_APP_CRED_TEST_CONTENT_EXPANSION=\'$P9K_GOOGLE_APP_CRED_PROJECT_ID\'\n  typeset -g POWERLEVEL9K_GOOGLE_APP_CRED_CLASSES=(\n      # \'*:*prod*:*\'  PROD    # These values are examples that are unlikely\n      # \'*:*test*:*\'  TEST    # to match your needs. Customize them as needed.\n      \'*\'             DEFAULT)\n  typeset -g POWERLEVEL9K_GOOGLE_APP_CRED_DEFAULT_FOREGROUND=7\n  typeset -g POWERLEVEL9K_GOOGLE_APP_CRED_DEFAULT_BACKGROUND=4\n  # typeset -g POWERLEVEL9K_GOOGLE_APP_CRED_DEFAULT_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  # Use POWERLEVEL9K_GOOGLE_APP_CRED_CONTENT_EXPANSION to specify the content displayed by\n  # google_app_cred segment. Parameter expansions are very flexible and fast, too. See reference:\n  # http://zsh.sourceforge.net/Doc/Release/Expansion.html#Parameter-Expansion.\n  #\n  # You can use the following parameters in the expansion. Each of them corresponds to one of the\n  # fields in the JSON file pointed to by GOOGLE_APPLICATION_CREDENTIALS.\n  #\n  #   Parameter                        | JSON key file field\n  #   ---------------------------------+---------------\n  #   P9K_GOOGLE_APP_CRED_TYPE         | type\n  #   P9K_GOOGLE_APP_CRED_PROJECT_ID   | project_id\n  #   P9K_GOOGLE_APP_CRED_CLIENT_EMAIL | client_email\n  #\n  # Note: ${VARIABLE//\\%/%%} expands to ${VARIABLE} with all occurrences of \'%\' replaced by \'%%\'.\n  typeset -g POWERLEVEL9K_GOOGLE_APP_CRED_DEFAULT_CONTENT_EXPANSION=\'${P9K_GOOGLE_APP_CRED_PROJECT_ID//\\%/%%}\'\n\n  ##############[ toolbox: toolbox name (https://github.com/containers/toolbox) ]###############\n  # Toolbox color.\n  typeset -g POWERLEVEL9K_TOOLBOX_FOREGROUND=0\n  typeset -g POWERLEVEL9K_TOOLBOX_BACKGROUND=3\n  # Don\'t display the name of the toolbox if it matches fedora-toolbox-*.\n  typeset -g POWERLEVEL9K_TOOLBOX_CONTENT_EXPANSION=\'${P9K_TOOLBOX_NAME:#fedora-toolbox-*}\'\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_TOOLBOX_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n  # Custom prefix.\n  typeset -g POWERLEVEL9K_TOOLBOX_PREFIX=\'in \'\n\n  ###############################[ public_ip: public IP address ]###############################\n  # Public IP color.\n  typeset -g POWERLEVEL9K_PUBLIC_IP_FOREGROUND=7\n  typeset -g POWERLEVEL9K_PUBLIC_IP_BACKGROUND=0\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_PUBLIC_IP_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ########################[ vpn_ip: virtual private network indicator ]#########################\n  # VPN IP color.\n  typeset -g POWERLEVEL9K_VPN_IP_FOREGROUND=0\n  typeset -g POWERLEVEL9K_VPN_IP_BACKGROUND=6\n  # When on VPN, show just an icon without the IP address.\n  # Tip: To display the private IP address when on VPN, remove the next line.\n  typeset -g POWERLEVEL9K_VPN_IP_CONTENT_EXPANSION=\n  # Regular expression for the VPN network interface. Run `ifconfig` or `ip -4 a show` while on VPN\n  # to see the name of the interface.\n  typeset -g POWERLEVEL9K_VPN_IP_INTERFACE=\'(gpd|wg|(.*tun)|tailscale)[0-9]*\'\n  # If set to true, show one segment per matching network interface. If set to false, show only\n  # one segment corresponding to the first matching network interface.\n  # Tip: If you set it to true, you\'ll probably want to unset POWERLEVEL9K_VPN_IP_CONTENT_EXPANSION.\n  typeset -g POWERLEVEL9K_VPN_IP_SHOW_ALL=false\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_VPN_IP_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ###########[ ip: ip address and bandwidth usage for a specified network interface ]###########\n  # IP color.\n  typeset -g POWERLEVEL9K_IP_BACKGROUND=4\n  typeset -g POWERLEVEL9K_IP_FOREGROUND=0\n  # The following parameters are accessible within the expansion:\n  #\n  #   Parameter             | Meaning\n  #   ----------------------+-------------------------------------------\n  #   P9K_IP_IP             | IP address\n  #   P9K_IP_INTERFACE      | network interface\n  #   P9K_IP_RX_BYTES       | total number of bytes received\n  #   P9K_IP_TX_BYTES       | total number of bytes sent\n  #   P9K_IP_RX_BYTES_DELTA | number of bytes received since last prompt\n  #   P9K_IP_TX_BYTES_DELTA | number of bytes sent since last prompt\n  #   P9K_IP_RX_RATE        | receive rate (since last prompt)\n  #   P9K_IP_TX_RATE        | send rate (since last prompt)\n  typeset -g POWERLEVEL9K_IP_CONTENT_EXPANSION=\'${P9K_IP_RX_RATE:+\u{21e3}$P9K_IP_RX_RATE }${P9K_IP_TX_RATE:+\u{21e1}$P9K_IP_TX_RATE }$P9K_IP_IP\'\n  # Show information for the first network interface whose name matches this regular expression.\n  # Run `ifconfig` or `ip -4 a show` to see the names of all network interfaces.\n  typeset -g POWERLEVEL9K_IP_INTERFACE=\'[ew].*\'\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_IP_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  #########################[ proxy: system-wide http/https/ftp proxy ]##########################\n  # Proxy color.\n  typeset -g POWERLEVEL9K_PROXY_FOREGROUND=4\n  typeset -g POWERLEVEL9K_PROXY_BACKGROUND=0\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_PROXY_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  ################################[ battery: internal battery ]#################################\n  # Show battery in red when it\'s below this level and not connected to power supply.\n  typeset -g POWERLEVEL9K_BATTERY_LOW_THRESHOLD=20\n  typeset -g POWERLEVEL9K_BATTERY_LOW_FOREGROUND=1\n  # Show battery in green when it\'s charging or fully charged.\n  typeset -g POWERLEVEL9K_BATTERY_{CHARGING,CHARGED}_FOREGROUND=2\n  # Show battery in yellow when it\'s discharging.\n  typeset -g POWERLEVEL9K_BATTERY_DISCONNECTED_FOREGROUND=3\n  # Battery pictograms going from low to high level of charge.\n  typeset -g POWERLEVEL9K_BATTERY_STAGES=(\'%K{232}\u{2581}\' \'%K{232}\u{2582}\' \'%K{232}\u{2583}\' \'%K{232}\u{2584}\' \'%K{232}\u{2585}\' \'%K{232}\u{2586}\' \'%K{232}\u{2587}\' \'%K{232}\u{2588}\')\n  # Don\'t show the remaining time to charge/discharge.\n  typeset -g POWERLEVEL9K_BATTERY_VERBOSE=false\n  typeset -g POWERLEVEL9K_BATTERY_BACKGROUND=0\n\n  #####################################[ wifi: wifi speed ]#####################################\n  # WiFi color.\n  typeset -g POWERLEVEL9K_WIFI_FOREGROUND=0\n  typeset -g POWERLEVEL9K_WIFI_BACKGROUND=4\n  # Custom icon.\n  # typeset -g POWERLEVEL9K_WIFI_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  # Use different colors and icons depending on signal strength ($P9K_WIFI_BARS).\n  #\n  #   # Wifi colors and icons for different signal strength levels (low to high).\n  #   typeset -g my_wifi_fg=(0 0 0 0 0)                                # <-- change these values\n  #   typeset -g my_wifi_icon=(\'WiFi\' \'WiFi\' \'WiFi\' \'WiFi\' \'WiFi\')     # <-- change these values\n  #\n  #   typeset -g POWERLEVEL9K_WIFI_CONTENT_EXPANSION=\'%F{${my_wifi_fg[P9K_WIFI_BARS+1]}}$P9K_WIFI_LAST_TX_RATE Mbps\'\n  #   typeset -g POWERLEVEL9K_WIFI_VISUAL_IDENTIFIER_EXPANSION=\'%F{${my_wifi_fg[P9K_WIFI_BARS+1]}}${my_wifi_icon[P9K_WIFI_BARS+1]}\'\n  #\n  # The following parameters are accessible within the expansions:\n  #\n  #   Parameter             | Meaning\n  #   ----------------------+---------------\n  #   P9K_WIFI_SSID         | service set identifier, a.k.a. network name\n  #   P9K_WIFI_LINK_AUTH    | authentication protocol such as \"wpa2-psk\" or \"none\"; empty if unknown\n  #   P9K_WIFI_LAST_TX_RATE | wireless transmit rate in megabits per second\n  #   P9K_WIFI_RSSI         | signal strength in dBm, from -120 to 0\n  #   P9K_WIFI_NOISE        | noise in dBm, from -120 to 0\n  #   P9K_WIFI_BARS         | signal strength in bars, from 0 to 4 (derived from P9K_WIFI_RSSI and P9K_WIFI_NOISE)\n\n  ####################################[ time: current time ]####################################\n  # Current time color.\n  typeset -g POWERLEVEL9K_TIME_FOREGROUND=0\n  typeset -g POWERLEVEL9K_TIME_BACKGROUND=7\n  # Format for the current time: 09:51:02. See `man 3 strftime`.\n  typeset -g POWERLEVEL9K_TIME_FORMAT=\'%D{%H:%M:%S}\'\n  # If set to true, time will update when you hit enter. This way prompts for the past\n  # commands will contain the start times of their commands as opposed to the default\n  # behavior where they contain the end times of their preceding commands.\n  typeset -g POWERLEVEL9K_TIME_UPDATE_ON_COMMAND=false\n  # Custom icon.\n  typeset -g POWERLEVEL9K_TIME_VISUAL_IDENTIFIER_EXPANSION=\n  # Custom prefix.\n  typeset -g POWERLEVEL9K_TIME_PREFIX=\'at \'\n\n  # Example of a user-defined prompt segment. Function prompt_example will be called on every\n  # prompt if `example` prompt segment is added to POWERLEVEL9K_LEFT_PROMPT_ELEMENTS or\n  # POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS. It displays an icon and yellow text on red background\n  # greeting the user.\n  #\n  # Type `p10k help segment` for documentation and a more sophisticated example.\n  function prompt_example() {\n    p10k segment -b 1 -f 3 -i \'\u{2b50}\' -t \'hello, %n\'\n  }\n\n  # User-defined prompt segments may optionally provide an instant_prompt_* function. Its job\n  # is to generate the prompt segment for display in instant prompt. See\n  # https://github.com/romkatv/powerlevel10k/blob/master/README.md#instant-prompt.\n  #\n  # Powerlevel10k will call instant_prompt_* at the same time as the regular prompt_* function\n  # and will record all `p10k segment` calls it makes. When displaying instant prompt, Powerlevel10k\n  # will replay these calls without actually calling instant_prompt_*. It is imperative that\n  # instant_prompt_* always makes the same `p10k segment` calls regardless of environment. If this\n  # rule is not observed, the content of instant prompt will be incorrect.\n  #\n  # Usually, you should either not define instant_prompt_* or simply call prompt_* from it. If\n  # instant_prompt_* is not defined for a segment, the segment won\'t be shown in instant prompt.\n  function instant_prompt_example() {\n    # Since prompt_example always makes the same `p10k segment` calls, we can call it from\n    # instant_prompt_example. This will give us the same `example` prompt segment in the instant\n    # and regular prompts.\n    prompt_example\n  }\n\n  # User-defined prompt segments can be customized the same way as built-in segments.\n  typeset -g POWERLEVEL9K_EXAMPLE_FOREGROUND=3\n  typeset -g POWERLEVEL9K_EXAMPLE_BACKGROUND=1\n  # typeset -g POWERLEVEL9K_EXAMPLE_VISUAL_IDENTIFIER_EXPANSION=\'\u{2b50}\'\n\n  # Transient prompt works similarly to the builtin transient_rprompt option. It trims down prompt\n  # when accepting a command line. Supported values:\n  #\n  #   - off:      Don\'t change prompt when accepting a command line.\n  #   - always:   Trim down prompt when accepting a command line.\n  #   - same-dir: Trim down prompt when accepting a command line unless this is the first command\n  #               typed after changing current working directory.\n  typeset -g POWERLEVEL9K_TRANSIENT_PROMPT=always\n\n  # Instant prompt mode.\n  #\n  #   - off:     Disable instant prompt. Choose this if you\'ve tried instant prompt and found\n  #              it incompatible with your zsh configuration files.\n  #   - quiet:   Enable instant prompt and don\'t print warnings when detecting console output\n  #              during zsh initialization. Choose this if you\'ve read and understood\n  #              https://github.com/romkatv/powerlevel10k/blob/master/README.md#instant-prompt.\n  #   - verbose: Enable instant prompt and print a warning when detecting console output during\n  #              zsh initialization. Choose this if you\'ve never tried instant prompt, haven\'t\n  #              seen the warning, or if you are unsure what this all means.\n  typeset -g POWERLEVEL9K_INSTANT_PROMPT=verbose\n\n  # Hot reload allows you to change POWERLEVEL9K options after Powerlevel10k has been initialized.\n  # For example, you can type POWERLEVEL9K_BACKGROUND=red and see your prompt turn red. Hot reload\n  # can slow down prompt by 1-2 milliseconds, so it\'s better to keep it turned off unless you\n  # really need it.\n  typeset -g POWERLEVEL9K_DISABLE_HOT_RELOAD=true\n\n  # If p10k is already loaded, reload configuration.\n  # This works even with POWERLEVEL9K_DISABLE_HOT_RELOAD=true.\n  (( ! $+functions[p10k] )) || p10k reload\n}\n\n# Tell `p10k configure` which file it should overwrite.\ntypeset -g POWERLEVEL9K_CONFIG_FILE=${${(%):-%x}:a}\n\n(( ${#p10k_config_opts} )) && setopt ${p10k_config_opts[@]}\n\'builtin\' \'unset\' \'p10k_config_opts\'\n";
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct P10K_CONFIG_PATH {
            __private_field: (),
        }
        #[doc(hidden)]
        static P10K_CONFIG_PATH: P10K_CONFIG_PATH = P10K_CONFIG_PATH {
            __private_field: (),
        };
        impl ::lazy_static::__Deref for P10K_CONFIG_PATH {
            type Target = String;
            fn deref(&self) -> &String {
                #[inline(always)]
                fn __static_ref_initialize() -> String {
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("{0}/.p10k.zsh", *HOME_DIR),
                        );
                        res
                    }
                }
                #[inline(always)]
                fn __stability() -> &'static String {
                    static LAZY: ::lazy_static::lazy::Lazy<String> = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for P10K_CONFIG_PATH {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        struct P10K_FILES_PATH {
            __private_field: (),
        }
        #[doc(hidden)]
        static P10K_FILES_PATH: P10K_FILES_PATH = P10K_FILES_PATH {
            __private_field: (),
        };
        impl ::lazy_static::__Deref for P10K_FILES_PATH {
            type Target = String;
            fn deref(&self) -> &String {
                #[inline(always)]
                fn __static_ref_initialize() -> String {
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "{0}/.oh-my-zsh/custom/themes/powerlevel10k",
                                *HOME_DIR,
                            ),
                        );
                        res
                    }
                }
                #[inline(always)]
                fn __stability() -> &'static String {
                    static LAZY: ::lazy_static::lazy::Lazy<String> = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for P10K_FILES_PATH {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
        pub struct PowerLevel10k {
            p10k_config_available: ConfigStatus,
            p10k_repo_installed: bool,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PowerLevel10k {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "PowerLevel10k",
                    "p10k_config_available",
                    &self.p10k_config_available,
                    "p10k_repo_installed",
                    &&self.p10k_repo_installed,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for PowerLevel10k {
            #[inline]
            fn default() -> PowerLevel10k {
                PowerLevel10k {
                    p10k_config_available: ::core::default::Default::default(),
                    p10k_repo_installed: ::core::default::Default::default(),
                }
            }
        }
        static POWER_LEVEL10K: std::sync::OnceLock<&'static mut PowerLevel10k> = std::sync::OnceLock::new();
        impl PowerLevel10k {
            pub fn singleton() -> &'static mut Self {
                use std::sync::OnceLock;
                POWER_LEVEL10K
                    .get_or_init(|| {
                        ::std::boxed::Box::leak(::std::boxed::Box::new(Self::default()))
                    })
            }
        }
        impl DependencyInfo for PowerLevel10k {
            fn name(&self) -> &'static str {
                "powerlevel10k"
            }
            fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([Zsh::singleton(), OhMyZsh::singleton()]),
                )
            }
        }
        impl DependencyInstallable for PowerLevel10k {
            fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
                let is_present = metadata(&*P10K_CONFIG_PATH).is_ok();
                let p10k_contents = read_to_string(&*P10K_CONFIG_PATH)?;
                let is_correct = p10k_contents == POWER_LEVEL_10K_CONFIG_BASE;
                self
                    .p10k_config_available = match (is_present, is_correct) {
                    (false, _) => ConfigStatus::NotPresent,
                    (true, false) => ConfigStatus::PresentIncorrect,
                    (true, true) => ConfigStatus::PresentCorrect,
                };
                let is_present = metadata(&*P10K_FILES_PATH).is_ok();
                self.p10k_repo_installed = is_present;
                match (&self.p10k_config_available, &self.p10k_repo_installed) {
                    (ConfigStatus::PresentCorrect, true) => {
                        Ok(InstallationStatus::FullyInstalled)
                    }
                    (ConfigStatus::NotPresent, false) => {
                        Ok(InstallationStatus::NotInstalled)
                    }
                    _ => Ok(InstallationStatus::PartialInstall),
                }
            }
            fn install(&self, _: Option<&str>) -> Result<(), DependencyError> {
                if (match self.p10k_config_available {
                    ConfigStatus::PresentCorrect => true,
                    _ => false,
                }) && self.p10k_repo_installed
                {
                    return Ok(());
                }
                if !match self.p10k_config_available {
                    ConfigStatus::PresentCorrect => true,
                    _ => false,
                } {
                    rename_bak_file(&*P10K_CONFIG_PATH)?;
                    std::fs::write(&*P10K_CONFIG_PATH, POWER_LEVEL_10K_CONFIG_BASE)?;
                }
                if !self.p10k_repo_installed {
                    let output = Command::new("git")
                        .arg("clone")
                        .arg("--depth=1")
                        .arg(POWER_LEVEL_10K_GITHUB_URL)
                        .arg(&*P10K_FILES_PATH)
                        .output()?;
                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Err(
                            DependencyError::DependencyFailed({
                                let res = ::alloc::fmt::format(
                                    format_args!("Failed to install powerlevel10k: {0}", stderr),
                                );
                                res
                            }),
                        );
                    }
                }
                Ok(())
            }
        }
    }
    pub mod zsh_aliases {
        use std::fs::{metadata, read_to_string};
        use singleton_derive::Singleton;
        use super::{
            DependencyError, DependencyInfo, DependencyInstallable, InstallationStatus,
            rename_bak_file, ConfigStatus,
        };
        use crate::{HOME_DIR, dependencies::zsh::Zsh};
        const ZSH_ALIASES_REF: &str = "alias lh=\'ls -lhrt\'\nalias c=\'clear\'\nalias v=\'vim\'\nalias di=\'sudo dnf install\'\nalias v=\'vi\'\nalias rmd=\'yes | rm -r\'\nalias rsync-copy=\"rsync -avz --progress -h\"\nalias rsync-move=\"rsync -avz --progress -h --remove-source-files\"\nalias rsync-update=\"rsync -avzu --progress -h\"\nalias rsync-synchronize=\"rsync -avzu --delete --progress -h\"\nalias e=\'exit 0\'\n";
        pub struct ZshAliases {
            zsh_aliases_available: ConfigStatus,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ZshAliases {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ZshAliases",
                    "zsh_aliases_available",
                    &&self.zsh_aliases_available,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ZshAliases {
            #[inline]
            fn default() -> ZshAliases {
                ZshAliases {
                    zsh_aliases_available: ::core::default::Default::default(),
                }
            }
        }
        static ZSH_ALIASES: std::sync::OnceLock<&'static mut ZshAliases> = std::sync::OnceLock::new();
        impl ZshAliases {
            pub fn singleton() -> &'static mut Self {
                use std::sync::OnceLock;
                ZSH_ALIASES
                    .get_or_init(|| {
                        ::std::boxed::Box::leak(::std::boxed::Box::new(Self::default()))
                    })
            }
        }
        impl DependencyInfo for ZshAliases {
            fn name(&self) -> &'static str {
                "zsh_aliases"
            }
            fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([Zsh::singleton()]),
                )
            }
        }
        impl DependencyInstallable for ZshAliases {
            fn is_installed(&self) -> Result<InstallationStatus, DependencyError> {
                let zsh_aliases_path = {
                    let res = ::alloc::fmt::format(
                        format_args!("{0}/.zsh_aliases", *HOME_DIR),
                    );
                    res
                };
                let is_present = metadata(zsh_aliases_path.clone()).is_ok();
                let zsh_aliases_contents = read_to_string(zsh_aliases_path.clone())?;
                let all_present = ZSH_ALIASES_REF
                    .lines()
                    .all(|line| zsh_aliases_contents.contains(line));
                self
                    .zsh_aliases_available = match (is_present, all_present) {
                    (false, _) => ConfigStatus::NotPresent,
                    (true, false) => ConfigStatus::PresentIncorrect,
                    (true, true) => ConfigStatus::PresentCorrect,
                };
                if match self.zsh_aliases_available {
                    ConfigStatus::PresentCorrect => true,
                    _ => false,
                } {
                    Ok(InstallationStatus::FullyInstalled)
                } else {
                    Ok(InstallationStatus::NotInstalled)
                }
            }
            fn install(&self, _: Option<&str>) -> Result<(), DependencyError> {
                if match self.zsh_aliases_available {
                    ConfigStatus::PresentCorrect => true,
                    _ => false,
                } {
                    return Ok(());
                }
                let zsh_aliases_path = {
                    let res = ::alloc::fmt::format(
                        format_args!("{0}/.zsh_aliases", *HOME_DIR),
                    );
                    res
                };
                let zsh_aliases_contents = read_to_string(&zsh_aliases_path)?;
                rename_bak_file(&zsh_aliases_path)?;
                std::fs::write(&zsh_aliases_path, ZSH_ALIASES_REF)?;
                for line in zsh_aliases_contents.lines() {
                    if !ZSH_ALIASES_REF.contains(line) {
                        std::fs::write(
                            &zsh_aliases_path,
                            {
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}\n{1}", zsh_aliases_contents, line),
                                );
                                res
                            },
                        )?;
                    }
                }
                Ok(())
            }
        }
    }
    pub fn rename_bak_file(file_path: &str) -> Result<(), std::io::Error> {
        let bak_path = {
            let res = ::alloc::fmt::format(format_args!("{0}.bak", file_path));
            res
        };
        if metadata(&bak_path).is_ok() {
            std::fs::remove_file(&bak_path)?;
        }
        std::fs::rename(file_path, bak_path)?;
        Ok(())
    }
    pub enum ConfigStatus {
        NotPresent,
        PresentIncorrect,
        PresentCorrect,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConfigStatus {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ConfigStatus::NotPresent => "NotPresent",
                    ConfigStatus::PresentIncorrect => "PresentIncorrect",
                    ConfigStatus::PresentCorrect => "PresentCorrect",
                },
            )
        }
    }
    impl Default for ConfigStatus {
        fn default() -> Self {
            ConfigStatus::NotPresent
        }
    }
    pub enum DependencyError {
        Unknown,
        NotInstalled,
        UnsupportedOperatingSystem,
        IoError(std::io::Error),
        DependencyFailed(String),
        Utf8Error(FromUtf8Error),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DependencyError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                DependencyError::Unknown => {
                    ::core::fmt::Formatter::write_str(f, "Unknown")
                }
                DependencyError::NotInstalled => {
                    ::core::fmt::Formatter::write_str(f, "NotInstalled")
                }
                DependencyError::UnsupportedOperatingSystem => {
                    ::core::fmt::Formatter::write_str(f, "UnsupportedOperatingSystem")
                }
                DependencyError::IoError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "IoError",
                        &__self_0,
                    )
                }
                DependencyError::DependencyFailed(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "DependencyFailed",
                        &__self_0,
                    )
                }
                DependencyError::Utf8Error(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Utf8Error",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl std::fmt::Display for DependencyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                DependencyError::Unknown => f.write_fmt(format_args!("Unknown error")),
                DependencyError::NotInstalled => {
                    f.write_fmt(format_args!("Dependency not installed"))
                }
                DependencyError::UnsupportedOperatingSystem => {
                    f.write_fmt(format_args!("Unsupported operating system"))
                }
                DependencyError::IoError(e) => {
                    f.write_fmt(format_args!("IO error: {0}", e))
                }
                DependencyError::DependencyFailed(e) => {
                    f.write_fmt(
                        format_args!(
                            "Missing or unable to install required dependency: {0}",
                            e,
                        ),
                    )
                }
                DependencyError::Utf8Error(e) => {
                    f.write_fmt(format_args!("UTF-8 error: {0}", e))
                }
            }
        }
    }
    impl std::error::Error for DependencyError {}
    impl From<std::io::Error> for DependencyError {
        fn from(e: std::io::Error) -> Self {
            DependencyError::IoError(e)
        }
    }
    impl From<FromUtf8Error> for DependencyError {
        fn from(e: FromUtf8Error) -> Self {
            DependencyError::Utf8Error(e)
        }
    }
    pub enum InstallationStatus {
        FullyInstalled,
        PartialInstall,
        NotInstalled,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InstallationStatus {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    InstallationStatus::FullyInstalled => "FullyInstalled",
                    InstallationStatus::PartialInstall => "PartialInstall",
                    InstallationStatus::NotInstalled => "NotInstalled",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for InstallationStatus {
        #[inline]
        fn clone(&self) -> InstallationStatus {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for InstallationStatus {}
    pub enum Installable {
        AlreadyInstalled,
        MissingDependency,
        InvalidOS,
        Other(String),
        Unknown,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Installable {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Installable::AlreadyInstalled => {
                    ::core::fmt::Formatter::write_str(f, "AlreadyInstalled")
                }
                Installable::MissingDependency => {
                    ::core::fmt::Formatter::write_str(f, "MissingDependency")
                }
                Installable::InvalidOS => {
                    ::core::fmt::Formatter::write_str(f, "InvalidOS")
                }
                Installable::Other(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Other",
                        &__self_0,
                    )
                }
                Installable::Unknown => ::core::fmt::Formatter::write_str(f, "Unknown"),
            }
        }
    }
    pub trait DependencyInfo {
        /// Get the name of the dependency.
        fn name(&self) -> &'static str;
        /// Get a list of all dependencies that this application requires
        fn requires(&self) -> Vec<&'static dyn DependencyInstallable> {
            ::alloc::vec::Vec::new()
        }
    }
    pub trait DependencyInstallable: DependencyInfo {
        /// Check if the dependency is installed on the current system.
        /// Updates internal state to reflect the current status.
        fn is_installed(&self) -> Result<InstallationStatus, DependencyError>;
        /// Install the dependency.
        fn install(&self, version: Option<&str>) -> Result<(), DependencyError>;
    }
}
mod event {
    use crate::app::AppResult;
    use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
    use std::sync::mpsc;
    use std::thread;
    use std::time::{Duration, Instant};
    /// Terminal events.
    pub enum Event {
        /// Terminal tick.
        Tick,
        /// Key press.
        Key(KeyEvent),
        /// Mouse click/scroll.
        Mouse(MouseEvent),
        /// Terminal resize.
        Resize(u16, u16),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Event {
        #[inline]
        fn clone(&self) -> Event {
            let _: ::core::clone::AssertParamIsClone<KeyEvent>;
            let _: ::core::clone::AssertParamIsClone<MouseEvent>;
            let _: ::core::clone::AssertParamIsClone<u16>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Event {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Event {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Event::Tick => ::core::fmt::Formatter::write_str(f, "Tick"),
                Event::Key(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Key",
                        &__self_0,
                    )
                }
                Event::Mouse(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Mouse",
                        &__self_0,
                    )
                }
                Event::Resize(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "Resize",
                        __self_0,
                        &__self_1,
                    )
                }
            }
        }
    }
    /// Terminal event handler.
    #[allow(dead_code)]
    pub struct EventHandler {
        /// Event sender channel.
        sender: mpsc::Sender<Event>,
        /// Event receiver channel.
        receiver: mpsc::Receiver<Event>,
        /// Event handler thread.
        handler: thread::JoinHandle<()>,
    }
    #[automatically_derived]
    #[allow(dead_code)]
    impl ::core::fmt::Debug for EventHandler {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "EventHandler",
                "sender",
                &self.sender,
                "receiver",
                &self.receiver,
                "handler",
                &&self.handler,
            )
        }
    }
    impl EventHandler {
        /// Constructs a new instance of [`EventHandler`].
        pub fn new(tick_rate: u64) -> Self {
            let tick_rate = Duration::from_millis(tick_rate);
            let (sender, receiver) = mpsc::channel();
            let handler = {
                let sender = sender.clone();
                thread::spawn(move || {
                    let mut last_tick = Instant::now();
                    loop {
                        let timeout = tick_rate
                            .checked_sub(last_tick.elapsed())
                            .unwrap_or(tick_rate);
                        if event::poll(timeout).expect("no events available") {
                            match event::read().expect("unable to read event") {
                                CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                                CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                                CrosstermEvent::Resize(w, h) => {
                                    sender.send(Event::Resize(w, h))
                                }
                                _ => ::core::panicking::panic("not implemented"),
                            }
                                .expect("failed to send terminal event")
                        }
                        if last_tick.elapsed() >= tick_rate {
                            sender.send(Event::Tick).expect("failed to send tick event");
                            last_tick = Instant::now();
                        }
                    }
                })
            };
            Self { sender, receiver, handler }
        }
        /// Receive the next event from the handler thread.
        ///
        /// This function will always block the current thread if
        /// there is no data available and it's possible for more data to be sent.
        pub fn next(&self) -> AppResult<Event> {
            Ok(self.receiver.recv()?)
        }
    }
}
mod handler {
    use crate::app::{App, AppResult};
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.quit();
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
            }
            _ => {}
        }
        Ok(())
    }
}
mod tui {
    use crate::app::{App, AppResult};
    use crate::event::EventHandler;
    use crate::ui;
    use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
    use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
    use ratatui::backend::Backend;
    use ratatui::Terminal;
    use std::io;
    /// Representation of a terminal user interface.
    ///
    /// It is responsible for setting up the terminal,
    /// initializing the interface and handling the draw events.
    pub struct Tui<B: Backend> {
        /// Interface to the Terminal.
        terminal: Terminal<B>,
        /// Terminal event handler.
        pub events: EventHandler,
    }
    #[automatically_derived]
    impl<B: ::core::fmt::Debug + Backend> ::core::fmt::Debug for Tui<B> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Tui",
                "terminal",
                &self.terminal,
                "events",
                &&self.events,
            )
        }
    }
    impl<B: Backend> Tui<B> {
        /// Constructs a new instance of [`Tui`].
        pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
            Self { terminal, events }
        }
        /// Initializes the terminal interface.
        ///
        /// It enables the raw mode and sets terminal properties.
        pub fn init(&mut self) -> AppResult<()> {
            terminal::enable_raw_mode()?;
            {
                use ::std::io::Write;
                {
                    use ::std::io::Write;
                    Ok(io::stderr().by_ref())
                        .and_then(|writer| ::crossterm::QueueableCommand::queue(
                            writer,
                            EnterAlternateScreen,
                        ))
                        .and_then(|writer| ::crossterm::QueueableCommand::queue(
                            writer,
                            EnableMouseCapture,
                        ))
                        .map(|_| ())
                }
                    .and_then(|()| { ::std::io::Write::flush(io::stderr().by_ref()) })
            }?;
            self.terminal.hide_cursor()?;
            self.terminal.clear()?;
            Ok(())
        }
        /// [`Draw`] the terminal interface by [`rendering`] the widgets.
        ///
        /// [`Draw`]: ratatui::Terminal::draw
        /// [`rendering`]: crate::ui:render
        pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
            self.terminal.draw(|frame| ui::render(app, frame))?;
            Ok(())
        }
        /// Exits the terminal interface.
        ///
        /// It disables the raw mode and reverts back the terminal properties.
        pub fn exit(&mut self) -> AppResult<()> {
            terminal::disable_raw_mode()?;
            {
                use ::std::io::Write;
                {
                    use ::std::io::Write;
                    Ok(io::stderr().by_ref())
                        .and_then(|writer| ::crossterm::QueueableCommand::queue(
                            writer,
                            LeaveAlternateScreen,
                        ))
                        .and_then(|writer| ::crossterm::QueueableCommand::queue(
                            writer,
                            DisableMouseCapture,
                        ))
                        .map(|_| ())
                }
                    .and_then(|()| { ::std::io::Write::flush(io::stderr().by_ref()) })
            }?;
            self.terminal.show_cursor()?;
            Ok(())
        }
    }
}
mod ui {
    use std::collections::HashMap;
    use ratatui::{
        backend::Backend, style::Color,
        widgets::{
            canvas::{Canvas, Circle, Line},
            Block, Borders,
        },
        Frame,
    };
    use crate::app::App;
    /// Find the intersection point between a circle and a line, picks the intersection that will reduce
    /// the length of the line the most.
    fn find_intersection(
        circle_pos: (f64, f64),
        circle_radius: f64,
        line_start: (f64, f64),
        line_end: (f64, f64),
    ) -> Option<(f64, f64)> {
        let dx = line_end.0 - line_start.0;
        let dy = line_end.1 - line_start.1;
        let a = dx * dx + dy * dy;
        let b = 2.0
            * (dx * (line_start.0 - circle_pos.0) + dy * (line_start.1 - circle_pos.1));
        let c = (line_start.0 - circle_pos.0) * (line_start.0 - circle_pos.0)
            + (line_start.1 - circle_pos.1) * (line_start.1 - circle_pos.1)
            - circle_radius * circle_radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant <= 0.0 {
            return None;
        }
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        Some((line_start.0 + t1 * dx, line_start.1 + t1 * dy))
    }
    const CIRCLE_DISTANCE: f64 = 100.0;
    const CIRCLE_RADIUS: f64 = 20.0;
    const COL_SEPARATION: f64 = 100.0;
    /// Renders the user interface widgets.
    pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Dependency Graph"))
            .x_bounds([0.0, 1000.0])
            .y_bounds([0.0, 500.0])
            .paint(|ctx| {});
        frame.render_widget(canvas, frame.size());
    }
}
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
use crate::dependencies::{
    zsh::Zsh, powerlevel10k::PowerLevel10k, zsh_aliases::ZshAliases, docker::Docker,
    DependencyInstallable, zshrc::Zshrc, DependencyInfo,
};
#[allow(missing_copy_implementations)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
struct OPERATING_SYSTEM {
    __private_field: (),
}
#[doc(hidden)]
static OPERATING_SYSTEM: OPERATING_SYSTEM = OPERATING_SYSTEM {
    __private_field: (),
};
impl ::lazy_static::__Deref for OPERATING_SYSTEM {
    type Target = OperatingSystem;
    fn deref(&self) -> &OperatingSystem {
        #[inline(always)]
        fn __static_ref_initialize() -> OperatingSystem {
            OperatingSystem::from_sysinfo()
                .expect("Unable to determine operating system")
        }
        #[inline(always)]
        fn __stability() -> &'static OperatingSystem {
            static LAZY: ::lazy_static::lazy::Lazy<OperatingSystem> = ::lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}
impl ::lazy_static::LazyStatic for OPERATING_SYSTEM {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}
#[allow(missing_copy_implementations)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
struct CURRENT_USER {
    __private_field: (),
}
#[doc(hidden)]
static CURRENT_USER: CURRENT_USER = CURRENT_USER {
    __private_field: (),
};
impl ::lazy_static::__Deref for CURRENT_USER {
    type Target = String;
    fn deref(&self) -> &String {
        #[inline(always)]
        fn __static_ref_initialize() -> String {
            whoami::username()
        }
        #[inline(always)]
        fn __stability() -> &'static String {
            static LAZY: ::lazy_static::lazy::Lazy<String> = ::lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}
impl ::lazy_static::LazyStatic for CURRENT_USER {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}
#[allow(missing_copy_implementations)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
struct HOME_DIR {
    __private_field: (),
}
#[doc(hidden)]
static HOME_DIR: HOME_DIR = HOME_DIR { __private_field: () };
impl ::lazy_static::__Deref for HOME_DIR {
    type Target = String;
    fn deref(&self) -> &String {
        #[inline(always)]
        fn __static_ref_initialize() -> String {
            home::home_dir()
                .expect("Unable to find home directory")
                .to_str()
                .expect("Unable to convert home directory to String")
                .to_string()
        }
        #[inline(always)]
        fn __stability() -> &'static String {
            static LAZY: ::lazy_static::lazy::Lazy<String> = ::lazy_static::lazy::Lazy::INIT;
            LAZY.get(__static_ref_initialize)
        }
        __stability()
    }
}
impl ::lazy_static::LazyStatic for HOME_DIR {
    fn initialize(lazy: &Self) {
        let _ = &**lazy;
    }
}
enum DotfilesError {
    UnknownOperatingSystem(String),
    UnsupportedOperatingSystem,
}
#[automatically_derived]
impl ::core::fmt::Debug for DotfilesError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            DotfilesError::UnknownOperatingSystem(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "UnknownOperatingSystem",
                    &__self_0,
                )
            }
            DotfilesError::UnsupportedOperatingSystem => {
                ::core::fmt::Formatter::write_str(f, "UnsupportedOperatingSystem")
            }
        }
    }
}
enum OperatingSystem {
    Ubuntu2204,
    Ubuntu2004,
    Ubuntu1804,
    Fedora38,
    Rocky9,
    Rocky8,
    PopOS2104,
}
#[automatically_derived]
impl ::core::fmt::Debug for OperatingSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                OperatingSystem::Ubuntu2204 => "Ubuntu2204",
                OperatingSystem::Ubuntu2004 => "Ubuntu2004",
                OperatingSystem::Ubuntu1804 => "Ubuntu1804",
                OperatingSystem::Fedora38 => "Fedora38",
                OperatingSystem::Rocky9 => "Rocky9",
                OperatingSystem::Rocky8 => "Rocky8",
                OperatingSystem::PopOS2104 => "PopOS2104",
            },
        )
    }
}
impl OperatingSystem {
    fn from_sysinfo() -> Result<Self, DotfilesError> {
        let system = sysinfo::System::new_all();
        {
            ::std::io::_print(
                format_args!("System name:             {0:?}\n", system.name()),
            );
        };
        {
            ::std::io::_print(
                format_args!("System kernel version:   {0:?}\n", system.kernel_version()),
            );
        };
        {
            ::std::io::_print(
                format_args!("System OS version:       {0:?}\n", system.os_version()),
            );
        };
        {
            ::std::io::_print(
                format_args!("System host name:        {0:?}\n", system.host_name()),
            );
        };
        {
            ::std::io::_print(
                format_args!("System uptime:           {0}\n", system.uptime()),
            );
        };
        {
            ::std::io::_print(
                format_args!("System number of users:  {0}\n", system.users().len()),
            );
        };
        {
            ::std::io::_print(
                format_args!("System processes:        {0}\n", system.processes().len()),
            );
        };
        {
            ::std::io::_print(
                format_args!("System total memory:     {0} kB\n", system.total_memory()),
            );
        };
        {
            ::std::io::_print(
                format_args!("System free memory:      {0} kB\n", system.free_memory()),
            );
        };
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
            Err(
                DotfilesError::UnknownOperatingSystem(
                    "Unable to determine operating system".to_string(),
                ),
            )
        }
    }
}
fn main() {
    pretty_env_logger::formatted_builder().filter_level(LevelFilter::Trace).init();
    {
        let lvl = ::log::Level::Error;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                format_args!("This is an error!"),
                lvl,
                &("dotfiles", "dotfiles", "src/main.rs", 146u32),
                ::log::__private_api::Option::None,
            );
        }
    };
    {
        let lvl = ::log::Level::Warn;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                format_args!("This is a warn!"),
                lvl,
                &("dotfiles", "dotfiles", "src/main.rs", 147u32),
                ::log::__private_api::Option::None,
            );
        }
    };
    {
        let lvl = ::log::Level::Info;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                format_args!("This is an info!"),
                lvl,
                &("dotfiles", "dotfiles", "src/main.rs", 148u32),
                ::log::__private_api::Option::None,
            );
        }
    };
    {
        let lvl = ::log::Level::Debug;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                format_args!("This is a debug!"),
                lvl,
                &("dotfiles", "dotfiles", "src/main.rs", 149u32),
                ::log::__private_api::Option::None,
            );
        }
    };
    {
        let lvl = ::log::Level::Trace;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                format_args!("This is a trace!"),
                lvl,
                &("dotfiles", "dotfiles", "src/main.rs", 150u32),
                ::log::__private_api::Option::None,
            );
        }
    };
    let is_sudo = sudo::escalate_if_needed().unwrap();
    if is_sudo != RunningAs::Root {
        {
            ::std::io::_print(
                format_args!(
                    "This application requires root privileges to install dependencies\n",
                ),
            );
        };
        std::process::exit(1);
    }
    let user = whoami::username();
    {
        ::std::io::_print(format_args!("Running as user: {0}\n", user));
    };
    let to_install: Vec<&dyn DependencyInstallable> = <[_]>::into_vec(
        #[rustc_box]
        ::alloc::boxed::Box::new([
            Docker::singleton(),
            PowerLevel10k::singleton(),
            ZshAliases::singleton(),
            Zshrc::singleton(),
        ]),
    );
    fn recursively_install_dependencies(dependency: &mut dyn DependencyInstallable) {
        {
            ::std::io::_print(
                format_args!("Installing dependencies for: {0}\n", dependency.name()),
            );
        };
        for dep in dependency.requires() {
            {
                ::std::io::_print(
                    format_args!("Installing dependency: {0}\n", dep.name()),
                );
            };
            recursively_install_dependencies(dependency);
            dependency.install(None).unwrap();
        }
        {
            ::std::io::_print(
                format_args!("Installing dependency: {0}\n", dependency.name()),
            );
        };
        dependency.install(None).unwrap();
    }
    for mut dependency in to_install.into_iter() {
        let dep = &mut *dependency;
        recursively_install_dependencies(dep);
    }
}
