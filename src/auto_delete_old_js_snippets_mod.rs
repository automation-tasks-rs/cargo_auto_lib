// auto_delete_old_js_snippets_mod

//! deletes old js snippets when working with wasm-pack

//region: use statements
use crate::public_api_mod::{RED, RESET, YELLOW};
use filetime::FileTime;

//endregion

/// deletes old js snippets when working with wasm-pack  
/// The old folders for `js snippets` are not automatically deleted on building with `wasm-pack`.  
/// This utils do that.  
/// The function must be executed in the root project folder where is the Cargo.toml.  
pub fn auto_delete_old_js_snippets() {
    let current_dir = std::env::current_dir().unwrap();
    let snippets_dir = current_dir.join("pkg").join("snippets");
    //the first folder can be None
    let mut opt_first_folder: Option<std::path::PathBuf> = None;
    let mut opt_first_mtime: Option<FileTime> = None;

    //find the newer folder and remove the older folder
    //but not with dodrio_xxx name
    for entry in std::fs::read_dir(snippets_dir).unwrap() {
        let entry = entry.unwrap();
        let second_folder = entry.path();
        let second_name = entry.file_name().into_string().unwrap().to_lowercase();
        if !second_name.starts_with("dodrio") {
            //println!("{:?}",second_folder);
            let second_metadata = std::fs::metadata(&second_folder).unwrap();
            let second_mtime = FileTime::from_last_modification_time(&second_metadata);
            //println!("{:?}",second_mtime);

            match opt_first_mtime {
                None => {
                    opt_first_folder = Some(second_folder.clone());
                    opt_first_mtime = Some(second_mtime);
                }
                Some(first_mtime) => match second_mtime.cmp(&first_mtime) {
                    // if second_mtime > first_mtime {
                    std::cmp::Ordering::Greater => {
                        let first_folder = opt_first_folder.unwrap();
                        println!("    {YELLOW}delete first: {:?}{RESET}", first_folder);
                        std::fs::remove_dir_all(first_folder).unwrap();

                        opt_first_folder = Some(second_folder.clone());
                        opt_first_mtime = Some(second_mtime);
                    }
                    //  } else if first_mtime > second_mtime {
                    std::cmp::Ordering::Less => {
                        println!("    {YELLOW}delete second: {:?}{RESET}", second_folder);
                        std::fs::remove_dir_all(second_folder).unwrap();
                    }
                    // else
                    std::cmp::Ordering::Equal => {
                        eprintln!("{RED}Error: folders have the same date?{RESET}");
                    }
                },
            }
        }
    }
}
