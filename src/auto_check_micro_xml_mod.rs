// auto_check_micro_xml_mod.rs

use glob::glob;
use reader_for_microxml::{ReaderForMicroXml, Token};
use unwrap::unwrap;

/// I want html pages to be correct microXML when I use them for single page application.
/// Before build or release this function will check for correctness.
pub fn auto_check_micro_xml(path_to_html_pages: &str) {
    println!("auto_check_micro_xml {}", path_to_html_pages);
    let glob_str = format!("{}/*.html", path_to_html_pages.trim_end_matches("/"));
    // find html pages for single page application
    for filename_result in unwrap!(glob(&glob_str)) {
        let filename_pathbuff = unwrap!(filename_result);
        let file_name = unwrap!(unwrap!(filename_pathbuff.file_name()).to_str());
        let str_xml = unwrap!(std::fs::read_to_string(&filename_pathbuff));

        // check if file have CRLF instead of LF and show error
        if str_xml.contains("\r\n") {
            panic!("Error: {} has CRLF line endings instead of LF. The task auto_check_micro_xml cannot work! Closing.", filename_pathbuff.to_string_lossy());
        }

        // check microxml correctness. Panic on errors.
        check_micro_xml(&str_xml, file_name);
    }
}

/// panics if the microXML string is not correct
fn check_micro_xml(str_xml: &str, file_name: &str) {
    println!("Check MicroXml: {}", file_name);
    // remove <!DOCTYPE html> because it is not microXML
    let str_xml = str_xml.replace("<!DOCTYPE html>", "");
    let reader_iterator = ReaderForMicroXml::new(&str_xml);
    // reader_iterator is iterator Option<Result<Token,&str>>
    // the first option is used for the iterator to know where is the end
    // then the Result can have an Token or an Error
    let mut vec_elem: Vec<&str> = vec![];
    for result_token in reader_iterator {
        match result_token {
            Ok(token) => match token {
                Token::StartElement(name) => vec_elem.push(name),
                Token::Attribute(_name, _value) => continue,
                Token::TextNode(_txt) => continue,
                Token::Comment(_txt) => continue,
                Token::EndElement(end_element_name) => {
                    let start_element_name = unwrap!(vec_elem.pop());
                    if end_element_name != "" && end_element_name != start_element_name {
                        panic!(
                            "{}MicroXml {} start {} does not match end {}{}",
                            *crate::RED,
                            file_name,
                            start_element_name,
                            end_element_name,
                            *crate::RESET
                        );
                    }
                }
            },
            Err(err_msg) => {
                panic!(
                    "{}MicroXml {} incorrect : {}{}",
                    *crate::RED,
                    file_name,
                    err_msg,
                    *crate::RESET
                );
            }
        }
    }
}
