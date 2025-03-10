// auto_helper_functions_mod

//! various helper functions

use crate::public_api_mod::{RED, RESET};

// region: auto_md_to_doc_comments include doc_comments/exit_if_not_run_in_rust_project_root_directory.md A ///
/// Check if the code was run inside the Rust project root directory.  
///
/// There must be the `Cargo.toml` file and the directory `automation_tasks_rs`  
/// If not, exit with error message.  
///
// endregion: auto_md_to_doc_comments include doc_comments/exit_if_not_run_in_rust_project_root_directory.md A ///
pub fn exit_if_not_run_in_rust_project_root_directory() {
    if !(camino::Utf8Path::new("automation_tasks_rs").exists() && (camino::Utf8Path::new("Cargo.toml").exists())) {
        eprintln!("{RED}Error: `automation_tasks_rs` must be run inside the Rust project in the dir that contains");
        println!("`Cargo.toml` file and `automation_tasks_rs` directory. Exiting...{RESET}");
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
