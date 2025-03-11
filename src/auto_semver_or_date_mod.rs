// auto_semver_or_date_mod.rs

//! If the major number is greater than 2000, it is a date-version else it is semver.

use crate::public_api_mod::{RESET, YELLOW};

// this trait must be in scope to use these methods of CargoToml
use crate::public_api_mod::CargoTomlPublicApiMethods;

/// Increment the version in Cargo.toml.
///
/// If the major version is greater than 2000, it is a date version  
/// else it is semver and increments the patch part.
pub fn auto_version_increment_semver_or_date() {
    println!("  {YELLOW}Running auto_semver_or_date{RESET}");
    let cargo_toml = crate::auto_cargo_toml_mod::CargoToml::read();
    let version = cargo_toml.package_version();
    let version = semver::Version::parse(&version).unwrap();
    if version.major > 2000 {
        crate::auto_version_from_date_mod::auto_version_from_date();
    } else {
        crate::auto_semver_mod::auto_semver_increment_patch();
    }
    println!("  {YELLOW}Finished auto_semver_or_date{RESET}");
}

/// Increment the version in Cargo.toml, forced.
///
/// If the major version is greater than 2000, it is a date version
/// else it is semver and increments the patch part.
/// Forced is used in workspaces to force all members to have the same date version.
pub fn auto_version_increment_semver_or_date_forced() {
    println!("  {YELLOW}Running auto_semver_or_date{RESET}");
    let cargo_toml = crate::auto_cargo_toml_mod::CargoToml::read();
    let version = cargo_toml.package_version();
    let version = semver::Version::parse(&version).unwrap();
    if version.major > 2000 {
        crate::auto_version_from_date_mod::auto_version_from_date_forced();
    } else {
        crate::auto_semver_mod::auto_semver_increment_patch();
    }
    println!("  {YELLOW}Finished auto_semver_or_date{RESET}");
}
