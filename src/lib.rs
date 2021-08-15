// region: auto_md_to_doc_comments include README.md A //!

// endregion: auto_md_to_doc_comments include README.md A //!

// region: mod, extern and use statements
mod auto_cargo_toml_to_md_mod;
mod auto_delete_old_js_snippets_mod;
/// TODO: cargo-auto_lib could change the code to make some element visibility `pub` only for testing. And after return to normal.  
mod auto_lines_of_code_mod;
mod auto_md_to_doc_comments_mod;
mod auto_semver_mod;
mod auto_version_from_date_mod;
mod auto_helper_functions_mod;
mod auto_cargo_toml_mod;
pub mod utils_mod;

// reexport functions for callers of the library
pub use auto_cargo_toml_to_md_mod::auto_cargo_toml_to_md;
pub use auto_delete_old_js_snippets_mod::auto_delete_old_js_snippets;
pub use auto_lines_of_code_mod::auto_lines_of_code;
pub use auto_md_to_doc_comments_mod::auto_md_to_doc_comments;
pub use auto_semver_mod::auto_semver_increment_minor;
pub use auto_semver_mod::auto_semver_increment_patch;
pub use auto_version_from_date_mod::auto_version_from_date;
pub use auto_helper_functions_mod::run_shell_command;
pub use auto_helper_functions_mod::run_shell_commands;
pub use auto_helper_functions_mod::GREEN;
pub use auto_helper_functions_mod::YELLOW;
pub use auto_helper_functions_mod::RED;
pub use auto_helper_functions_mod::RESET;
pub use auto_helper_functions_mod::CLEAR_LINE;
pub use auto_helper_functions_mod::CLEAR_ALL;
pub use auto_helper_functions_mod::UNHIDE_CURSOR;
pub use auto_cargo_toml_mod::package_name;
pub use auto_cargo_toml_mod::package_version;
pub use auto_cargo_toml_mod::package_authors_string_without_emails;
pub use auto_cargo_toml_mod::package_repository;
pub use auto_cargo_toml_mod::package_description;
