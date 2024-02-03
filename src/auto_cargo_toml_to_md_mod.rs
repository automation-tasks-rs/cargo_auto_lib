// auto_cargo_toml_to_md_mod

//! includes data from Cargo.toml to README.md files: version, authors,...
//! It works for workspaces and for single projects.  

// region: use statements

use chrono::Datelike;
use chrono::Utc;
use glob::glob;
use lazy_static::lazy_static;
use regex::*;
use std::fs;
use unwrap::unwrap;

use crate::auto_helper_functions_mod::*;

// endregion: use statements

lazy_static! {
    static ref REGEX_MD_START: Regex = unwrap!(Regex::new(
        r#"(?m)^\[//\]: # \(auto_cargo_toml_to_md start\)$"#
    ));
    static ref REGEX_MD_END: Regex = unwrap!(Regex::new(
        r#"(?m)^\[//\]: # \(auto_cargo_toml_to_md end\)$"#
    ));
}

/// includes data from Cargo.toml to README.md files: version, authors,...
/// It works for workspaces and for single projects.  
/// To avoid out of sync data like version, authors and description in the README.md files, `auto_cargo_toml_to_md` includes this data from Cargo.toml.  
/// Run it on every build with [cargo auto](https://crates.io/crates/cargo-auto).  
/// It works also with other md files in the project, not only README.md.  
/// In the md file write these markers in markdown comments (invisible),  
/// don't copy the numbers 1 and 2:  
///
/// ```markdown
/// 1 [//]: # (auto_cargo_toml_to_md start)
/// 2 [//]: # (auto_cargo_toml_to_md end)
/// ```
///
/// `auto_cargo_toml_to_md` deletes the old lines between the markers and includes the Cargo.toml data:  
/// description, repository, version, &utc_now(), authors  
///
/// Run the example:  
///
/// ```bash
/// cargo run --example example_01_auto_cargo_toml_to_md
/// ```  
pub fn auto_cargo_toml_to_md() {
    println!("    Running auto_cargo_toml_to_md");
    let cargo_toml = crate::auto_cargo_toml_mod::CargoToml::read();
    let members = cargo_toml.workspace_members();
    match members {
        None => do_one_project(),
        Some(members) => {
            // this will read cargo.toml from the first `main` member and inject into workspace README.md
            do_one_project();
            for member in members.iter() {
                unwrap!(std::env::set_current_dir(member));
                do_one_project();
                unwrap!(std::env::set_current_dir(".."));
            }
        }
    }
    println!("    Finished auto_cargo_toml_to_md");
}

fn do_one_project() {
    let cargo_toml = crate::auto_cargo_toml_mod::CargoToml::read();
    let version = cargo_toml.package_version();
    let author_name = cargo_toml.package_author_name();
    let homepage = cargo_toml.package_homepage();
    let repository = cargo_toml.package_repository().unwrap_or("".to_owned());
    let description = cargo_toml.package_description().unwrap_or("".to_owned());

    let new_text = format!(
        "\n**{}**  \n***version: {} date: {} author: [{}]({}) repository: [GitHub]({})***  \n\n",
        &description,
        &version,
        &utc_now(),
        &author_name,
        &homepage,
        &repository,
    );

    for filename_result in unwrap!(glob("*.md")) {
        let filename_pathbuff = unwrap!(filename_result);
        let md_filename = unwrap!(filename_pathbuff.to_str());
        // println!("checking md_filename: {}", &md_filename);
        let mut md_text_content = unwrap!(fs::read_to_string(md_filename));

        // check if file have CRLF and show error
        if md_text_content.contains("\r\n") {
            panic!("Error: {} has CRLF line endings instead of LF. The task auto_cargo_toml_to_md cannot work! Closing.", &md_filename);
        }

        if let Some(cap) = REGEX_MD_START.captures(&md_text_content) {
            let pos_start = unwrap!(cap.get(0)).end() + 1;
            if let Some(cap) = REGEX_MD_END.captures(&md_text_content) {
                let pos_end = unwrap!(cap.get(0)).start();
                md_text_content.replace_range(pos_start..pos_end, &new_text);
                println!("{YELLOW}write to md file: {}{RESET}", md_filename);
                println!("{GREEN}{}{RESET}", &new_text.trim_end_matches("\n\n"));
                unwrap!(fs::write(md_filename, md_text_content));
            }
        }
    }
}

/// utc now
fn utc_now() -> String {
    let now = Utc::now();
    format!(
        "{:04}-{:02}-{:02}",
        now.year(),
        now.month() as i32,
        now.day() as i32,
    )
}
