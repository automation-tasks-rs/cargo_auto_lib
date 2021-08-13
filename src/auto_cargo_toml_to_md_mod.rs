// auto_cargo_toml_to_md

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Red, Yellow};
use chrono::Datelike;
use chrono::Utc;
use glob::glob;
use lazy_static::lazy_static;
use regex::*;
use std::fs;
use unwrap::unwrap;

lazy_static! {
    static ref REGEX_REMOVE_EMAIL: Regex = unwrap!(Regex::new(r#"( <.+?>)"#));
    static ref REGEX_VERSION: Regex = unwrap!(Regex::new(r#"(?m)^version = "(.+?)"$"#));
    static ref REGEX_AUTHORS: Regex = unwrap!(Regex::new(r#"(?m)^authors = \["(.+?)"\]$"#));
    static ref REGEX_REPOSITORY: Regex = unwrap!(Regex::new(r#"(?m)^repository = "(.+?)"$"#));
    static ref REGEX_DESCRIPTION: Regex = unwrap!(Regex::new(r#"(?m)^description = "(.+?)"$"#));
    static ref REGEX_MD_START: Regex =
        Regex::new(r#"(?m)^\[comment\]: # \(auto_cargo_toml_to_md start\)$"#).unwrap();
    static ref REGEX_MD_END: Regex =
        Regex::new(r#"(?m)^\[comment\]: # \(auto_cargo_toml_to_md end\)$"#).unwrap();
}

/// `auto_cargo_toml_to_md` Includes data from Cargo.toml to README.md files.  
/// version, authors, repository and description.  
pub fn auto_cargo_toml_to_md() {
    let cargo_toml_text = unwrap!(fs::read_to_string("Cargo.toml")); 
    // TODO: I could use the crate  cargo_toml  
    let cap = unwrap!(REGEX_VERSION.captures(&cargo_toml_text));
    let version = cap.get(1).unwrap().as_str();
    let cap = unwrap!(REGEX_AUTHORS.captures(&cargo_toml_text));
    let authors = cap.get(1).unwrap().as_str();
    let authors = REGEX_REMOVE_EMAIL.replace_all(authors, "");
    let cap = unwrap!(REGEX_REPOSITORY.captures(&cargo_toml_text));
    let repository = cap.get(1).unwrap().as_str();
    let cap = unwrap!(REGEX_DESCRIPTION.captures(&cargo_toml_text));
    let description = cap.get(1).unwrap().as_str();

    let new_text = format!(
        "\n**{}**  \n***[repo]({}); version: {}  date: {} authors: {}***  \n\n",
        description,
        repository,
        version,
        &utc_now(),
        authors,
    );
    println!("new text: '{}'", Green.paint(&new_text));

    for filename_result in unwrap!(glob("*.md")) {
        let filename_pathbuff = unwrap!(filename_result);
        let md_filename = unwrap!(filename_pathbuff.to_str());
        let mut md_text_content = unwrap!(fs::read_to_string(md_filename));

        if let Some(cap) = REGEX_MD_START.captures(&md_text_content) {
            let pos_start = unwrap!(cap.get(0)).end() + 1;
            if let Some(cap) = REGEX_MD_END.captures(&md_text_content) {
                let pos_end = unwrap!(cap.get(0)).start();
                md_text_content.replace_range(pos_start..pos_end, &new_text);
                println!("write md file: {}", Yellow.paint(md_filename));
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
