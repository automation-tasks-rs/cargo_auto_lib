// auto_delete_old_js_snippets_mod

//region: use statements
use filetime::FileTime;
use std::env;
use std::fs;
use std::path::PathBuf;
use unwrap::unwrap;
//endregion

pub fn auto_delete_old_js_snippets() {
    let current_dir = unwrap!(env::current_dir());
    let snippets_dir = current_dir.join("pkg").join("snippets");
    //the first folder can be None
    let mut opt_first_folder: Option<PathBuf> = None;
    let mut opt_first_mtime: Option<FileTime> = None;

    //find the newer folder and remove the older folder
    //but not with dodrio_xxx name
    for entry in unwrap!(fs::read_dir(snippets_dir)) {
        let entry = unwrap!(entry);
        let second_folder = entry.path();
        let second_name = unwrap!(entry.file_name().into_string()).to_lowercase();
        if !second_name.starts_with("dodrio") {
            //println!("{:?}",second_folder);
            let second_metadata = unwrap!(fs::metadata(&second_folder));
            let second_mtime = FileTime::from_last_modification_time(&second_metadata);
            //println!("{:?}",second_mtime);

            match opt_first_mtime {
                None => {
                    opt_first_folder = Some(second_folder.clone());
                    opt_first_mtime = Some(second_mtime);
                }
                Some(first_mtime) => {
                    if second_mtime > first_mtime {
                        let first_folder = unwrap!(opt_first_folder);
                        println!("delete first: {:?}", first_folder);
                        unwrap!(std::fs::remove_dir_all(first_folder));

                        opt_first_folder = Some(second_folder.clone());
                        opt_first_mtime = Some(second_mtime);
                    } else if first_mtime > second_mtime {
                        println!("delete second: {:?}", second_folder);
                        unwrap!(std::fs::remove_dir_all(second_folder));
                    } else {
                        println!("Error: folders have the same date?");
                    }
                }
            }
        }
    }
}
