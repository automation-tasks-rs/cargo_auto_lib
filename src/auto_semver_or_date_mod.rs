// auto_semver_or_date.rs

//! it reads the version in Cargo.toml and if the major number
//! is greater than 2000, it is a date-version, else it is semver
//! It works for workspaces and for single projects.  

use unwrap::unwrap;

/// increments the version in Cargo.toml.
/// if the major version is greater than 2000, it is a date version
/// else it is semver and increments the patch part
pub fn auto_version_increment_semver_or_date() {
    println!("Running auto_semver_or_date");
    // Cargo.toml contains the list of projects
    let cargo_toml = crate::auto_cargo_toml_mod::CargoToml::read();
    match cargo_toml.workspace_members() {
        None => one_project(),
        Some(members) => {
            for member in members.iter() {
                println!("Member: {}", member);
                unwrap!(std::env::set_current_dir(member));
                one_project();
                unwrap!(std::env::set_current_dir(".."));
            }
        }
    }
    println!("Finished auto_semver_or_date");
}

/// increments the version in Cargo.toml.
/// if the major version is greater than 2000, it is a date version
/// forced is used in workspaces to force all members to have the same date version
/// else it is semver and increments the patch part
pub fn auto_version_increment_semver_or_date_forced() {
    println!("Running auto_semver_or_date");
    // Cargo.toml contains the list of projects
    let cargo_toml = crate::auto_cargo_toml_mod::CargoToml::read();
    match cargo_toml.workspace_members() {
        None => one_project(),
        Some(members) => {
            for member in members.iter() {
                println!("{}", member);
                unwrap!(std::env::set_current_dir(member));
                one_project_forced();
                unwrap!(std::env::set_current_dir(".."));
            }
        }
    }
    println!("Finished auto_semver_or_date");
}

fn one_project() {
    let cargo_toml = crate::auto_cargo_toml_mod::CargoToml::read();
    let version = cargo_toml.package_version();
    let version = semver::Version::parse(&version).unwrap();
    if version.major > 2000 {
        crate::auto_version_from_date_mod::auto_version_from_date();
    } else {
        crate::auto_semver_mod::auto_semver_increment_patch();
    }
}

fn one_project_forced() {
    let cargo_toml = crate::auto_cargo_toml_mod::CargoToml::read();
    let version = cargo_toml.package_version();
    let version = semver::Version::parse(&version).unwrap();
    if version.major > 2000 {
        crate::auto_version_from_date_mod::auto_version_from_date_forced();
    } else {
        crate::auto_semver_mod::auto_semver_increment_patch();
    }
}
