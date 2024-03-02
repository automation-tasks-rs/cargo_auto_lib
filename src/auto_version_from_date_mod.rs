// auto_version_from_date_mod

//! new version as date is written to Cargo.toml and service_worker.js
//! It works for workspaces and for single projects.  

// region: use statements

use crate::error_mod::{LibError, ResultWithLibError};
use crate::public_api_mod::{RED, RESET, YELLOW};
use chrono::DateTime;
use chrono::Timelike;
use chrono::{Datelike, Utc};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

// this trait must be in scope to use these methods of CargoToml
use crate::public_api_mod::CargoTomlPublicApiMethods;

// endregion: use statements

// region: structs
/// file metadata
#[derive(Serialize, Deserialize)]
pub struct FileMetaData {
    /// filename with path from Cargo.toml folder
    filename: String,
    /// hash of file
    filehash: String,
}

/// the struct that represents the file .automation_tasks_rs_file_hashes.json
#[derive(Serialize, Deserialize)]
pub struct AutoVersionFromDate {
    /// vector of file metadata
    pub vec_file_metadata: Vec<FileMetaData>,
}

// endregion: structs

// region: public functions

#[doc=include_str!("../doc_comments_long/auto_version_from_date.md")]
pub fn auto_version_from_date() {
    auto_version_from_date_internal(false).unwrap_or_else(|err| panic!("{RED}{err}{RESET}"));
}

/// Works for single projects and workspaces.  
/// Just like auto_version_from_date(), but force the new version even if no files are changed.
/// For workspaces `release` I want to have the same version in all members.  
/// It is slower, but easier to understand when deployed.
pub fn auto_version_from_date_forced() {
    auto_version_from_date_internal(true).unwrap_or_else(|err| panic!("{RED}{err}{RESET}"));
}

// endregion: public functions

// region: private functions

fn auto_version_from_date_internal(force_version: bool) -> ResultWithLibError<()> {
    let date = Utc::now();
    let new_version = version_from_date(date);
    let cargo_toml = crate::auto_cargo_toml_mod::CargoToml::read();
    let members = cargo_toml.workspace_members();
    match members {
        None => do_one_project(&new_version, force_version)?,
        Some(members) => {
            for member in members.iter() {
                std::env::set_current_dir(member).unwrap();
                do_one_project(&new_version, force_version)?;
                std::env::set_current_dir("..").unwrap();
            }
        }
    }
    modify_service_js(&new_version);
    Ok(())
}

fn do_one_project(new_version: &str, force_version: bool) -> ResultWithLibError<()> {
    let vec_of_metadata = read_file_metadata()?;
    let is_files_equal = if force_version {
        false
    } else {
        let js_struct = read_json_file(".automation_tasks_rs_file_hashes.json")?;
        are_files_equal(&vec_of_metadata, &js_struct.vec_file_metadata)
    };

    if !is_files_equal {
        write_version_to_cargo_and_modify_metadata(new_version, vec_of_metadata)?;
    }
    Ok(())
}

/// search for file service_worker.js and modify version
fn modify_service_js(new_version: &str) {
    let start_dir = camino::Utf8Path::new("./");
    for js_filename in &crate::utils_mod::traverse_dir_with_exclude_dir(start_dir.as_std_path(), "/service_worker.js", &["/.git".to_string(), "/target".to_string()]).unwrap() {
        // println!("{}write version in {}{}", *GREEN, js_filename, *RESET);
        let mut js_content = std::fs::read_to_string(js_filename).unwrap();

        // check if file have CRLF instead of LF and show error
        if js_content.contains("\r\n") {
            panic!("{RED}Error: {js_filename} has CRLF line endings instead of LF. Correct the file! Exiting...{RESET}");
        }

        let delimiter = r#"const CACHE_NAME = '"#;
        let delimiter_len = delimiter.len();
        let option_location = js_content.find(delimiter);
        if let Some(location) = option_location {
            let start_version = location + delimiter_len;
            let option_end_quote = find_from(js_content.as_str(), start_version, r#"';"#);
            if let Some(end_version) = option_end_quote {
                //delete all the characters in between the markers
                let old_version: String = js_content.drain(start_version..end_version).collect();
                //println!(r#"old version: "{}""#, old_version.as_str());
                if new_version != old_version {
                    println!("    {YELLOW}Modify version: {old_version} -> {new_version}{RESET}");
                    js_content.insert_str(start_version, new_version);
                    //println!("{}write file: {}{}", *YELLOW, js_filename, *RESET);
                    let _x = std::fs::write(js_filename, js_content);
                }
            } else {
                panic!("{RED}no end quote for version{RESET}");
            }
        } else {
            panic!("{RED}service_worker.js has no version{RESET}");
        }
    }
}

/// move vec_of_metadata
fn write_version_to_cargo_and_modify_metadata(new_version: &str, mut vec_of_metadata: Vec<FileMetaData>) -> ResultWithLibError<()> {
    // println!("{}write version to Cargo.toml{}", *GREEN, *RESET);
    let cargo_filename = "Cargo.toml";
    let mut cargo_content = std::fs::read_to_string(cargo_filename).unwrap();

    // check if file have CRLF instead of LF and show error
    if cargo_content.contains("\r\n") {
        panic!("{RED}Error: {} has CRLF line endings instead of LF. Correct the file! Exiting...{RESET}", cargo_filename);
    }

    let delimiter = r#"version = ""#;
    let delimiter_len = delimiter.len();
    let option_location = cargo_content.find(delimiter);
    if let Some(location) = option_location {
        let start_version = location + delimiter_len;
        let option_end_quote = find_from(cargo_content.as_str(), start_version, r#"""#);
        if let Some(end_version) = option_end_quote {
            //delete all the characters in between the markers
            let old_version: String = cargo_content.drain(start_version..end_version).collect();
            //println!(r#"old version: "{}""#, old_version.as_str());
            if new_version != old_version.as_str() {
                println!("    {YELLOW}Modify version: {old_version} -> {new_version}{RESET}");
                cargo_content.insert_str(start_version, new_version);
                // println!("{}write file: {}{}", *YELLOW, cargo_filename, *RESET);
                let _x = std::fs::write(cargo_filename, cargo_content);

                //the Cargo.toml is now different
                correct_file_metadata_for_cargo_tom_inside_vec(&mut vec_of_metadata)?;
                save_json_file_for_file_meta_data(vec_of_metadata);
            }
        } else {
            panic!("{RED}no end quote for version{RESET}");
        }
    } else {
        panic!("{RED}Cargo.toml has no version{RESET}");
    }
    Ok(())
}

/// the Cargo.toml is now different and needs to be changed in the vec of file metadata
pub fn correct_file_metadata_for_cargo_tom_inside_vec(vec_of_metadata: &mut [FileMetaData]) -> ResultWithLibError<()> {
    //correct the vector only for Cargo.toml file
    let filename = "Cargo.toml".to_string();
    // calculate hash of file
    let filehash = sha256_digest(std::path::PathBuf::from_str(&filename)?.as_path())?;
    vec_of_metadata.get_mut(0).ok_or(LibError::ErrorFromStr("error vec_of_metadata.get_mut(0)"))?.filehash = filehash;
    Ok(())
}

/// if files are added or deleted, other files must be also changed
/// I need to check if the files on the filesystem are the same as in the json
pub fn are_files_equal(vec_of_metadata: &[FileMetaData], js_vec_of_metadata: &[FileMetaData]) -> bool {
    let mut is_files_equal = true;
    for x in vec_of_metadata.iter() {
        //search in json file
        let mut is_one_equal = false;
        for y in js_vec_of_metadata.iter() {
            if x.filename == y.filename && x.filehash == y.filehash {
                is_one_equal = true;
                break;
            }
        }
        if !is_one_equal {
            // println!("{} {}", x.filename, x.filehash);
            is_files_equal = false;
            break;
        }
    }
    is_files_equal
}

/// make a vector of file metadata
pub fn read_file_metadata() -> ResultWithLibError<Vec<FileMetaData>> {
    let mut vec_of_metadata: Vec<FileMetaData> = Vec::new();
    let filename = "Cargo.toml".to_string();
    // calculate hash of file
    let filehash = sha256_digest(std::path::PathBuf::from_str(&filename)?.as_path())?;
    vec_of_metadata.push(FileMetaData { filename, filehash });

    let files_paths = crate::utils_mod::traverse_dir_with_exclude_dir(
        camino::Utf8Path::new("src").as_std_path(),
        "/*.rs",
        // avoid big folders and other folders with *.crev
        &[],
    )
    .unwrap();

    for filename in files_paths {
        // calculate hash of file
        let filehash = sha256_digest(std::path::PathBuf::from_str(&filename)?.as_path())?;
        vec_of_metadata.push(FileMetaData { filename, filehash });
    }
    Ok(vec_of_metadata)
}

/// calculate the hash for a file
fn sha256_digest(path: &std::path::Path) -> ResultWithLibError<String> {
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let mut context = ring::digest::Context::new(&ring::digest::SHA256);
    let mut buffer = [0; 1024];
    use std::io::Read;
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    let digest = context.finish();
    let hash_string = data_encoding::HEXLOWER.encode(digest.as_ref());
    // return
    Ok(hash_string)
}

/// read .automation_tasks_rs_file_hashes.json
pub fn read_json_file(json_filepath: &str) -> ResultWithLibError<AutoVersionFromDate> {
    let js_struct: AutoVersionFromDate;
    let f = std::fs::read_to_string(json_filepath);

    match f {
        Ok(x) => {
            // check if file have CRLF instead of LF. This are unusable - create empty struct
            if x.contains("\r\n") {
                //create empty struct
                js_struct = AutoVersionFromDate { vec_file_metadata: Vec::new() }
            } else {
                //read struct from file
                js_struct = serde_json::from_str(x.as_str())?;
            }
        }
        Err(_error) => {
            // println!("Creating new file: {}", json_filepath);
            //create empty struct
            js_struct = AutoVersionFromDate { vec_file_metadata: Vec::new() }
        }
    };
    Ok(js_struct)
}

/// save the new file metadata
pub fn save_json_file_for_file_meta_data(vec_of_metadata: Vec<FileMetaData>) {
    let x = AutoVersionFromDate { vec_file_metadata: vec_of_metadata };
    let y = serde_json::to_string_pretty(&x).unwrap();
    let json_filepath = ".automation_tasks_rs_file_hashes.json";
    let _f = std::fs::write(json_filepath, y);
}

/// converts a date to a version
fn version_from_date(date: DateTime<Utc>) -> String {
    // in Rust the version must not begin with zero.
    // There is an exceptional situation where is midnight 00.
    //return
    if date.hour() == 0 {
        format!("{:04}.{}{:02}.{}", date.year(), date.month(), date.day(), date.minute())
    } else {
        format!("{:04}.{}{:02}.{}{:02}", date.year(), date.month(), date.day(), date.hour(), date.minute())
    }
}

/// in string find from position
fn find_from(rs_content: &str, from: usize, find: &str) -> Option<usize> {
    let slice01 = rs_content.get(from..).unwrap();
    let option_location = slice01.find(find);
    if let Some(location) = option_location {
        //return Option with usize
        Some(from + location)
    } else {
        //return Option with none
        option_location
    }
}

// endregion: private functions

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_date_to_version() {
        let date_time = chrono::TimeZone::with_ymd_and_hms(&Utc, 2020, 5, 22, 00, 34, 0).unwrap();

        let version = version_from_date(date_time);
        assert_eq!(version, "2020.522.34");
    }

    #[test]
    pub fn test_sha256_digest() -> ResultWithLibError<()> {
        let digest = sha256_digest(camino::Utf8Path::new("LICENSE").as_std_path())?;
        let hash_string = data_encoding::HEXLOWER.encode(digest.as_ref());
        let expected_hex = "65666234343064623966363462343963643663333839373166303131663632636334666364323733663461323635303161346336366139633935656337373139";
        assert_eq!(&hash_string, expected_hex);
        Ok(())
    }
}
