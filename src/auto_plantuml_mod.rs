// auto_plantuml_mod.rs

//! Find md files.
//! Search for marker (auto_plantuml start) and (auto_plantuml end).
//! This markers around the plantuml code define that we want a svg file.
//! If there are no markers, the plantuml will not be processed.
//!
//! 1. [comment]: # (auto_plantuml start)
//! ```plantuml
//! @startuml
//! [Bob] ..> [Alice]
//! @enduml
//! ```
//!
//! ![svg_534231](images/svg_534231.svg)  
//!
//! 2. [comment]: # (auto_plantuml end)
//!
//! Between the last triple backtick and the end marker is the processed svg file.
//! Calculate a short hash from the plantuml code.
//! If the name of the svg file contains this hash code it means, we already have the correct svg file.
//! Else we have to delete the old svg file and get a new one from the modified plantuml code.
//! Call the plantuml.com server with the plantuml code and saves the result svg file in folder images/.
//! Add the hash code to the filename.

use unwrap::unwrap;
use lazy_static::lazy_static;
use crate::utils_mod::*;


lazy_static! {
    static ref REGEX_PLANTUML_START: regex::Regex = unwrap!(regex::Regex::new(
        r#"(?m)^\[comment\]: # \(auto_plantuml start\)$"#
    ));
    static ref REGEX_PLANTUML_END: regex::Regex = unwrap!(regex::Regex::new(
        r#"(?m)^\[comment\]: # \(auto_plantuml end\)$"#
    ));
    // capture group
    static ref REGEX_IMG_LINK: regex::Regex = unwrap!(regex::Regex::new(
    r#"!\[.+\]\(.+/svg_(.+)\.svg\)"#
    ));
}

/// process plantuml in current directory
pub fn auto_plantuml(){
    let path = std::env::current_dir().unwrap();
    auto_plantuml_for_path(&path);
}

/// process plantuml for all md files
/// for test and examples I need to provide the path
pub fn auto_plantuml_for_path(path:&std::path::Path){
    //use traverse instead of glob
    let files = unwrap!(crate::utils_mod::traverse_dir_with_exclude_dir(
        path,
        "/*.md",
        // avoid big folders and other folders with *.crev
        &vec![
            "/.git".to_string(),
            "/target".to_string(),
            "/docs".to_string()
        ]
    ));
    for md_filename in files{
        dbg!(&md_filename);
        let md_text_content = unwrap!(std::fs::read_to_string(&md_filename));

        // check if file have CRLF and show error
        if md_text_content.contains("\r\n") {
            panic!("Error: {} has CRLF line endings instead of LF. The task auto_plantuml cannot work! Closing.", &md_filename);
        }
        let mut pos = 0;
        // find markers
        while let Some(marker_start) = find_pos_start_data_after_delimiter(&md_text_content, pos,"\n[comment]: # (auto_plantuml start)\n"){
            pos = marker_start+34;
            if let Some(code_start) = find_pos_start_data_after_delimiter(&md_text_content, marker_start,"\n```plantuml\n"){
                if let Some(code_end) = find_pos_end_data_before_delimiter(&md_text_content, code_start,"\n```\n"){
                    let code_end_after=code_end+5;
                    let plantuml_code = &md_text_content[code_start..code_end];
                    dbg!(plantuml_code);
                    let plantuml_code_hash = hash_for_filename(plantuml_code);
                    dbg!(&plantuml_code_hash);
                    if let Some(marker_end) = find_pos_end_data_before_delimiter(&md_text_content, pos,"\n[comment]: # (auto_plantuml end)\n"){
                        let img_link = md_text_content[code_end_after..marker_end].trim();
                        let mut get_new_svg=false;
                        if img_link.is_empty(){
                            get_new_svg=true;
                            dbg!("img_link is empty.");
                        } else{
                            dbg!(img_link);
                            // parse this format ![svg_534231](images/svg_534231.svg)                          
                            let cap_group = REGEX_IMG_LINK .captures(img_link).unwrap();
                            let old_hash = &cap_group[1];
                            dbg!(old_hash);
                            if old_hash != &plantuml_code_hash{
                                get_new_svg=true;                              
                            }
                        }
                        if get_new_svg==true{
                            // rename the old image file to .obsolete
                            // get the new svg image
                            let svg_code = get_svg(plantuml_code);
                            // dbg!(&svg_code);
                            let new_file_path = std::path::Path::new(&md_filename).parent().unwrap().join("images").join(format!("svg_{}.svg",plantuml_code_hash));
                            dbg!(&new_file_path);
                            std::fs::write(&new_file_path, svg_code).unwrap();
                            // create the new image lnk
                            let img_link = format!("\n![svg_{}](images/svg_{}.svg)\n",plantuml_code_hash, plantuml_code_hash);
                            // delete the old img_link and insert the new one
                            //then write the modified md_file.
                            
                        }
                            
                    }
                }
            }
        }
    }

}


pub fn hash_for_filename(text: &str) -> String {
    let hash = <sha2::Sha256 as sha2::Digest>::digest(text.as_bytes());
    // base64ct = {version = "1.5.0", features = ["alloc"] }
    let base64_hash = <base64ct::Base64UrlUnpadded as base64ct::Encoding>::encode_string(&hash);
    // return
    base64_hash
}

pub fn get_svg(plant_uml_code: &str) -> String {
    let base_url = "https://plantuml.com/plantuml";
    let url_parameter = compress_plant_uml_code(plant_uml_code);
    let url = format!("{}/svg/{}", base_url, url_parameter);
    // use reqwest to GET from plantuml.com server
    let resp = reqwest::blocking::get(&url)
        .unwrap()
        .text_with_charset("utf-8")
        .unwrap();
    // return
    resp
}

/// deflate and strange base64, that is Url_safe
pub fn compress_plant_uml_code(plant_uml_code: &str) -> String {
    // first deflate
    let data = plant_uml_code.as_bytes();
    let compressed = deflate::deflate_bytes(data);
    // then the strange base64
    // https://plantuml.com/text-encoding
    let my_cfg = radix64::CustomConfig::with_alphabet(
        "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_",
    )
    .no_padding()
    .build()
    .unwrap();
    let b64 = my_cfg.encode(&compressed);
    // return
    b64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_hash() {
        assert_eq!(
            "n4bQgYhMfWWaL-qgxVrQFaO_TxsrC4Is0V1sFbDwCgg",
            hash_for_filename("test")
        );
    }

    #[test]
    pub fn test_compress_plant_uml_code() {
        // http://www.plantuml.com/plantuml/uml/SoWkIImgAStDuNBCoKnELT2rKt3AJx9IS2mjoKZDAybCJYp9pCzJ24ejB4qjBk42oYde0jM05MDHLLoGdrUSokMGcfS2D1C0
        assert_eq!(
            compress_plant_uml_code(
                r#"@startuml
Alice -> Bob: Authentication Request
Bob --> Alice: Authentication Response
@enduml"#
            ),
            "SoWkIImgAStDuNBCoKnELT2rKt3AJx9IS2mjoKZDAybCJYp9pCzJ24ejB4qjBk42oYde0jM05MDHLLoGdrUSokMGcfS2D1C0"
        );
    }

}
