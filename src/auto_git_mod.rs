// auto_git_mod

//! Functions to work with git from automation_tasks_rs

/// has git remote
pub fn git_has_remote() -> bool {
    // it returns only "origin" if exists or nothing if it does not exist
    let output = std::process::Command::new("git")
        .arg("remote")
        .output()
        .unwrap();
    // return
    String::from_utf8(output.stdout).unwrap() != ""
}

/// check if this folder is a local Git repository
pub fn git_is_local_repository() -> bool {
    let output = std::process::Command::new("git")
        .arg("status")
        .output()
        .unwrap();
    let output = String::from_utf8(output.stderr).unwrap();
    // return bool
    !output.contains("not a git repository")
}
