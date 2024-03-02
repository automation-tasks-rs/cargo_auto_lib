// auto_cargo_toml_to_md_mod

//! includes data from Cargo.toml to README.md files: version, authors,...
//! It works for workspaces and for single projects.  

// region: use statements

use crate::public_api_mod::{GREEN, RED, RESET, YELLOW};
use chrono::Datelike;
use chrono::Utc;
use glob::glob;
use lazy_static::lazy_static;
use regex::*;

// this trait must be in scope to use these methods of CargoToml
use crate::public_api_mod::CargoTomlPublicApiMethods;

// endregion: use statements

lazy_static! {
    static ref REGEX_MD_START: Regex = Regex::new(r#"(?m)^\[//\]: # \(auto_cargo_toml_to_md start\)$"#).unwrap();
    static ref REGEX_MD_END: Regex = Regex::new(r#"(?m)^\[//\]: # \(auto_cargo_toml_to_md end\)$"#).unwrap();
}

#[doc=include_str!("../doc_comments_long/auto_cargo_toml_to_md.md")]
pub fn auto_cargo_toml_to_md() {
    println!("    {YELLOW}Running auto_cargo_toml_to_md{RESET}");
    let cargo_toml = crate::auto_cargo_toml_mod::CargoToml::read();
    let members = cargo_toml.workspace_members();
    match members {
        None => do_one_project(),
        Some(members) => {
            // this will read cargo.toml from the first `main` member and inject into workspace README.md
            do_one_project();
            for member in members.iter() {
                std::env::set_current_dir(member).unwrap();
                do_one_project();
                std::env::set_current_dir("..").unwrap();
            }
        }
    }
    println!("    {YELLOW}Finished auto_cargo_toml_to_md{RESET}");
}

fn do_one_project() {
    let cargo_toml = crate::auto_cargo_toml_mod::CargoToml::read();
    let version = cargo_toml.package_version();
    let author_name = cargo_toml.package_author_name();
    let homepage = cargo_toml.package_homepage();
    let repository = cargo_toml.package_repository().unwrap_or("".to_owned());
    let description = cargo_toml.package_description().unwrap_or("".to_owned());
    let keywords = cargo_toml.package_keywords().to_vec();
    let utc_now = &utc_now();

    let mut new_text = format!("\n**{description}**  \n");
    new_text.push_str(&format!(
        "***version: {version} date: {utc_now} author: [{author_name}]({homepage}) repository: [GitHub]({repository})***\n\n"
    ));

    for keyword in keywords.iter() {
        let color = if keyword == "work-in-progress" {
            "yellow"
        } else if keyword == "maintained" || keyword == "ready-for-use" {
            "green"
        } else if keyword == "obsolete" || keyword == "archived" {
            "red"
        } else {
            "orange"
        };
        // inside the shield badge syntax, hyphens must be replaced by underscore
        let keyword_underscore = keyword.replace('-', "_");
        new_text.push_str(&format!(" ![{keyword}](https://img.shields.io/badge/{keyword_underscore}-{color})\n"));
    }
    new_text.push_str("\n");

    for filename_result in glob("*.md").unwrap() {
        let filename_pathbuff = filename_result.unwrap();
        let md_filename = filename_pathbuff.to_str().unwrap();
        // println!("checking md_filename: {}", &md_filename);
        let mut md_text_content = std::fs::read_to_string(md_filename).unwrap();

        // check if file have CRLF and show error
        if md_text_content.contains("\r\n") {
            panic!(
                "{RED}Error: {} has CRLF line endings instead of LF. The task auto_cargo_toml_to_md cannot work! Exiting.{RESET}",
                &md_filename
            );
        }

        if let Some(cap) = REGEX_MD_START.captures(&md_text_content) {
            let pos_start = cap.get(0).unwrap().end() + 1;
            if let Some(cap) = REGEX_MD_END.captures(&md_text_content) {
                let pos_end = cap.get(0).unwrap().start();
                md_text_content.replace_range(pos_start..pos_end, &new_text);
                println!("    {YELLOW}Write to md file: {}{RESET}", md_filename);
                println!("{GREEN}{}{RESET}", &new_text.trim_end_matches("\n\n"));
                std::fs::write(md_filename, md_text_content).unwrap();
            }
        }
    }
}

/// utc now
fn utc_now() -> String {
    let now = Utc::now();
    format!("{:04}-{:02}-{:02}", now.year(), now.month() as i32, now.day() as i32,)
}
