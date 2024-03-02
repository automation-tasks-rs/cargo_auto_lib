// auto_md_to_doc_comments_mod

//! finds rs files with markers and include segments from md files

use glob::glob;
use lazy_static::lazy_static;
use regex::Regex;

// this trait must be in scope to use these methods of CargoToml
use crate::public_api_mod::CargoTomlPublicApiMethods;
use crate::public_api_mod::{RED, RESET, YELLOW};

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

// region: auto_md_to_doc_comments include doc_comments/auto_md_to_doc_comments.md A ///
/// This function finds rs files with markers and include segments from md files as doc comments.  
///
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
/// In the rs file write these markers:  
///
/// ```code
/// comment region: auto_md_to_doc_comments include README.md A ///
/// comment endregion: auto_md_to_doc_comments include README.md A ///
/// ```
///
/// In your rust code, change the word `comment` with double slash `//`.  
/// In the md file put markers to mark the segment:  
///
/// ```markdown
/// [comment]: # (auto_md_to_doc_comments segment start A)  
/// [comment]: # (auto_md_to_doc_comments segment end A)  
/// ```
///
/// In your markdown, change the word `[comment]` with double slash `[//]`.
///
/// The marker must be exclusively in one line. No other text in the same line.  
/// auto_md_to_doc_comments will delete the old lines between the markers.  
/// It will find the md file and read the content between the markers.  
/// Before each line it will add the doc comment symbol as is defined in the marker.  
/// Finally it will include the new lines as doc comments in the rs file.
///
// endregion: auto_md_to_doc_comments include doc_comments/auto_md_to_doc_comments.md A ///
pub fn auto_md_to_doc_comments() {
    println!("    {YELLOW}auto_md_to_doc_comments{RESET}");
    // Cargo.toml contains the list of projects
    let cargo_toml = crate::auto_cargo_toml_mod::CargoToml::read();
    match cargo_toml.workspace_members() {
        None => one_project(),
        Some(members) => {
            for member in members.iter() {
                println!("    {YELLOW}{member}{RESET}");
                std::env::set_current_dir(member).unwrap();
                one_project();
                std::env::set_current_dir("..").unwrap();
            }
        }
    }
}

fn one_project() {
    let mut cache_md_segments = vec![];
    for rs_filename in rs_files().iter() {
        let mut rs_text_content = std::fs::read_to_string(rs_filename).unwrap();

        // check if file have CRLF instead of LF and show error
        if rs_text_content.contains("\r\n") {
            panic!("{RED}Error: {rs_filename} has CRLF line endings instead of LF. Correct the file! Exiting...{RESET}");
        }

        let markers = rs_file_markers(&rs_text_content);
        if !markers.is_empty() {
            for marker in markers.iter().rev() {
                let segment_text = get_md_segments_using_cache(&mut cache_md_segments, &marker.md_filename, &marker.marker_name, &marker.comment_symbol);
                rs_text_content.replace_range(marker.pos_start..marker.pos_end, &segment_text);
            }
            println!("    {YELLOW}Write file: {rs_filename}{RESET}");
            std::fs::write(rs_filename, rs_text_content).unwrap();
        }
    }
}

/// All rs files in src, tests and examples folders.
/// The current dir must be the project root where the Cargo.toml is.
fn rs_files() -> Vec<String> {
    let mut rs_files = vec![];
    // in Unix shell ** means recursive match through all the subdirectories
    for filename_result in glob("src/**/*.rs").unwrap() {
        let filename_pathbuff = filename_result.unwrap();
        let rs_filename = filename_pathbuff.to_str().unwrap().to_string();
        rs_files.push(rs_filename);
    }
    for filename_result in glob("tests/**/*.rs").unwrap() {
        let filename_pathbuff = filename_result.unwrap();
        let rs_filename = filename_pathbuff.to_str().unwrap().to_string();
        rs_files.push(rs_filename);
    }
    for filename_result in glob("examples/**/*.rs").unwrap() {
        let filename_pathbuff = filename_result.unwrap();
        let rs_filename = filename_pathbuff.to_str().unwrap().to_string();
        rs_files.push(rs_filename);
    }
    //return
    rs_files
}

lazy_static! {
    static ref REGEX_RS_START: Regex = Regex::new(r#"(?m)^ *?// region: auto_md_to_doc_comments include (.*?) (.*?) (.*?)$"#).unwrap();
}
lazy_static! {
    static ref REGEX_RS_END: Regex = Regex::new(r#"(?m)^ *?// endregion: auto_md_to_doc_comments include (.*?) (.*?) (.*?)$"#).unwrap();
}
/// markers in rs files
fn rs_file_markers(rs_text_content: &str) -> Vec<RsMarker> {
    let mut markers = vec![];
    for cap in REGEX_RS_START.captures_iter(rs_text_content) {
        let rs_marker = RsMarker {
            md_filename: cap[1].to_string(),
            marker_name: cap[2].to_string(),
            comment_symbol: cap[3].to_string(),
            pos_start: cap.get(0).unwrap().end() + 1,
            pos_end: 0,
        };
        markers.push(rs_marker);
    }
    for cap in REGEX_RS_END.captures_iter(rs_text_content) {
        let marker = markers.iter_mut().find(|m| m.md_filename == cap[1] && m.marker_name == cap[2]).unwrap();
        marker.pos_end = cap.get(0).unwrap().start();
    }
    // return
    markers
}

lazy_static! {
    static ref REGEX_MD_START: Regex = Regex::new(r#"(?m)^\[//\]: # \(auto_md_to_doc_comments segment start (.*?)\)$"#).unwrap();
}
lazy_static! {
    static ref REGEX_MD_END: Regex = Regex::new(r#"(?m)^\[//\]: # \(auto_md_to_doc_comments segment end (.*?)\)$"#).unwrap();
}

/// The first time it is called read the file and extracts all the segments into a cache vector.
///
/// Subsequent calls read from the cache.
fn get_md_segments_using_cache(cache: &mut Vec<MdSegment>, md_filename: &str, marker_name: &str, comment_symbol: &str) -> String {
    // check the cache
    if let Some(_seg) = cache.iter().find(|m| m.md_filename == md_filename) {
        let segment = cache.iter().find(|m| m.md_filename == md_filename && m.marker_name == marker_name).unwrap();
        segment.text.to_string()
    } else {
        // process the file
        println!("    {YELLOW}Read file: {md_filename}{RESET}");
        let md_text_content = std::fs::read_to_string(md_filename).unwrap();

        // check if file have CRLF instead of LF and show error
        if md_text_content.contains("\r\n") {
            panic!("{RED}Error: {md_filename} has CRLF line endings instead of LF. Correct the file! Exiting...{RESET}");
        }

        for cap in REGEX_MD_START.captures_iter(&md_text_content) {
            cache.push(MdSegment {
                md_filename: md_filename.to_owned(),
                marker_name: cap[1].to_owned(),
                pos_start: cap.get(0).unwrap().end() + 1,
                pos_end: 0,
                text: String::new(),
            });
        }
        for cap in REGEX_MD_END.captures_iter(&md_text_content) {
            let segment = cache.iter_mut().find(|m| m.md_filename == md_filename && m.marker_name == cap[1]).unwrap();
            segment.pos_end = cap.get(0).unwrap().start();
            // the segment begins with a comment, so don't include the next empty row
            let mut last_line_was_comment = true;
            for line in md_text_content[segment.pos_start..segment.pos_end].lines() {
                if line.starts_with("[//]: # (") {
                    // don't include md comments
                    last_line_was_comment = true;
                } else if last_line_was_comment && line.is_empty() {
                    // don't include empty line after md comments
                    last_line_was_comment = false;
                } else {
                    last_line_was_comment = false;
                    segment.text.push_str(comment_symbol);
                    if !line.is_empty() {
                        segment.text.push(' ');
                    }
                    segment.text.push_str(line);
                    segment.text.push('\n');
                }
            }
        }

        let segment = cache.iter().find(|m| m.md_filename == md_filename && m.marker_name == marker_name).unwrap();
        //return
        segment.text.to_string()
    }
}
