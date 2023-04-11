// auto_md_to_doc_comments_mod

//! finds rs files with markers and include segments from md files
//! It works for workspaces and for single projects.  

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

/// finds rs files with markers and include segments from md files
/// Includes segments of md files into rs files as doc comments.  
/// It works with workspaces and single projects.
/// From this doc comments `cargo doc` will generated the documentation and auto-completion.  
/// We don't want to manually copy this segments. We want them to be automatically in sync.  
/// We will just run this function before every `cargo doc` with an automation task.  
/// The `auto_md_to_doc_comments` function must be executed in the project root folder where is the Cargo.toml file.  
/// First it searches all the rs files in src, tests and examples folders.  
/// If they contain the markers, than finds the md file and the named segment and include it as doc comments into the rs file.  
/// The markers are always in pairs: start and end. So exactly the content in between is changed.
/// The markers are always comments, so they don't change the code.  
/// It works only for files with LF line delimiter. No CR and no CRLF.  
///
/// ## markers
///
/// In the rs file write these markers (don't copy the numbers 1 and 2):  
///
/// ```code
/// 1. // region: auto_md_to_doc_comments include README.md //! A  
/// 2. // endregion: auto_md_to_doc_comments include README.md //! A  
/// ```
///
/// In the md file put markers to mark the segment:  
///
/// ```markdown
/// 1. [//]: # (auto_md_to_doc_comments segment start A)  
/// 2. [//]: # (auto_md_to_doc_comments segment end A)  
/// ```
///
/// The marker must be exclusively in one line. No other text in the same line.  
/// auto_md_to_doc_comments will delete the old lines between the markers.  
/// It will find the md file and read the content between the markers.  
/// Before each line it will add the doc comment symbol as is defined in the marker.  
/// Finally it will include the new lines as doc comments in the rs file.  
pub fn auto_md_to_doc_comments() {
    println!("    auto_md_to_doc_comments");
    // Cargo.toml contains the list of projects
    let cargo_toml = crate::auto_cargo_toml_mod::CargoToml::read();
    match cargo_toml.workspace_members() {
        None => one_project(),
        Some(members) => {
            for member in members.iter() {
                println!("    {}", member);
                unwrap!(std::env::set_current_dir(member));
                one_project();
                unwrap!(std::env::set_current_dir(".."));
            }
        }
    }
}

fn one_project() {
    let mut cache_md_segments = vec![];
    for rs_filename in rs_files().iter() {
        let mut rs_text_content = unwrap!(fs::read_to_string(rs_filename));

        // check if file have CRLF instead of LF and show error
        if rs_text_content.contains("\r\n") {
            panic!("Error: {} has CRLF line endings instead of LF. The task auto_md_to_doc_comments cannot work! Closing.", rs_filename);
        }

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
            println!("    write file: {}", rs_filename);
            unwrap!(fs::write(rs_filename, rs_text_content));
        }
    }
}

/// All rs files in src, tests and examples folders.
/// The current dir must be the project root where the Cargo.toml is.
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
        Regex::new(r#"(?m)^\[//\]: # \(auto_md_to_doc_comments segment start (.*?)\)$"#).unwrap();
}
lazy_static! {
    static ref REGEX_MD_END: Regex =
        Regex::new(r#"(?m)^\[//\]: # \(auto_md_to_doc_comments segment end (.*?)\)$"#).unwrap();
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
        println!("    read file: {}", md_filename);
        let md_text_content = unwrap!(fs::read_to_string(md_filename));

        // check if file have CRLF instead of LF and show error
        if md_text_content.contains("\r\n") {
            panic!("Error: {} has CRLF line endings instead of LF. The task auto_md_to_doc_comments cannot work! Closing.", md_filename);
        }

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
                if line.starts_with("[//]: # (") {
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
