// auto_helper_functions_mod

//! various helper functions

use std::process::exit;

use crate::public_api_mod::{RED, RESET};

/// similar to std::process::Output, but with i32 and Strings for easier work
#[derive(Debug)]
pub struct ShellOutput {
    /// The status (exit code) of the process.
    pub status: i32,
    /// The string that the process wrote to stdout.
    pub stdout: String,
    /// The string that the process wrote to stderr.
    pub stderr: String,
}

/// run one shell command
/// Stops task execution if the command has Exit Status != 0
pub fn run_shell_command(shell_command: &str) {
    if !shell_command.starts_with("echo ") {
        println!("    $ {}", shell_command);
    }
    let status = std::process::Command::new("sh").arg("-c").arg(shell_command).spawn().unwrap().wait().unwrap();
    let exit_code = status.code();
    if exit_code.is_some() && exit_code != Some(0) {
        eprintln!("{RED}!!! cargo_auto error {}. Stopping automation task execution !!!{RESET}", exit_code.unwrap());
        exit(1);
    }
}

/// run shell commands from a vector of strings.
/// Stops task execution if oe of the commands has Exit Status != 0
pub fn run_shell_commands(shell_commands: Vec<&str>) {
    for shell_command in shell_commands {
        run_shell_command(shell_command);
    }
}

/// run one shell command and return ShellOutput {exit_status,stdout,stderr}
pub fn run_shell_command_output(shell_command: &str) -> ShellOutput {
    if !shell_command.starts_with("echo ") {
        println!("    $ {}", shell_command);
    }
    let output = std::process::Command::new("sh").arg("-c").arg(shell_command).output().unwrap();
    // return
    ShellOutput {
        status: output.status.code().unwrap(),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

/// check if run in rust project root directory error
/// there must be Cargo.toml and the directory automation_tasks_rs
/// exit with error message if not
pub fn exit_if_not_run_in_rust_project_root_directory() {
    if !(std::path::Path::new("automation_tasks_rs").exists() && (std::path::Path::new("Cargo.toml").exists() || std::path::Path::new("Cargo-auto.toml").exists())) {
        eprintln!("{RED}Error: automation_tasks_rs must be called in the root directory of the rust project beside the Cargo.toml (or Cargo-auto.toml) file and automation_tasks_rs directory.{RESET}");
        // early exit
        std::process::exit(0);
    }
}

/// println one, more or all sub_commands
pub fn completion_return_one_or_more_sub_commands(sub_commands: Vec<&str>, word_being_completed: &str) {
    let mut sub_found = false;
    for sub_command in sub_commands.iter() {
        if sub_command.starts_with(word_being_completed) {
            println!("{}", sub_command);
            sub_found = true;
        }
    }
    if !sub_found {
        // print all sub-commands
        for sub_command in sub_commands.iter() {
            println!("{}", sub_command);
        }
    }
}

/// home_dir() using the home crate
/// panics if HOME not found
pub fn home_dir() -> std::path::PathBuf {
    match home::home_dir() {
        Some(path_buff) => {
            if !path_buff.as_os_str().is_empty() {
                path_buff
            } else {
                panic!("Unable to get your home dir!");
            }
        }
        None => panic!("Unable to get your home dir!"),
    }
}
