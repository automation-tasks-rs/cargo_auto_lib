// auto_semver_mod

//! semver utilities

use crate::utils_mod::*;

use std::fs;
use unwrap::unwrap;

use crate::auto_helper_functions_mod::*;

enum VersionPart {
    Patch,
    Minor,
}

/// Increments the patch version in Cargo.toml file only if files are changed
pub fn auto_semver_increment_patch() {
    increment_part(VersionPart::Patch, false);
}

/// Increments the patch version in Cargo.toml file even if files are not changed
pub fn auto_semver_increment_patch_forced() {
    increment_part(VersionPart::Patch, true);
}

/// Increments the minor version in Cargo.toml file only if files are changed
pub fn auto_semver_increment_minor() {
    increment_part(VersionPart::Minor, false);
}

/// Increments the minor version in Cargo.toml file even if files are not changed
pub fn auto_semver_increment_minor_forced() {
    increment_part(VersionPart::Minor, true);
}

fn increment_part(part: VersionPart, force_version: bool) {
    let mut vec_of_metadata = crate::auto_version_from_date_mod::read_file_metadata();
    let is_files_equal = if force_version {
        false
    } else {
        let js_struct =
            crate::auto_version_from_date_mod::read_json_file(".auto_version_from_date.json");
        crate::auto_version_from_date_mod::are_files_equal(
            &vec_of_metadata,
            &js_struct.vec_file_metadata,
        )
    };

    if !is_files_equal {
        // println!("pub fn increment_patch");
        let cargo_toml_filename = "Cargo.toml";
        let cargo_toml_text = unwrap!(fs::read_to_string(cargo_toml_filename));

        // check if file have CRLF instead of LF and show error
        if cargo_toml_text.contains("\r\n") {
            panic!("Error: {} has CRLF line endings instead of LF. The task auto_semver_increment cannot work! Closing.", cargo_toml_filename);
        }

        // find the line with "version = " including the start quote
        if let Some(pos_start_data) =
            find_pos_start_data_after_delimiter(&cargo_toml_text, 0, r#"version = ""#)
        {
            // find the end quote
            if let Some(pos_end_data) =
                find_pos_end_data_before_delimiter(&cargo_toml_text, pos_start_data, r#"""#)
            {
                let version = cargo_toml_text[pos_start_data..pos_end_data].to_string();
                println!(r#"    old version: "{}""#, &version);
                //increment the last number
                let pos = pos_start_data;
                let (major, pos) = parse_next_number(&cargo_toml_text, pos);
                //jump over dot
                let pos = pos + 1;
                let (mut minor, pos) = parse_next_number(&cargo_toml_text, pos);
                //jump over dot
                let pos = pos + 1;
                let (mut patch, pos) = parse_next_number(&cargo_toml_text, pos);
                let pos_at_the_end_of_semver = pos;
                // increment
                match part {
                    VersionPart::Patch => {
                        patch += 1;
                    }
                    VersionPart::Minor => {
                        minor += 1;
                        patch = 0;
                    }
                }
                // println!(r#"major: {},minor: {}, patch: {}"#, major, minor, patch);
                let new_semver = format!("{}.{}.{}", major, minor, patch);
                println!("    {GREEN}new version: '{}'{RESET}", &new_semver);
                let new_cargo_toml_text = format!(
                    "{}{}{}",
                    &cargo_toml_text[..pos_start_data],
                    &new_semver,
                    &cargo_toml_text[pos_at_the_end_of_semver..]
                );
                //save the file
                let _x = fs::write(cargo_toml_filename, new_cargo_toml_text);

                //the Cargo.toml is now different
                crate::auto_version_from_date_mod::correct_file_metadata_for_cargo_tom_inside_vec(
                    &mut vec_of_metadata,
                );
                crate::auto_version_from_date_mod::save_json_file_for_file_meta_data(
                    vec_of_metadata,
                );
            } else {
                panic!("no end quote for version");
            }
        } else {
            panic!("Cargo.toml has no version");
        }
    }
}

fn parse_next_number(cargo_toml_text: &str, pos: usize) -> (usize, usize) {
    let mut pos = pos;
    let mut number = "".to_string();
    let mut one_char = cargo_toml_text[pos..pos + 1].chars().next().unwrap();
    while one_char.is_numeric() {
        number.push(one_char);
        pos += 1;
        one_char = cargo_toml_text[pos..pos + 1].chars().next().unwrap();
    }
    let number: usize = unwrap!(number.parse());
    //return
    (number, pos)
}
