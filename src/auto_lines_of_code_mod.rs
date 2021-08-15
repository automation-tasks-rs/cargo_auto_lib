// auto_lines_of_code_mod

use anyhow;
use regex::Regex;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, fs, path::Path};
use unwrap::unwrap;

use crate::auto_helper_functions_mod::*;

#[derive(Deserialize)]
struct CargoToml {
    workspace: Option<Workspace>,
}

#[derive(Deserialize)]
struct Workspace {
    members: Vec<String>,
}

#[derive(Default, Debug)]
/// Struct that contains 4 types of lines count: code, doc comments, comments, test and examples.
pub struct LinesOfCode {
    /// lines with code in srs files
    pub src_code_lines: usize,
    /// lines with doc_comments in srs files
    pub src_doc_comment_lines: usize,
    /// lines with comments in srs files
    pub src_comment_lines: usize,
    /// unit plus integration tests
    pub tests_lines: usize,
    /// all lines in examples files
    pub examples_lines: usize,
}

/// parameter Link to include in shield badge. If empty_string, the git remote repository will be used.
pub fn auto_lines_of_code(link: &str) {
    let text_to_include = text_to_include(link);
    include_into_readme_md(&text_to_include);
}
fn text_to_include(link: &str) -> String {
    let v = workspace_or_project_count_lines();

    let link = if link.is_empty() {
        process_git_remote()
    } else {
        link.to_string()
    };
    let text_to_include = to_string_as_shield_badges(&v, &link);
    // println!("{}", &text_to_include);
    // return
    text_to_include
}
/// Returns the struct LinesOfCode for 4 types of lines:
/// code, doc comments, comments, test and examples.
/// Automatically detects if this is a workspace or single rust project.
///
/// ## Example
///
/// ```
/// let v = cargo_auto_lib::auto_lines_of_code_mod::workspace_or_project_count_lines();
/// dbg!(&v);
/// ```
/// TODO: cargo-auto_lib could change the code to make some element visibility `pub` only for testing. And after return to normal.
fn workspace_or_project_count_lines() -> LinesOfCode {
    let mut lines_of_code = LinesOfCode::default();

    let current_dir = unwrap!(env::current_dir());
    println!(
        "{}current_dir: {}{}",
        *YELLOW,
        unwrap!(current_dir.to_str()),
        *RESET
    );

    // cargo toml contains the list of projects
    let cargo_toml = unwrap!(fs::read_to_string("Cargo.toml"));
    let cargo_toml: CargoToml = unwrap!(toml::from_str(&cargo_toml));
    if let Some(workspace) = cargo_toml.workspace {
        for member in workspace.members.iter() {
            println!("{}", &member);
            let v = one_project_count_lines(&current_dir.join(member));
            lines_of_code.src_code_lines += v.src_code_lines;
            lines_of_code.src_doc_comment_lines += v.src_doc_comment_lines;
            lines_of_code.src_comment_lines += v.src_comment_lines;
            lines_of_code.tests_lines += v.tests_lines;
            lines_of_code.examples_lines += v.examples_lines;
        }
    } else {
        lines_of_code = one_project_count_lines(&current_dir);
    }
    // return
    lines_of_code
}
/// Return the string for link for badges like: <https://github.com/LucianoBestia/auto_lines_of_code/>.  
/// Get the output string after $ git remote -v.  
/// Then finds out the link to the repository with regex.  
/// Returns empty string if something goes wrong: no git, no remote,...  
fn process_git_remote() -> String {
    let output = match git_remote_output() {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            return "".to_string();
        }
    };
    match regex_capture(output) {
        Ok(s) => return s,
        Err(e) => {
            println!("{}", e);
            return "".to_string();
        }
    }
}

/// private function. Use public workspace_or_project_count_lines().
fn one_project_count_lines(project_path: &Path) -> LinesOfCode {
    let mut lines_of_code = LinesOfCode::default();

    // src folder
    let files = unwrap!(crate::utils_mod::traverse_dir_with_exclude_dir(
        &project_path.join("src"),
        "/*.rs",
        // avoid big folders and other folders with *.crev
        &vec![
            "/.git".to_string(),
            "/target".to_string(),
            "/docs".to_string()
        ]
    ));
    // println!("{:#?}", files);
    for rs_file_name in files.iter() {
        //dbg!(&rs_file_name);
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(rs_file_name).unwrap();
        let reader = BufReader::new(file);
        let mut is_unit_test = false;
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for line in reader.lines() {
            let line = line.unwrap(); // Ignore errors.
            let line = line.trim_start();
            if line.starts_with("///") || line.starts_with("//!") {
                lines_of_code.src_doc_comment_lines += 1;
            } else if line.starts_with("//") || line.starts_with("/!") {
                lines_of_code.src_comment_lines += 1;
            } else if line.starts_with("#[cfg(test)]") {
                is_unit_test = true;
            } else if is_unit_test == true {
                lines_of_code.tests_lines += 1;
            } else {
                lines_of_code.src_code_lines += 1;
            }
        }
    }
    // tests folder
    let files = unwrap!(crate::utils_mod::traverse_dir_with_exclude_dir(
        &project_path.join("tests"),
        "/*.rs",
        // avoid big folders and other folders with *.crev
        &vec![
            "/.git".to_string(),
            "/target".to_string(),
            "/docs".to_string()
        ]
    ));
    // println!("{:#?}", files);
    for rs_file_name in files.iter() {
        //dbg!(&rs_file_name);
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(rs_file_name).unwrap();
        let reader = BufReader::new(file);
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for _line in reader.lines() {
            lines_of_code.tests_lines += 1;
        }
    }

    // examples folder
    let files = unwrap!(crate::utils_mod::traverse_dir_with_exclude_dir(
        &project_path.join("examples"),
        "/*.rs",
        // avoid big folders and other folders with *.crev
        &vec![
            "/.git".to_string(),
            "/target".to_string(),
            "/docs".to_string()
        ]
    ));
    for rs_file_name in files.iter() {
        //dbg!(&rs_file_name);
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(rs_file_name).unwrap();
        let reader = BufReader::new(file);
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for _line in reader.lines().enumerate() {
            lines_of_code.examples_lines += 1;
        }
    }
    //println!("{:#?}", &lines_of_code);
    // return
    lines_of_code
}
fn git_remote_output() -> anyhow::Result<String> {
    let output = std::process::Command::new("git")
        .arg("remote")
        .arg("-v")
        .output()?;

    let output = String::from_utf8(output.stdout)?;
    println!("output: {}", &output);
    // return
    Ok(output)
}
/// returns a Result.
/// in the case of error the calling fn will return empty string.
fn regex_capture(output: String) -> anyhow::Result<String> {
    // on Github actions they don't use ssh, but https, I need to check that also
    // I test my regex on https://regex101.com/
    // regex capture 3 groups: website, user_name and repo_name
    // "origin  git@github.com:LucianoBestia/auto_lines_of_code.git (fetch)"
    // origin    https://github.com/LucianoBestia/auto_lines_of_code (fetch)
    println!("{}", &output);
    let reg = Regex::new(
        r#"origin\s*(?:https://)?(?:git@)?([^:/]*?)[:/]([^/]*?)/([^. ]*?)(?:\.git)?\s*\(fetch\)"#,
    )?;
    let cap = reg
        .captures(&output)
        .ok_or(anyhow::anyhow!("Error: reg.captures is None"))?;
    // dbg!(&cap);

    // indexing can panic, but I would like it to Error
    anyhow::ensure!(
        cap.len() == 4,
        "Error: cap len is not 4, because there are 4 capture groups in regex."
    );
    return Ok(format!("https://{}/{}/{}/", &cap[1], &cap[2], &cap[3]));
}
/// Returns a string with the markdown code for 4 shield badges.
///
/// Every badge has the link to the url given as first parameter
/// or automatically finds out the github git remote repository url.
///
/// ## Example
///
/// ```
/// let v = cargo_auto_lib::auto_lines_of_code_mod::workspace_or_project_count_lines();
/// let badges = cargo_auto_lib::auto_lines_of_code_mod::to_string_as_shield_badges(&v,"");
///
/// println!("{}", badges);
/// ```
/// TODO: cargo-auto_lib could change the code to make some element visibility `pub` only for testing. And after return to normal.  
fn to_string_as_shield_badges(v: &LinesOfCode, link: &str) -> String {
    //println!("to_string_as_shield_badges() start");

    let src_code_lines = format!(
        "[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-{}-green.svg)]({})",
        v.src_code_lines, link
    );
    let src_doc_comment_lines = format!("[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-{}-blue.svg)]({})",v.src_doc_comment_lines,link);
    let src_comment_lines = format!(
        "[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-{}-purple.svg)]({})",
        v.src_comment_lines, link
    );
    let example_lines = format!(
        "[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-{}-yellow.svg)]({})",
        v.examples_lines, link
    );
    let tests_lines = format!(
        "[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-{}-orange.svg)]({})",
        v.tests_lines, link
    );
    //return
    format!(
        "{}\n{}\n{}\n{}\n{}\n",
        src_code_lines, src_doc_comment_lines, src_comment_lines, example_lines, tests_lines
    )
}

/// Includes (writes, modifies) the shield badge code into README.md file.
///
/// ## Example
///
/// ```
/// cargo_auto_lib::auto_lines_of_code_mod::include_into_readme_md("test test test");
/// ```
/// TODO: cargo-auto_lib could change the code to make some element visibility `pub` only for testing. And after return to normal.  
fn include_into_readme_md(include_str: &str) {
    let start_delimiter = "[comment]: # (auto_lines_of_code start)";
    let end_delimiter = "[comment]: # (auto_lines_of_code end)";
    let file_name = "README.md";

    if let Ok(readme_content) = fs::read_to_string(file_name) {
        let mut new_readme_content = String::with_capacity(readme_content.len());
        if let Some(mut pos_start) = readme_content.find(start_delimiter) {
            pos_start += start_delimiter.len();
            if let Some(pos_end) = readme_content.find(end_delimiter) {
                new_readme_content.push_str(&readme_content[..pos_start]);
                new_readme_content.push_str("\n");
                new_readme_content.push_str(include_str);
                new_readme_content.push_str("\n");
                new_readme_content.push_str(&readme_content[pos_end..]);
                println!(
                    "{}include_into_readme_md write file: {}{}",
                    *GREEN, file_name, *RESET
                );
                unwrap!(fs::write(file_name, new_readme_content));
            }
        }
    }
}
