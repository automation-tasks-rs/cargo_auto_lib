// auto_copy_files_to_strings_mod.rs

// trait must be in scope
use base64ct::Encoding;

/// copy all files from the folder into a module as strings (static &str)
/// the module has the markers: region: files copied into strings by automation tasks and endregion:
/// the string will be in a vector with the file name
/// first we create the complete text, then we check if the old text needs to be replaced
/// let ext_for_binary_files=vec![".ico",".jpg",".png",".woff2"];
/// let exclude_big_folders = vec!["/.git".to_string(),"/target".to_string(),"/docs".to_string()];
pub fn copy_folder_files_into_module(
    folder_path: &std::path::Path,
    module_path: &std::path::Path,
    ext_for_binary_files: &[&str],
    exclude_big_folders: &[String],
) {
    println!(
        "copy_folder_files_into_module {}, {}",
        folder_path.to_string_lossy(),
        module_path.to_string_lossy()
    );
    // traverse and get all file_names
    let files =
        crate::traverse_dir_with_exclude_dir(&folder_path, "", exclude_big_folders).unwrap();
    let mut new_code = String::new();
    for file_name in files.iter() {
        let file_name_short =
            file_name.trim_start_matches(&format!("{}/", folder_path.to_string_lossy()));
        // avoid Cargo.lock file
        if file_name_short == "Cargo.lock" {
            continue;
        }
        // let the user define in an input parameter what files are binaries and not text.
        let mut is_binary_file = false;
        for x in ext_for_binary_files.iter() {
            if file_name_short.ends_with(x) {
                is_binary_file = true;
                break;
            }
        }

        let file_content = if is_binary_file {
            // convert binary file to base64
            let b = std::fs::read(&file_name).unwrap();
            base64ct::Base64::encode_string(&b)
        } else {
            // all others are text files
            // dbg!(&file_name_short);
            std::fs::read_to_string(&file_name).unwrap()
        };

        new_code.push_str(&format!(
            r####"vec_file.push(crate::FileItem{{
            file_name :"{}",
            file_content : r###"{}"###,
}});    
"####,
            &file_name_short, &file_content
        ));
    }

    // read the content of the module, delimited by markers
    let module_content = std::fs::read_to_string(module_path).unwrap();
    let start_pos = crate::find_pos_start_data_after_delimiter(
        &module_content,
        0,
        "// region: files copied into strings by automation tasks\n",
    )
    .expect("didn't find // region: files copied..");
    let end_pos = crate::find_pos_end_data_before_delimiter(
        &module_content,
        0,
        "// endregion: files copied into strings by automation tasks",
    )
    .expect("didn't find // endregion: files copied..");
    let old_code = &module_content[start_pos..end_pos];

    // compare the text, if different replace
    if old_code != new_code {
        let mut new_module_content = String::new();
        new_module_content.push_str(&module_content[..start_pos]);
        new_module_content.push_str(&new_code);
        new_module_content.push_str(&module_content[end_pos..]);
        std::fs::write(module_path, &new_module_content).unwrap();
    }
}