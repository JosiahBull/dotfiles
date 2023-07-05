use std::{
    fmt::{Debug, Display},
    io::BufRead,
    process::{Command, Stdio},
};

use crate::CURRENT_USER;

#[derive(Debug)]
pub enum CommandError {
    IoError(std::io::Error),
    Utf8Error(std::string::FromUtf8Error),
    CommandFailed(Output),
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::IoError(e) => write!(f, "IO error: {}", e),
            CommandError::Utf8Error(e) => write!(f, "UTF-8 error: {}", e),
            CommandError::CommandFailed(e) => write!(f, "Command failed: {}", e),
        }
    }
}

impl std::error::Error for CommandError {}

impl From<std::io::Error> for CommandError {
    fn from(e: std::io::Error) -> Self {
        CommandError::IoError(e)
    }
}

impl From<std::string::FromUtf8Error> for CommandError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        CommandError::Utf8Error(e)
    }
}

#[derive(Debug, Clone)]
pub struct Output {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "success: {}\nstdout: {}\nstderr: {}",
            self.success, self.stdout, self.stderr
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AsUser {
    DefaultUser,
    Root,
    DoNothing,
}

impl Default for AsUser {
    fn default() -> Self {
        Self::Root
    }
}

pub struct DCommand<'a, T: AsRef<std::ffi::OsStr>> {
    /// The command to run.
    cmd: T,
    /// Arguments to pass to the command.
    args: &'a [T],
    /// What user to run the command as.
    user: AsUser,
    /// Whether to print the stdout/stderr as the command is running.
    live_print: bool,
}

impl<T: AsRef<std::ffi::OsStr> + Debug> Debug for DCommand<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command")
            .field("cmd", &self.cmd)
            .field("args", &self.args)
            .field("user", &self.user)
            .field("live_print", &self.live_print)
            .finish()
    }
}

impl<T: AsRef<std::ffi::OsStr>> Display for DCommand<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut command = self.cmd.as_ref().to_string_lossy().to_string();
        for arg in self.args {
            command.push(' ');
            command.push_str(arg.as_ref().to_string_lossy().as_ref());
        }
        write!(f, "{}", command)
    }
}

impl<T: AsRef<std::ffi::OsStr> + Clone> Clone for DCommand<'_, T> {
    fn clone(&self) -> Self {
        Self {
            cmd: self.cmd.clone(),
            args: self.args.clone(),
            user: self.user.clone(),
            live_print: self.live_print.clone(),
        }
    }
}

impl<T: AsRef<std::ffi::OsStr> + Copy> Copy for DCommand<'_, T> {}

impl<T: AsRef<std::ffi::OsStr> + Default> Default for DCommand<'_, T> {
    fn default() -> Self {
        Self {
            cmd: Default::default(),
            args: &[],
            user: Default::default(),
            live_print: true,
        }
    }
}

impl<'a, T: AsRef<std::ffi::OsStr>> DCommand<'a, T> {
    pub fn new(cmd: T, args: &'a [T]) -> Self {
        Self {
            cmd,
            args,
            user: Default::default(),
            live_print: false,
        }
    }

    pub fn user(mut self, user: AsUser) -> Self {
        self.user = user;
        self
    }

    pub fn live_print(mut self, live_print: bool) -> Self {
        self.live_print = live_print;
        self
    }

    pub fn run(self) -> Result<Output, CommandError> {
        // sudo -u "sh -c "curl https://sh.rustup.rs -sSf | sh -s -- -y""

        // commands should be run as
        // sh -c "command <args>"
        // or, if running as a non-root user
        // sudo -u <user> sh -c "command <args>"
        let mut command;
        if let AsUser::Root = self.user {
            command = Command::new("sh");
            command.arg("-c").arg(self.build_command_string());
        } else {
            let user = match self.user {
                AsUser::DefaultUser => &*CURRENT_USER,
                AsUser::Root => "root",
                AsUser::DoNothing => "",
            };
            command = Command::new("sudo");
            command
                .arg("-u")
                .arg(user)
                .arg("sh")
                .arg("-c")
                .arg(self.build_command_string());
        }

        // print full command + arg
        println!("With self {}", self);
        println!("Command constructed: {:?}", command);

        if !self.live_print {
            command.stdout(Stdio::piped()).stderr(Stdio::piped());
            let cmd = command.spawn().unwrap().wait_with_output().unwrap();

            let stdout = String::from_utf8(cmd.stdout).unwrap();
            let stderr = String::from_utf8(cmd.stderr).unwrap();

            return Ok(Output {
                success: cmd.status.success(),
                stdout,
                stderr,
            });
        }

        // live printing, means we want to print the stdout/stderr as the command is running
        // AND we want to return the command result containing that same stdout/stderr
        command.stdout(Stdio::piped()).stderr(Stdio::piped());

        let mut cmd = command.spawn().unwrap();

        // capture piped output as it's produced
        let stdout = cmd.stdout.take().unwrap();
        let stderr = cmd.stderr.take().unwrap();

        // print as produced, and save into a string
        let mut stdout_reader = std::io::BufReader::new(stdout);
        let mut stderr_reader = std::io::BufReader::new(stderr);
        let mut stdout = String::new();
        let mut stderr = String::new();
        let mut stdout_done = false;
        let mut stderr_done = false;
        loop {
            // try read from stdout, if there is a line, print it then save to string
            if !stdout_done {
                let mut stdout_line = String::new();
                match stdout_reader.read_line(&mut stdout_line) {
                    Ok(0) => stdout_done = true,
                    Ok(_) => {
                        print!("{}", stdout_line);
                        stdout.push_str(&stdout_line);
                    }
                    Err(e) => {
                        eprintln!("Error reading stdout: {}", e);
                        stdout_done = true;
                    }
                }
            }

            // try read from stderr, if there is a line, print it then save to string
            if !stderr_done {
                let mut stderr_line = String::new();
                match stderr_reader.read_line(&mut stderr_line) {
                    Ok(0) => stderr_done = true,
                    Ok(_) => {
                        eprint!("{}", stderr_line);
                        stderr.push_str(&stderr_line);
                    }
                    Err(e) => {
                        eprintln!("Error reading stderr: {}", e);
                        stderr_done = true;
                    }
                }
            }

            if stdout_done && stderr_done {
                break;
            }

            // wait a bit before trying to read again
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        let cmd = cmd.wait()?;
        Ok(Output {
            success: cmd.success(),
            stdout,
            stderr,
        })
    }

    fn build_command_string(&self) -> String {
        let cmd = self.cmd.as_ref().to_string_lossy().to_string();
        let args = self
            .args
            .iter()
            .map(|arg| arg.as_ref().to_string_lossy().to_string())
            .collect::<Vec<String>>()
            .join(" ");
        format!("{} {}", cmd, args)
    }
}

// XXX: Implementing a system-wide package manager would be useful
// for creating fully async commands. This would allow us to run
// multiple commands at once, and wait for them all to finish.
// We could have a single default for all platforms, and then allow
// the consumer to override it for individual platforms if they wish.
// This should ideally be worked out at compile time, maybe with traits?
