// region: auto_md_to_doc_comments include README.md A //!

// endregion: auto_md_to_doc_comments include README.md A //!

// region: mod, extern and use statements
mod auto_cargo_toml_to_md_mod;
mod auto_lines_of_code_mod;
mod auto_md_to_doc_comments_mod;
mod auto_semver_mod;
mod auto_version_from_date_mod;
mod utils_mod;

// reexport objects for callers of the library
pub use auto_cargo_toml_to_md_mod::auto_cargo_toml_to_md;
pub use auto_lines_of_code_mod::auto_lines_of_code;
pub use auto_md_to_doc_comments_mod::auto_md_to_doc_comments;
pub use auto_semver_mod::auto_semver_increment_minor;
pub use auto_semver_mod::auto_semver_increment_patch;
pub use auto_version_from_date_mod::auto_version_from_date;
