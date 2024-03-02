// auto_lines_of_code_mod

//! inserts shield badges with lines_of_code into README.rs

use crate::public_api_mod::{RED, RESET, YELLOW};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

// use crate::auto_helper_functions_mod::*;
// this trait must be in scope to use these methods of CargoToml
use crate::public_api_mod::CargoTomlPublicApiMethods;

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

// region: auto_md_to_doc_comments include doc_comments_long/auto_lines_of_code.md A ///
/// This function inserts shield badges with lines_of_code into README.rs.  
///
/// The parameter Link will be used for shield badge. If empty_string, the git remote repository will be used.  
/// Lines of code are not a "perfect" measurement of anything.\
/// Anybody can write a very big number of lines of useless code and comments.\
/// But for 95% of the cases they are good enough.\
/// Most of the developers use some "standard" coding practices and that is quantifiable and comparable.  
///
/// The `src_code_lines` is the most important count.\
/// That is actual code written for that project without  doc comments, comments, unit tests, integration tests and examples.\
/// Sometimes is great to see a big number here. It means there was a lot of work invested. But other times we want to see a small number. It
/// means the developer understands the problem very well and don't try to solve anything outside that scope.  
/// The `src_doc_comment_lines` counts doc comments. They will eventually become docs. The count of lines shows how many documentation is written.  
/// The `src_comment_lines` counts code comments. Code comments are important to understand the code. The count of lines shows how understandable is the code.  
/// The `tests_lines` counts lines in tests and shows how good is the code tested. Here are the unit tests and integration test combined.  
/// The `examples_lines` counts lines in examples and shows how good is explained how to use the code.  
///
///
/// ## Folder and file structure
///
/// The folder structure of a single Rust project is simple.\
/// The project starts in the folder that contains `Cargo.toml`.\
/// The `src/` folder contains all the rust `*.rs` files.\
/// The `tests/` folder contains integration tests.\
/// The `examples/` folder contains examples.\
/// Some rs files can be excluded from the count adding this line near the start of the file: // exclude from auto_lines_of_code
/// Inside a rs file the doc comment line start with `///` or `//!`.\
/// The normal comments start with `//` or `/!`.\
/// I will ignore the block comments. They are usually NOT used for comments, but to temporarily disable a piece of code. So I count this as code and not comments.  
/// The `src/*.rs` file can contain unit tests that start with `#[cfg(test)]`. I assume that these are always at the end of the file.  
/// There should not be any normal code after `#[cfg(test)]`, only tests.  
/// All other files: `md`, `toml`, `html`, `js`, ... are not counted.  
///
/// ### Workspace
///
/// Workspaces have member projects, that are written in `Cargo.toml`.\
/// The program counts lines of every project and sums them together.  
///
/// ## Include into README.md
///
/// If the README.md file contains these markers (don't copy the numbers 1 and 2):  
///
/// ```md
/// [comment]: # (auto_lines_of_code start)
///
/// [comment]: # (auto_lines_of_code end)
/// ```
///
/// In your markdown, change the word `[comment]` with double slash `[//]`.  
///
/// The function will include the shield badges code between them.  
/// It will erase the previous content.  
/// Use git diff to see the change.  
///
// endregion: auto_md_to_doc_comments include doc_comments_long/auto_lines_of_code.md A ///
pub fn auto_lines_of_code(link: &str) {
    println!("    {YELLOW}Running auto_lines_of_code{RESET}");
    let link = if link.is_empty() { process_git_remote() } else { link.to_string() };
    // Cargo.toml contains the list of projects
    let cargo_toml = crate::auto_cargo_toml_mod::CargoToml::read();
    match cargo_toml.workspace_members() {
        None => {
            let v = one_project_count_lines();
            let text_to_include = to_string_as_shield_badges(&v, &link);
            include_into_readme_md(&text_to_include);
        }
        Some(members) => {
            let mut lines_of_code = LinesOfCode::default();
            for member in members.iter() {
                println!("    {YELLOW}Member: {member}{RESET}");
                std::env::set_current_dir(member).unwrap();
                let v = one_project_count_lines();
                let text_to_include = to_string_as_shield_badges(&v, &link);
                include_into_readme_md(&text_to_include);
                std::env::set_current_dir("..").unwrap();

                lines_of_code.src_code_lines += v.src_code_lines;
                lines_of_code.src_doc_comment_lines += v.src_doc_comment_lines;
                lines_of_code.src_comment_lines += v.src_comment_lines;
                lines_of_code.tests_lines += v.tests_lines;
                lines_of_code.examples_lines += v.examples_lines;
            }
            // the workspace README.md
            let text_to_include = to_string_as_shield_badges(&lines_of_code, &link);
            include_into_readme_md(&text_to_include);
        }
    }
    println!("    {YELLOW}Finished auto_lines_of_code{RESET}");
}

/// Return the string for link for badges like: <https://github.com/bestia-dev/auto_lines_of_code/>.  
///
/// Get the output string after $ git remote -v.  
/// Then finds out the link to the repository with regex.  
/// Returns empty string if something goes wrong: no git, no remote,...  
fn process_git_remote() -> String {
    let output = match git_remote_output() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{RED}{e}{RESET}");
            return "".to_string();
        }
    };
    match regex_capture(output) {
        Ok(s) => s,
        Err(_e) => {
            // eprintln!("{RED}process_git_remote error: {}{RESET}", e);
            "".to_string()
        }
    }
}

/// private function. Use public workspace_or_project_count_lines().
fn one_project_count_lines() -> LinesOfCode {
    let mut lines_of_code = LinesOfCode::default();

    // src folder
    let files = crate::utils_mod::traverse_dir_with_exclude_dir(
        camino::Utf8Path::new("src").as_std_path(),
        "/*.rs",
        // avoid big folders and other folders with *.crev
        &["/.git".to_string(), "/target".to_string(), "/docs".to_string()],
    )
    .unwrap();
    // println!("{:#?}", files);
    for rs_file_name in files.iter() {
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(rs_file_name).unwrap();
        let reader = BufReader::new(file);
        let mut is_unit_test = false;
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for line in reader.lines() {
            let line = line.unwrap(); // Ignore errors.
            let line = line.trim_start();
            if line == "// exclude from auto_lines_of_code" {
                break;
            }
            if line.starts_with("///") || line.starts_with("//!") {
                lines_of_code.src_doc_comment_lines += 1;
            } else if line.starts_with("//") || line.starts_with("/!") {
                lines_of_code.src_comment_lines += 1;
            } else if line.starts_with("#[cfg(test)]") {
                is_unit_test = true;
            } else if is_unit_test {
                lines_of_code.tests_lines += 1;
            } else {
                lines_of_code.src_code_lines += 1;
            }
        }
    }
    // tests folder
    let files = crate::utils_mod::traverse_dir_with_exclude_dir(
        camino::Utf8Path::new("tests").as_std_path(),
        "/*.rs",
        // avoid big folders and other folders with *.crev
        &["/.git".to_string(), "/target".to_string(), "/docs".to_string()],
    )
    .unwrap();
    // println!("{:#?}", files);
    for rs_file_name in files.iter() {
        // Open the file in read-only mode (ignoring errors).
        let file = File::open(rs_file_name).unwrap();
        let reader = BufReader::new(file);
        // Read the file line by line using the lines() iterator from std::io::BufRead.
        for _line in reader.lines() {
            lines_of_code.tests_lines += 1;
        }
    }

    // examples folder
    let files = crate::utils_mod::traverse_dir_with_exclude_dir(
        camino::Utf8Path::new("examples").as_std_path(),
        "/*.rs",
        // avoid big folders and other folders with *.crev
        &["/.git".to_string(), "/target".to_string(), "/docs".to_string()],
    )
    .unwrap();
    for rs_file_name in files.iter() {
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
    let output = std::process::Command::new("git").arg("remote").arg("-v").output()?;

    let output = String::from_utf8(output.stdout)?;
    // return
    Ok(output)
}

/// returns a Result.
/// in the case of error the calling fn will return empty string.
fn regex_capture(output: String) -> anyhow::Result<String> {
    // on GitHub actions they don't use ssh, but https, I need to check that also
    // I test my regex on https://regex101.com/
    // regex capture 3 groups: website, user_name and repo_name
    // "origin  git@github.com:bestia-dev/auto_lines_of_code.git (fetch)"
    // origin    https://github.com/bestia-dev/auto_lines_of_code (fetch)
    // println!("{}", &output);
    let reg = Regex::new(r#"origin\s*(?:https://)?(?:git@)?([^:/]*?)[:/]([^/]*?)/([^. ]*?)(?:\.git)?\s*\(fetch\)"#)?;
    let cap = reg.captures(&output).ok_or(anyhow::anyhow!("Error: reg.captures is None"))?;

    // indexing can panic, but I would like it to Error
    anyhow::ensure!(cap.len() == 4, "Error: cap len is not 4, because there are 4 capture groups in regex.");
    Ok(format!("https://{}/{}/{}/", &cap[1], &cap[2], &cap[3]))
}

/// Returns a string with the markdown code for 4 shield badges.
///
/// Every badge has the link to the url given as first parameter
/// or automatically finds out the github git remote repository url.
///
/// let v = cargo_auto_lib::auto_lines_of_code_mod::workspace_or_project_count_lines();
/// let badges = cargo_auto_lib::auto_lines_of_code_mod::to_string_as_shield_badges(&v,"");
///
/// println!("{}", badges);
/// TODO: cargo-auto_lib could change the code to make some element visibility `pub` only for testing. And after return to normal.  
fn to_string_as_shield_badges(v: &LinesOfCode, link: &str) -> String {
    //println!("to_string_as_shield_badges() start");

    let src_code_lines = format!("[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-{}-green.svg)]({})", v.src_code_lines, link);
    let src_doc_comment_lines = format!(
        "[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-{}-blue.svg)]({})",
        v.src_doc_comment_lines, link
    );
    let src_comment_lines = format!("[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-{}-purple.svg)]({})", v.src_comment_lines, link);
    let example_lines = format!("[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-{}-yellow.svg)]({})", v.examples_lines, link);
    let tests_lines = format!("[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-{}-orange.svg)]({})", v.tests_lines, link);
    //return
    format!("{}\n{}\n{}\n{}\n{}\n", src_code_lines, src_doc_comment_lines, src_comment_lines, example_lines, tests_lines)
}

/// Includes (writes, modifies) the shield badge code into README.md file.
/// include_into_readme_md("test test test");
fn include_into_readme_md(include_str: &str) {
    let start_delimiter = "[//]: # (auto_lines_of_code start)";
    let end_delimiter = "[//]: # (auto_lines_of_code end)";
    let file_name = "README.md";

    if let Ok(readme_content) = std::fs::read_to_string(file_name) {
        // check if file have CRLF instead of LF and show error
        if readme_content.contains("\r\n") {
            panic!("{RED}Error: {} has CRLF line endings instead of LF. Correct the file! Exiting...{RESET}", file_name);
        }

        let mut new_readme_content = String::with_capacity(readme_content.len());
        if let Some(mut pos_start) = readme_content.find(start_delimiter) {
            pos_start += start_delimiter.len();
            if let Some(pos_end) = readme_content.find(end_delimiter) {
                new_readme_content.push_str(&readme_content[..pos_start]);
                new_readme_content.push('\n');
                new_readme_content.push_str(include_str);
                new_readme_content.push('\n');
                new_readme_content.push_str(&readme_content[pos_end..]);
                /*
                println!(
                    "{}include_into_readme_md write file: {}{}",
                    *GREEN, file_name, *RESET
                );
                 */
                std::fs::write(file_name, new_readme_content).unwrap();
            }
        }
    }
}
