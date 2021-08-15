//! utils_mod.rs

use lazy_static::lazy_static;
use regex::Regex;
use std::{fs, io, path::Path};
use unwrap::unwrap;

lazy_static! {
    static ref REGEX_NUMBER_SPACE: Regex = Regex::new(r#"[0-9] "#).unwrap();
}
// region: delimiters cannot be INACTIVE like markers

/// return the position of start of the delimited data after the delimiter
pub fn find_pos_start_data_after_delimiter(
    md_text_content: &str,
    pos: usize,
    delimiter: &str,
) -> Option<usize> {
    if let Some(pos_start_data) = find_from(md_text_content, pos, delimiter) {
        let pos_start_data = pos_start_data + delimiter.len();
        return Some(pos_start_data);
    }
    // return
    None
}

/// return the position of end of the delimited data before the delimiter
pub fn find_pos_end_data_before_delimiter(
    md_text_content: &str,
    pos: usize,
    delimiter: &str,
) -> Option<usize> {
    if let Some(pos_end_data) = find_from(md_text_content, pos, delimiter) {
        return Some(pos_end_data);
    }
    //return
    None
}

// endregion: delimiters cannot be INACTIVE like markers

/// find from_pos
pub fn find_from(text: &str, from_pos: usize, find: &str) -> Option<usize> {
    let slice01 = text.get(from_pos..).unwrap();
    let option_location = slice01.find(find);
    if let Some(location) = option_location {
        // return Option with usize
        Some(from_pos + location)
    } else {
        // return Option with none
        option_location
    }
}

//// Traverse dir and its sub-dir, but avoid excluded dirs.
/// The find_file and the exclude dir strings must start with /.
///
/// ## Example
///
/// ```
/// use std::path::Path;
///
/// let files = cargo_auto_lib::utils_mod::traverse_dir_with_exclude_dir(
///     Path::new("/home/project/src"),
///     "/*.rs",
///     // avoid big folders and other folders with *.crev
///     &vec![
///         "/.git".to_string(),
///         "/target".to_string(),
///         "/docs".to_string()
///     ]
/// ).unwrap();
/// for rs_file_name in files.iter() {
///     println!("{}", &rs_file_name);
/// }
/// ```
pub fn traverse_dir_with_exclude_dir(
    dir: &Path,
    find_file: &str,
    exclude_dirs: &[String],
) -> io::Result<Vec<String>> {
    // if the parameter is /*.rs, I can eliminate /*
    let find_file = &find_file.replace("/*", "");

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
            } else if str_path.ends_with(find_file) {
                v.push(str_path.to_string());
            }
        }
    }
    Ok(v)
}

/// the original `concat()` function does not have a delimiter
pub fn concatenate_vec_to_string(vec: &Vec<String>, delimiter:&str) -> String {
    let size = vec.iter().fold(0, |a, b| a + b.len());
    let mut concatenated_string = String::with_capacity(size);
    for (i, item) in vec.iter().enumerate(){
        if i > 0 {
            concatenated_string.push_str(delimiter);
        }
        concatenated_string.push_str(item);    
    }
    // return
    concatenated_string
}