// auto_version_from_date_mod

//! new version as date is written to Cargo.toml and service_worker.js
//! It works for workspaces and for single projects.  

// region: use statements

use crate::error_mod::ResultWithLibError;
use crate::{LibError, RED, RESET};
use chrono::DateTime;
use chrono::Timelike;
use chrono::{Datelike, Utc};
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs, path::Path};
use unwrap::unwrap;

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

/// Works for single projects and workspaces.  
/// new version as date is written to Cargo.toml and service_worker.js
/// In Cargo.toml writes the version as the date `yyyy.mmdd.HHMM` ex. `2019.1221.2359`.  
/// For non-library projects, the semver specification is not really useful.  
/// Having the version as the date is just fine for executables and much more human readable.  
/// The function must be executed in the root project folder of a single project or workspace where is the Cargo.toml.  
///
/// ### service_worker.js
///
/// Inside the PWA service worker javascript file is also needed to change the version.  
/// The program searches for `service_worker.js` and modify the version.  
///
/// ### no need to change version if no files changed
///
/// If src/*.rs or Cargo.toml files are not changed from last compile, than no need to change version.  
/// The dates of the files will be stored in the file .automation_tasks_rs_file_hashes.json near to Cargo.toml.
/// Warning: I don't check if the service worker has changed because it rarely does.  
/// To know if the projects has changed or not, this function saves the dates of all files into `.automation_tasks_rs_file_hashes.json` near Cargo.toml
pub fn auto_version_from_date() {
    auto_version_from_date_internal(false).unwrap_or_else(|err| panic!("{}", err.to_string()));
}

/// Works for single projects and workspaces.  
/// Just like auto_version_from_date(), but force the new version even if no files are changed.
/// For workspaces `release` I want to have the same version in all members.  
/// It is slower, but easier to understand when deployed.
pub fn auto_version_from_date_forced() {
    auto_version_from_date_internal(true).unwrap_or_else(|err| panic!("{}", err.to_string()));
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
                unwrap!(std::env::set_current_dir(member));
                do_one_project(&new_version, force_version)?;
                unwrap!(std::env::set_current_dir(".."));
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
    let start_dir = Path::new("./");
    for js_filename in &unwrap!(crate::utils_mod::traverse_dir_with_exclude_dir(
        start_dir,
        "/service_worker.js",
        &vec!["/.git".to_string(), "/target".to_string()]
    )) {
        // println!("{}write version in {}{}", *GREEN, js_filename, *RESET);
        let mut js_content = unwrap!(fs::read_to_string(js_filename));

        // check if file have CRLF instead of LF and show error
        if js_content.contains("\r\n") {
            panic!("Error: {} has CRLF line endings instead of LF. The task modify_service_js cannot work! Closing.", js_filename);
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
                    println!("    Modify version: {} -> {}", old_version, new_version);
                    js_content.insert_str(start_version, new_version);
                    //println!("{}write file: {}{}", *YELLOW, js_filename, *RESET);
                    let _x = fs::write(js_filename, js_content);
                }
            } else {
                panic!("no end quote for version");
            }
        } else {
            panic!("service_worker.js has no version");
        }
    }
}

/// move vec_of_metadata
fn write_version_to_cargo_and_modify_metadata(
    new_version: &str,
    mut vec_of_metadata: Vec<FileMetaData>,
) -> ResultWithLibError<()> {
    // println!("{}write version to Cargo.toml{}", *GREEN, *RESET);
    let cargo_filename = "Cargo.toml";
    let mut cargo_content = unwrap!(fs::read_to_string(cargo_filename));

    // check if file have CRLF instead of LF and show error
    if cargo_content.contains("\r\n") {
        panic!("{RED}Error: {} has CRLF line endings instead of LF. The task write_version_to_cargo cannot work! Closing.{RESET}", cargo_filename);
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
                println!("    Modify version: {} -> {}", old_version, new_version);
                cargo_content.insert_str(start_version, new_version);
                // println!("{}write file: {}{}", *YELLOW, cargo_filename, *RESET);
                let _x = fs::write(cargo_filename, cargo_content);

                //the Cargo.toml is now different
                correct_file_metadata_for_cargo_tom_inside_vec(&mut vec_of_metadata)?;
                save_json_file_for_file_meta_data(vec_of_metadata);
            }
        } else {
            panic!("no end quote for version");
        }
    } else {
        panic!("Cargo.toml has no version");
    }
    Ok(())
}

/// the Cargo.toml is now different and needs to be changed in the vec of file metadata
pub fn correct_file_metadata_for_cargo_tom_inside_vec(
    vec_of_metadata: &mut Vec<FileMetaData>,
) -> ResultWithLibError<()> {
    //correct the vector only for Cargo.toml file
    let filename = "Cargo.toml".to_string();
    // calculate hash of file
    let filehash = sha256_digest(PathBuf::from_str(&filename)?.as_path())?;
    vec_of_metadata
        .get_mut(0)
        .ok_or(LibError::ErrorFromStr("error vec_of_metadata.get_mut(0)"))?
        .filehash = filehash;
    Ok(())
}

/// if files are added or deleted, other files must be also changed
/// I need to check if the files on the filesystem are the same as in the json
pub fn are_files_equal(
    vec_of_metadata: &Vec<FileMetaData>,
    js_vec_of_metadata: &Vec<FileMetaData>,
) -> bool {
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
            // println!("{} {}", x.filename, x.filedate);
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
    let filehash = sha256_digest(PathBuf::from_str(&filename)?.as_path())?;
    vec_of_metadata.push(FileMetaData { filename, filehash });

    for entry in fs::read_dir("src")? {
        let entry = unwrap!(entry);
        let path = entry.file_name();

        let filename = format!("src/{:?}", path);
        let filename = filename.replace("\"", "");
        // calculate hash of file
        let filehash = sha256_digest(PathBuf::from_str(&filename)?.as_path())?;
        vec_of_metadata.push(FileMetaData { filename, filehash });
    }
    Ok(vec_of_metadata)
}

/// calculate the hash for a file
fn sha256_digest(path: &Path) -> ResultWithLibError<String> {
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
    let f = fs::read_to_string(json_filepath);

    match f {
        Ok(x) => {
            // check if file have CRLF instead of LF. This are unusable - create empty struct
            if x.contains("\r\n") {
                //create empty struct
                js_struct = AutoVersionFromDate {
                    vec_file_metadata: Vec::new(),
                }
            } else {
                //read struct from file
                js_struct = serde_json::from_str(x.as_str())?;
            }
        }
        Err(_error) => {
            // println!("Creating new file: {}", json_filepath);
            //create empty struct
            js_struct = AutoVersionFromDate {
                vec_file_metadata: Vec::new(),
            }
        }
    };
    Ok(js_struct)
}

/// save the new file metadata
pub fn save_json_file_for_file_meta_data(vec_of_metadata: Vec<FileMetaData>) {
    let x = AutoVersionFromDate {
        vec_file_metadata: vec_of_metadata,
    };
    let y = unwrap!(serde_json::to_string(&x));
    let json_filepath = ".automation_tasks_rs_file_hashes.json";
    let _f = fs::write(json_filepath, y);
}

/// converts a date to a version
fn version_from_date(date: DateTime<Utc>) -> String {
    // in Rust the version must not begin with zero.
    // There is an exceptional situation where is midnight 00.
    let new_version = if date.hour() == 0 {
        format!(
            "{:04}.{}{:02}.{}",
            date.year(),
            date.month(),
            date.day(),
            date.minute()
        )
    } else {
        format!(
            "{:04}.{}{:02}.{}{:02}",
            date.year(),
            date.month(),
            date.day(),
            date.hour(),
            date.minute()
        )
    };
    //return
    new_version
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
        let digest = sha256_digest(Path::new("LICENSE"))?;
        let hash_string = data_encoding::HEXLOWER.encode(digest.as_ref());
        let expected_hex = "65666234343064623966363462343963643663333839373166303131663632636334666364323733663461323635303161346336366139633935656337373139";
        assert_eq!(&hash_string, expected_hex);
        Ok(())
    }
}
