// auto_cargo_toml_mod

//! functions to get data from Cargo.toml

use lazy_static::lazy_static;
use regex::*;
use unwrap::unwrap;

lazy_static! {
    pub static ref CARGO_TOML: cargo_toml::Manifest =
        unwrap!(cargo_toml::Manifest::from_path("Cargo.toml"));
    pub static ref PACKAGE: cargo_toml::Package = unwrap!(CARGO_TOML.package.as_ref()).to_owned();
    static ref REGEX_REMOVE_EMAIL: Regex = unwrap!(Regex::new(r#"( <.+?>)"#));
}

/// Cargo.toml package name
pub fn package_name() -> String {
    PACKAGE.name.to_string()
}

/// Cargo.toml package version
pub fn package_version() -> String {
    PACKAGE.version.to_string()
}

/// Cargo.toml package authors as string
pub fn package_authors_string() -> String {
    let authors = crate::utils_mod::concatenate_vec_to_string(&PACKAGE.authors, ", ");
    authors
}

/// Cargo.toml package authors as string without emails
pub fn package_authors_string_without_emails() -> String {
    let authors = package_authors_string();
    let authors = REGEX_REMOVE_EMAIL.replace_all(&authors, "").to_string();
    authors
}

/// Cargo.toml package repository
pub fn package_repository() -> Option<String> {
    PACKAGE.repository.to_owned()
}

/// Cargo.toml package repository
pub fn package_description() -> Option<String> {
    PACKAGE.description.to_owned()
}
