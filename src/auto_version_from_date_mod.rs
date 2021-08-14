//! auto_version_from_date_mod
//! In Cargo.toml and service_worker.js writes the version as the date**  

//region: use statements
use ansi_term::Colour::{Green, Red, Yellow};
use chrono::DateTime;
use chrono::Timelike;
use chrono::{Datelike, Utc};
use unwrap::unwrap;
use filetime::FileTime;
use serde_derive::{Deserialize, Serialize};
use std::{fs, io, path::Path};
//endregion

///file metadata
#[derive(Serialize, Deserialize)]
struct FileMetaData {
    //filename with path from Cargo.toml folder
    filename: String,
    //filedate from file
    filedate: String,
}

///the struct that is in the file auto_version_from_date.json
#[derive(Serialize, Deserialize)]
struct AutoVersionFromDate {
    ///vector of file metadata
    vec_file_metadata: Vec<FileMetaData>,
}

pub fn auto_version_from_date() {    
    let mut is_files_equal = true;

    //find auto_version_from_date.json
    let json_filepath = "target/auto_version_from_date.json";
    let js_struct: AutoVersionFromDate;
    let f = fs::read_to_string(json_filepath);
    match f {
        Ok(x) => {
            println!("reading from {}", json_filepath);
            //read struct from file
            js_struct = unwrap!(serde_json::from_str(x.as_str()));
        }
        Err(_error) => {
            println!("file does not exist: {}", Red.paint(json_filepath));
            //create empty struct
            js_struct = AutoVersionFromDate {
                vec_file_metadata: Vec::new(),
            }
        }
    };
    //make a vector of files
    let mut vec_of_metadata: Vec<FileMetaData> = Vec::new();
    //the Cargo.toml and in the folder rs
    let filename = "Cargo.toml".to_string();
    let metadata = unwrap!(fs::metadata(filename.as_str()));
    let mtime = FileTime::from_last_modification_time(&metadata);
    let filedate = format!("{}", mtime);
    vec_of_metadata.push(FileMetaData { filename, filedate });

    //println!("fs: {}", serde_json::to_string(&v).unwrap());

    let src_dir = format!("{}/src", unwrap!(current_dir.to_str()));
    for entry in unwrap!(fs::read_dir(src_dir)) {
        let entry = unwrap!(entry);
        let path = entry.file_name();

        let filename = format!("src/{:?}", path);
        let filename = filename.replace("\"", "");
        //println!("filename: {}", &filename);
        let metadata = unwrap!(fs::metadata(filename.as_str()));
        let mtime = FileTime::from_last_modification_time(&metadata);
        let filedate = format!("{}", mtime);
        vec_of_metadata.push(FileMetaData { filename, filedate });
    }

    //println!("fs: {}", serde_json::to_string(&v).unwrap());

    //if files are added or deleted, other files must be also changed
    //I need to check if the files on the filesystem are the same as in the json
    for x in &vec_of_metadata {
        //search in json file
        let mut is_equal = false;
        for y in &js_struct.vec_file_metadata {
            if x.filename == y.filename && x.filedate == y.filedate {
                is_equal = true;
                break;
            } else {
                //println!("{} {}\n", y.filename, y.filedate);
            }
        }
        if !is_equal {
            println!("{} {}", x.filename, x.filedate);
            is_files_equal = false;
            break;
        }
    }

    println!("is_files_equal: {}", is_files_equal);

    if !is_files_equal {
        let date = Utc::now();
        let new_version = version_from_date(date);
        //region: write version in Cargo.toml
        {
            println!("{}", Green.paint("write version to Cargo.toml"));
            //find version in Cargo.toml
            let cargo_filename = "Cargo.toml";
            let mut cargo_content = unwrap!(fs::read_to_string(cargo_filename));
            let delimiter = r#"version = ""#;
            let delimiter_len = delimiter.len();
            let option_location = cargo_content.find(delimiter);
            if let Some(location) = option_location {
                let start_version = location + delimiter_len;
                let option_end_quote = find_from(cargo_content.as_str(), start_version, r#"""#);
                if let Some(end_version) = option_end_quote {
                    //delete all the characters in between the markers
                    let old_version: String =
                        cargo_content.drain(start_version..end_version).collect();
                    println!(r#"old version: "{}""#, old_version.as_str());
                    if new_version != old_version {
                        println!("new_version {}", new_version);
                        cargo_content.insert_str(start_version, new_version.as_str());
                        println!("write file: {}", Yellow.paint(cargo_filename));
                        let _x = fs::write(cargo_filename, cargo_content);
                        //the Cargo.toml is now different

                        //correct the vector
                        let filename = "Cargo.toml".to_string();
                        let metadata = unwrap!(fs::metadata(filename.as_str()));
                        let mtime = FileTime::from_last_modification_time(&metadata);
                        let filedate = format!("{}", mtime);
                        unwrap!(vec_of_metadata.get_mut(0)).filedate = filedate;

                        println!("save the new file metadata");
                        let x = AutoVersionFromDate {
                            vec_file_metadata: vec_of_metadata,
                        };
                        let y = unwrap!(serde_json::to_string(&x));
                        let _f = fs::write(json_filepath, y);
                    }
                } else {
                    panic!("no end quote for version");
                }
            } else {
                panic!("Cargo.toml has no version");
            }
        }
        // endregion

        //region: write version in service_worker.js

        // search for file service_worker.js
        // if the parent folder has Cargo.toml, than search there
        // because it is a workspace with members
        // else search here

        let cargo_filename = "../Cargo.toml";
        let start_dir = if Path::new(cargo_filename).exists() {
            Path::new("../")
        } else {
            Path::new("./")
        };
        println!("start_dir: {:?}", start_dir,);

        // fill a vector of files
        for js_filename in &unwrap!(traverse_dir_with_exclude_dir(
            start_dir,
            "/service_worker.js",
            &vec!["/.git".to_string(), "/target".to_string()]
        )) {
            println!(
                "{} {}",
                Green.paint("write version in "),
                Green.paint(js_filename)
            );
            let mut js_content = unwrap!(fs::read_to_string(js_filename));
            let delimiter = r#"const CACHE_NAME = '"#;
            let delimiter_len = delimiter.len();
            let option_location = js_content.find(delimiter);
            if let Some(location) = option_location {
                let start_version = location + delimiter_len;
                let option_end_quote = find_from(js_content.as_str(), start_version, r#"';"#);
                if let Some(end_version) = option_end_quote {
                    //delete all the characters in between the markers
                    let old_version: String =
                        js_content.drain(start_version..end_version).collect();
                    println!(r#"old version: "{}""#, old_version.as_str());
                    if new_version != old_version {
                        println!("new_version {}", new_version);
                        js_content.insert_str(start_version, new_version.as_str());
                        println!("write file: {}", Yellow.paint(js_filename));
                        let _x = fs::write(js_filename, js_content);
                    }
                } else {
                    panic!("no end quote for version");
                }
            } else {
                panic!("service_worker.js has no version");
            }
        }
        //endregion
    }
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

/// traverse dir (sub-dir) with exclude dir
/// the find_file and the exclude dir strings must start with /
fn traverse_dir_with_exclude_dir(
    dir: &Path,
    find_file: &str,
    exclude_dirs: &Vec<String>,
) -> io::Result<Vec<String>> {
    let mut v = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let str_path = unwrap!(path.to_str());
            if path.is_dir() {
                let mut is_excluded = false;
                for excl in exclude_dirs {
                    if str_path.ends_with(excl) {
                        is_excluded = true;
                        break;
                    }
                }
                if !is_excluded {
                    let mut sub_v = traverse_dir_with_exclude_dir(&path, find_file, exclude_dirs)?;
                    v.append(&mut sub_v);
                }
            } else {
                if str_path.ends_with(find_file) {
                    v.push(str_path.to_string());
                }
            }
        }
    }
    Ok(v)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_date_to_version() {
        let date_time = Utc.ymd(2020, 5, 22).and_hms(00, 34, 0);

        let version = version_from_date(date_time);
        assert_eq!(version, "2020.522.34");
    }
}
