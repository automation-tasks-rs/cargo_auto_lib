//! utils_mod.rs

use lazy_static::lazy_static;
use regex::Regex;

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
