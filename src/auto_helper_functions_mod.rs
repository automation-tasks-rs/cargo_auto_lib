// auto_helper_functions_mod

//! various helper functions

use lazy_static::lazy_static;

// termion ansi codes for terminal
// format!("{}test{}", *RED,*RESET);
lazy_static! {
    /// ansi code for color
    pub static ref GREEN: String = termion::color::Fg(termion::color::Green).to_string();
    /// ansi code for color
    pub static ref YELLOW: String = termion::color::Fg(termion::color::Yellow).to_string();
    /// ansi code for color
    pub static ref RED: String = termion::color::Fg(termion::color::Red).to_string();
    /// ansi code for reset color
    pub static ref RESET: String = termion::color::Fg(termion::color::Reset).to_string();
    /// ansi code for clear line
    pub static ref CLEAR_LINE: String = termion::clear::CurrentLine.to_string();
    /// ansi code for clear all
    pub static ref CLEAR_ALL: String = termion::clear::All.to_string();
    /// ansi code to unhide cursor
    pub static ref UNHIDE_CURSOR: String = termion::cursor::Show.to_string();
}

/// run one shell command
pub fn run_shell_command(shell_command: &str) {
    if !shell_command.starts_with("echo ") {
        println!("$ {}", shell_command);
    }
    std::process::Command::new("sh")
        .arg("-c")
        .arg(shell_command)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

/// run shell commands from a vector of strings.
pub fn run_shell_commands(shell_commands: Vec<&str>) {
    for shell_command in shell_commands {
        run_shell_command(shell_command);
    }
}

/// check if run in rust project root directory error
/// there must be Cargo.toml and the directory automation_tasks_rs
/// exit with error message if not
pub fn exit_if_not_run_in_rust_project_root_directory() {
    if !(std::path::Path::new("automation_tasks_rs").exists()
        && std::path::Path::new("Cargo.toml").exists())
    {
        println!("Error: automation_tasks_rs must be called in the root directory of the rust project beside the Cargo.toml file and automation_tasks_rs directory.");
        // early exit
        std::process::exit(0);
    }
}

/// println one, more or all sub_commands
pub fn completion_return_one_or_more_sub_commands(
    sub_commands: Vec<&str>,
    word_being_completed: &str,
) {
    let mut sub_found = false;
    for sub_command in sub_commands.iter() {
        if sub_command.starts_with(word_being_completed) {
            println!("{}", sub_command);
            sub_found = true;
        }
    }
    if sub_found == false {
        // print all sub-commands
        for sub_command in sub_commands.iter() {
            println!("{}", sub_command);
        }
    }
}
