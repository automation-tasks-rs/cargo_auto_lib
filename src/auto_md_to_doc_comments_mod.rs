// auto_md_to_doc_comments_mod

use glob::glob;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use unwrap::unwrap;

#[derive(Debug)]
struct RsMarker {
    pub md_filename: String,
    pub marker_name: String,
    pub comment_symbol: String,
    pub pos_start: usize,
    pub pos_end: usize,
}

#[derive(Debug)]
struct MdSegment {
    pub md_filename: String,
    pub marker_name: String,
    pub pos_start: usize,
    pub pos_end: usize,
    pub text: String,
}

/// find rs files with markers and include md segments
pub fn auto_md_to_doc_comments() {
    let mut cache_md_segments = vec![];
    for rs_filename in rs_files().iter() {
        let mut rs_text_content = unwrap!(fs::read_to_string(rs_filename));
        let markers = rs_file_markers(&rs_text_content);
        if !markers.is_empty() {
            for marker in markers.iter().rev() {
                let segment_text = get_md_segments_using_cache(
                    &mut cache_md_segments,
                    &marker.md_filename,
                    &marker.marker_name,
                    &marker.comment_symbol,
                );
                rs_text_content.replace_range(marker.pos_start..marker.pos_end, &segment_text);
            }
            println!("write file: {}", rs_filename);
            unwrap!(fs::write(rs_filename, rs_text_content));
        }
    }
}

/// All rs files in src, tests and examples folders.
/// The current dir must be the project root where the Cargo.toml is.
/// In case of workspace, all the members projects must be processed separately.
fn rs_files() -> Vec<String> {
    let mut rs_files = vec![];
    // in Unix shell ** means recursive match through all the subdirectories
    for filename_result in unwrap!(glob("src/**/*.rs")) {
        let filename_pathbuff = unwrap!(filename_result);
        let rs_filename = unwrap!(filename_pathbuff.to_str()).to_string();
        rs_files.push(rs_filename);
    }
    for filename_result in unwrap!(glob("tests/**/*.rs")) {
        let filename_pathbuff = unwrap!(filename_result);
        let rs_filename = unwrap!(filename_pathbuff.to_str()).to_string();
        rs_files.push(rs_filename);
    }
    for filename_result in unwrap!(glob("examples/**/*.rs")) {
        let filename_pathbuff = unwrap!(filename_result);
        let rs_filename = unwrap!(filename_pathbuff.to_str()).to_string();
        rs_files.push(rs_filename);
    }
    //return
    rs_files
}

lazy_static! {
    static ref REGEX_RS_START: Regex =
        Regex::new(r#"(?m)^ *?// region: auto_md_to_doc_comments include (.*?) (.*?) (.*?)$"#)
            .unwrap();
}
lazy_static! {
    static ref REGEX_RS_END: Regex =
        Regex::new(r#"(?m)^ *?// endregion: auto_md_to_doc_comments include (.*?) (.*?) (.*?)$"#)
            .unwrap();
}
/// markers in rs files
fn rs_file_markers(rs_text_content: &str) -> Vec<RsMarker> {
    let mut markers = vec![];
    for cap in REGEX_RS_START.captures_iter(rs_text_content) {
        markers.push(RsMarker {
            md_filename: cap[1].to_string(),
            marker_name: cap[2].to_string(),
            comment_symbol: cap[3].to_string(),
            pos_start: unwrap!(cap.get(0)).end() + 1,
            pos_end: 0,
        });
    }
    for cap in REGEX_RS_END.captures_iter(rs_text_content) {
        let marker = unwrap!(markers
            .iter_mut()
            .find(|m| m.md_filename == cap[1] && m.marker_name == cap[2]));
        marker.pos_end = unwrap!(cap.get(0)).start();
    }
    // return
    markers
}

lazy_static! {
    static ref REGEX_MD_START: Regex =
        Regex::new(r#"(?m)^\[comment\]: # \(auto_md_to_doc_comments segment start (.*?)\)$"#)
            .unwrap();
}
lazy_static! {
    static ref REGEX_MD_END: Regex =
        Regex::new(r#"(?m)^\[comment\]: # \(auto_md_to_doc_comments segment end (.*?)\)$"#)
            .unwrap();
}

/// The first time it is called
/// reads the file and extracts all the segments
/// into a cache vector.
/// Subsequent calls read from the cache.
fn get_md_segments_using_cache(
    cache: &mut Vec<MdSegment>,
    md_filename: &str,
    marker_name: &str,
    comment_symbol: &str,
) -> String {
    // check the cache
    if let Some(_seg) = cache.iter().find(|m| m.md_filename == md_filename) {
        let segment = unwrap!(cache
            .iter()
            .find(|m| m.md_filename == md_filename && m.marker_name == marker_name));
        return segment.text.to_string();
    } else {
        // process the file
        println!("read file: {}", md_filename);
        let md_text_content = unwrap!(fs::read_to_string(md_filename));
        for cap in REGEX_MD_START.captures_iter(&md_text_content) {
            cache.push(MdSegment {
                md_filename: md_filename.to_owned(),
                marker_name: cap[1].to_owned(),
                pos_start: unwrap!(cap.get(0)).end() + 1,
                pos_end: 0,
                text: String::new(),
            });
        }
        for cap in REGEX_MD_END.captures_iter(&md_text_content) {
            let segment = unwrap!(cache
                .iter_mut()
                .find(|m| m.md_filename == md_filename && m.marker_name == cap[1]));
            segment.pos_end = unwrap!(cap.get(0)).start();
            // the segment begins with a comment, so don't include the next empty row
            let mut last_line_was_comment = true;
            for line in md_text_content[segment.pos_start..segment.pos_end].lines() {
                if line.starts_with("[comment]: # (") {
                    // don't include md comments
                    last_line_was_comment = true;
                } else if last_line_was_comment == true && line.is_empty() {
                    // don't include empty line after md comments
                    last_line_was_comment = false;
                } else {
                    last_line_was_comment = false;
                    segment.text.push_str(comment_symbol);
                    if !line.is_empty() {
                        segment.text.push_str(" ");
                    }
                    segment.text.push_str(line);
                    segment.text.push('\n');
                }
            }
        }
        let segment = unwrap!(cache
            .iter()
            .find(|m| m.md_filename == md_filename && m.marker_name == marker_name));
        return segment.text.to_string();
    }
}
