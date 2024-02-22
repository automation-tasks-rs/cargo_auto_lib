// lib.rs

// logo for docs.rs in png
#![doc(
    html_logo_url = "https://github.com/bestia-dev/cargo-auto/raw/main/images/logo/logo_cargo_auto.svg"
)]
// even favicon ico can be changed
// #![doc(html_favicon_url = "/logo.ico")]
// playground for examples. Warning: It didn't work well for me. And it works only in docs. Not in Github and not in crates.io.
// I will not use it. I will crate a gist and use that for the playground. That works flawlessly. Maybe create an automation task?
// #![doc(html_playground_url = "https://play.rust-lang.org/")]
// example how to insert a svg file inside the documentation
// #![doc=include_str!("shared-bus.svg")]

// include the README.md into doc_comments
#![doc=include_str!("../README.md")]

// region: mod, extern and use statements
mod auto_cargo_toml_mod;
mod auto_cargo_toml_to_md_mod;
mod auto_check_micro_xml_mod;
mod auto_copy_files_to_strings_mod;
mod auto_delete_old_js_snippets_mod;
mod auto_doc_tidy_html_mod;
mod auto_git_mod;
mod auto_github_mod;
mod auto_helper_functions_mod;
mod auto_lines_of_code_mod;
mod auto_md_to_doc_comments_mod;
mod auto_plantuml_mod;
mod auto_semver_mod;
mod auto_semver_or_date_mod;
mod auto_version_from_date_mod;
mod error_mod;
mod public_api_mod;
mod utils_mod;
// endregion

// only the Public API is exported (functions, structs, methods, enums, traits)
pub use public_api_mod::*;
