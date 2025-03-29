// auto_github_mod

//! Functions to work with GitHub.

// bring trait into scope
use crate::{CargoTomlPublicApiMethods, ShellCommandLimitedDoubleQuotesSanitizerTrait};

/// File contains releases changelog
pub const RELEASES_MD: &str = "RELEASES.md";

/// Sync, check, create, push git tag.
pub fn git_tag_sync_check_create_push(version: &str) -> String {
    // sync the local and remote tags
    crate::run_shell_command_static("git fetch origin --tags --force").unwrap_or_else(|e| panic!("{e}"));

    let tags = crate::run_shell_command_output("git tag").stdout;
    let tag_name_version = format!("v{}", &version);
    if !tags.contains(&format!("{}\n", tag_name_version)) {
        // create git tag and push
        crate::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"git tag -f -a "{tag_name_version}" -m "version_{version}" "#)
            .unwrap_or_else(|e| panic!("{e}"))
            .arg("{tag_name_version}", &tag_name_version)
            .unwrap_or_else(|e| panic!("{e}"))
            .arg("{version}", version)
            .unwrap_or_else(|e| panic!("{e}"))
            .run()
            .unwrap_or_else(|e| panic!("{e}"));

        crate::run_shell_command_static("git push origin --tags").unwrap_or_else(|e| panic!("{e}"));
    }
    // return
    tag_name_version
}

/// Get release text from RELEASES.md.
///
/// First, the user must write the content into file RELEASES.md in the section ## Unreleased.  
/// Then the automation task will copy the content to GitHub release  
/// and create a new Version title in RELEASES.md.  
pub fn body_text_from_releases_md() -> Option<String> {
    create_releases_md_if_file_not_exist();
    let release_md = std::fs::read_to_string(RELEASES_MD).unwrap();
    // find the start of ## Unreleased
    let pos_start_data = crate::find_pos_start_data_after_delimiter(&release_md, 0, "## Unreleased\n")?;
    // find the beginning of the next ## Version
    let pos_end_data = crate::find_pos_end_data_before_delimiter(&release_md, pos_start_data, "## Version ")?;
    let body_md_text = release_md[pos_start_data..pos_end_data - 1].to_string();

    // return
    Some(body_md_text)
}

/// Create a new Version title in RELEASES.md.
pub fn create_new_version_in_releases_md(release_name: &str) -> Option<()> {
    create_releases_md_if_file_not_exist();
    let release_md = std::fs::read_to_string(RELEASES_MD).unwrap();
    // find the start of ## Unreleased
    let pos_start_data = crate::find_pos_start_data_after_delimiter(&release_md, 0, "## Unreleased\n")?;

    // create a new Version title after ## Unreleased in RELEASES.md
    let new_release_md = format!(
        "{}\n## {}\n{}",
        &release_md[..pos_start_data],
        &release_name,
        &release_md[pos_start_data..]
    );
    std::fs::write(RELEASES_MD, new_release_md).unwrap();
    // return
    Some(())
}

/// Create RELEASES.md if file not exist
fn create_releases_md_if_file_not_exist() {
    if !camino::Utf8Path::new(RELEASES_MD).exists() {
        // create the template file
        let cargo_toml = crate::CargoToml::read();
        let project_name = cargo_toml.package_name();
        let github_owner = cargo_toml.github_owner().unwrap();
        let template = format!(
            r#"# Releases changelog of {project_name}

All notable changes to this project will be documented in this file.  
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).  
The library releases will be published on crates.io.  
The cargo-auto automation task will use the content of the section `## Unreleased` to create
the GitHub release consistently with this file.  
The ongoing changes that are not released, are visible in the git commits and GitHub pull requests.  
The TODO section is part of the [README.md](https://github.com/{github_owner}/{project_name}).  

## Unreleased

## Version 0.0.1

"#
        );
        std::fs::write(RELEASES_MD, template).unwrap();
    }
}

/// Add commit message to Unreleased in RELEASES.md.
pub fn add_message_to_unreleased(message: &str) {
    create_releases_md_if_file_not_exist();
    let release_md = std::fs::read_to_string(RELEASES_MD).unwrap();
    // find the beginning of the first ## Version
    let Some(pos_end_data) = crate::find_pos_end_data_before_delimiter(&release_md, 0, "## Version ") else {
        return;
    };
    // add before the first ## Version
    // I expect only one empty line before ## Version
    let added_message_md = format!("{}- {}\n\n{}", &release_md[..pos_end_data], message, &release_md[pos_end_data..]);
    std::fs::write(RELEASES_MD, added_message_md).unwrap();
}
