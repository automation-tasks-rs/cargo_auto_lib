// auto_helper_functions_mod

//! various helper functions

use crate::public_api_mod::{RED, RESET, YELLOW};

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

/// Run one shell command
///
/// Exit task execution if the command has Exit Status != 0.
pub fn run_shell_command(shell_command: &str) {
    if !shell_command.starts_with("echo ") {
        println!("    {YELLOW}$ {shell_command}{RESET}");
    }
    let status = std::process::Command::new("sh").arg("-c").arg(shell_command).spawn().unwrap().wait().unwrap();
    let exit_code = status.code().expect(&format!("{RED}Error. Exiting...{RESET}"));
    if exit_code != 0 {
        eprintln!("{RED}Error: {}. Exiting...{RESET}", exit_code);
        std::process::exit(1);
    }
}

/// Run one shell command and return ShellOutput {exit_status, stdout, stderr}
pub fn run_shell_command_output(shell_command: &str) -> ShellOutput {
    if !shell_command.starts_with("echo ") {
        println!("   {YELLOW} $ {shell_command}{RESET}");
    }
    let output = std::process::Command::new("sh").arg("-c").arg(shell_command).output().unwrap();
    // return
    ShellOutput {
        status: output.status.code().unwrap(),
        stdout: String::from_utf8(output.stdout).unwrap(),
        stderr: String::from_utf8(output.stderr).unwrap(),
    }
}

/// Run one shell command and return true if success
pub fn run_shell_command_success(shell_command: &str) -> bool {
    if !shell_command.starts_with("echo ") {
        println!("    {YELLOW}$ {shell_command}{RESET}",);
    }
    let status = std::process::Command::new("sh").arg("-c").arg(shell_command).status().unwrap();
    // return
    status.success()
}

// region: auto_md_to_doc_comments include doc_comments_long/exit_if_not_run_in_rust_project_root_directory.md A ///
/// Check if the code was run inside the Rust project root directory.  
///
/// There must be the `Cargo.toml` file and the directory `automation_tasks_rs`  
/// If not, exit with error message.  
///
// endregion: auto_md_to_doc_comments include doc_comments_long/exit_if_not_run_in_rust_project_root_directory.md A ///
pub fn exit_if_not_run_in_rust_project_root_directory() {
    if !(camino::Utf8Path::new("automation_tasks_rs").exists() && (camino::Utf8Path::new("Cargo.toml").exists())) {
        eprintln!("{RED}Error: `automation_tasks_rs` must be run inside the Rust project in the dir that contains");
        eprintln!("`Cargo.toml` file and `automation_tasks_rs` directory. Exiting...{RESET}");
        // early exit
        std::process::exit(1);
    }
}

/// println one, more or all sub_commands
pub fn completion_return_one_or_more_sub_commands(sub_commands: Vec<&str>, word_being_completed: &str) {
    let mut sub_found = false;
    for sub_command in sub_commands.iter() {
        if sub_command.starts_with(word_being_completed) {
            println!("{sub_command}");
            sub_found = true;
        }
    }
    if !sub_found {
        // print all sub-commands
        for sub_command in sub_commands.iter() {
            println!("{sub_command}");
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
                panic!("{RED}Unable to get your home dir!{RESET}");
            }
        }
        None => panic!("{RED}Unable to get your home dir!{RESET}"),
    }
}
