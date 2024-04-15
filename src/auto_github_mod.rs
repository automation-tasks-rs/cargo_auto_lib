// auto_github_mod

//! GitHub API calls

use crate::CargoTomlPublicApiMethods;

use crate::{BLUE, RED, RESET, YELLOW};

// endregion: bring traits in scope

/// File contains releases changelog
pub const RELEASES_MD: &str = "RELEASES.md";

/// Check if identity is already in ssh-agent and if not then run ssh-add to add it.
///
/// Parse the ~/.ssh/config file and finds the record for host_name and there is the identity_file_path.
/// If not found, ask user for identity_file_path,
/// This ssh-add will stay even after the process ends, so the parent process will still have it.
/// returns: fingerprint or None and identity_file_name
pub fn ssh_add_resolve(host_name: &str, default_identity_file_path: &str) -> Option<(crate::auto_ssh_mod::FingerprintString, crate::auto_ssh_mod::IdentityFilePathString)> {
    // I must find the filename of the identity_file for SSH connection to host_name,
    // to find out if I need ssh-add or not.
    // 1. Parse the ~/.ssh/config file and find the record for host_name and there is the identity_file_path.
    // returns: identity_file_path contains the path like: `~/.ssh/github_com_ssh_1`
    let mut identity_file_path = get_identity_file_path_from_ssh_config(host_name);
    if identity_file_path.is_none() {
        // 2. if not found in ssh/config then ask the user to provide the filename
        if let Some(filepath) = ask_for_identity_file_path_for_ssh(host_name, default_identity_file_path) {
            identity_file_path = Some(filepath);
        } else {
            identity_file_path = None;
        }
    }
    // ssh-add only if needed
    if let Some(identity_file_path) = identity_file_path {
        let fingerprint = ssh_add_if_needed(&identity_file_path).unwrap_or_else(|| panic!("{RED}Identity not found in ssh-agent!{RESET}"));
        Some((fingerprint, identity_file_path))
    } else {
        None
    }
}

/// Check if identity is already in ssh-agent and ask user if not
///
/// Identity_private_file_path contains the path of the private key like: `~/.ssh/github_com_ssh_1`.
/// Check if this file is in ssh-add.
/// If it is not, then ask user to ssh-add and enter passphrase.
pub fn ssh_add_if_needed(identity_private_file_path: &str) -> Option<crate::auto_ssh_mod::FingerprintString> {
    let fingerprint_from_file = crate::auto_ssh_mod::get_fingerprint_from_file(identity_private_file_path);
    let mut ssh_agent_client = crate::auto_ssh_mod::crate_ssh_agent_client();
    // returns the public_key inside ssh-add or None
    match crate::auto_ssh_mod::ssh_add_list_contains_fingerprint(&mut ssh_agent_client, &fingerprint_from_file) {
        Some(_key) => (),
        None => {
            // ssh-add if it is not contained in the ssh-agent
            eprintln!("{BLUE}SSH key for GitHub push is not in the ssh-agent.{RESET}");
            println!("    {BLUE}Add SSH identity with ssh-add to use with GitHub push.{RESET}");
            // I would like to make this question BLUE, but it does not work simply.
            eprintln!("{BLUE}");
            let cmd = format!("ssh-add -t 1h {}", identity_private_file_path);
            if !crate::run_shell_command_success(&cmd) {
                eprintln!("{RED}Error: ssh-add was not successful! {RESET}");
                // early exit
                return None;
            }
            eprintln!("{RESET}");
        }
    }
    Some(fingerprint_from_file)
}

/// Parse the ~/.ssh/config file and find the record for host_name and there is the identity_file_path.
/// returns: identity_file_path contains the path like: `~/.ssh/github_com_ssh_1`
pub fn get_identity_file_path_from_ssh_config(host_name: &str) -> Option<String> {
    let mut identity_file_path = String::new();
    if let Ok(config) = ssh2_config::SshConfig::parse_default_file(ssh2_config::ParseRule::STRICT) {
        // find the filename
        for x in config.get_hosts().iter() {
            if let Some(x_host_name) = x.params.host_name.as_ref() {
                if x_host_name == host_name {
                    if let Some(identity_files) = x.params.identity_file.as_ref() {
                        if !identity_files.is_empty() {
                            // there can be more identity_files, but I will use only the first
                            identity_file_path = camino::Utf8Path::from_path(&identity_files[0]).unwrap().to_string();
                        }
                    }
                    break;
                }
            }
        }
        if identity_file_path.is_empty() {
            return None;
        }
    }
    Some(identity_file_path)
}

/// Ask the user for the filename of the SSH key used to connect with SSH/git to a server.
///
/// host_name is like: github.com or bestia.dev, default like ~/.ssh/github_com_ssh_1 and ~/.ssh/bestia_dev_ssh_1
/// returns PathBuf to identity_file_path or None
pub fn ask_for_identity_file_path_for_ssh(host_name: &str, default_identity_file_path: &str) -> Option<String> {
    println!(
        r#"{RED}Cannot find identity file in ~/.ssh/config.{RESET}
    {YELLOW}It should contain the filepath of the SSH key used for SSH connection or git to {host_name}.
    The filepath itself is not a secret. Just the content of the file is a secret.
    Without this filepath I cannot check if it is ssh-added to the ssh-agent.
    If you create the file ~/.ssh/config with content like this: 
    <https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod/raw/main/crustde_install/ssh_config_template>
    you will never be asked again to enter this filepath.{RESET}
"#,
    );
    let identity_file_for_ssh = inquire::Text::new(&format!("Which filepath has the SSH identity for {host_name}?"))
        .with_initial_value(default_identity_file_path)
        .prompt()
        .unwrap();
    if identity_file_for_ssh.is_empty() {
        // early exit
        eprintln!("{RED}Error: The filepath for the SSH key was not given. {RESET}");
        return None;
    }

    // check if the file exists
    let identity_file_for_ssh = identity_file_for_ssh.replace("~", camino::Utf8Path::from_path(&crate::home_dir()).unwrap().as_str());
    let identity_file_for_ssh = camino::Utf8Path::new(&identity_file_for_ssh).to_owned();
    if !identity_file_for_ssh.exists() {
        eprintln!("{RED}Error: File {identity_file_for_ssh} does not exist! {RESET}");
        // early exit
        return None;
    }
    let identity_file_for_ssh = identity_file_for_ssh.to_string();
    // return
    Some(identity_file_for_ssh)
}

/// sync, check, create, push git tag
pub fn git_tag_sync_check_create_push(version: &str) -> String {
    // sync the local and remote tags
    crate::run_shell_command("git fetch origin --tags --force");

    let tags = crate::run_shell_command_output("git tag").stdout;
    let tag_name_version = format!("v{}", &version);
    if !tags.contains(&format!("{}\n", tag_name_version)) {
        // create git tag and push
        let shell_command = format!("git tag -f -a {tag_name_version} -m version_{version}");
        crate::run_shell_command(&shell_command);
        crate::run_shell_command("git push origin --tags");
    }
    // return
    tag_name_version
}

/// Get release text from RELEASES.md
///
/// First, the user must write the content into file RELEASES.md in the section ## Unreleased.  
/// Then the automation task will copy the content to GitHub release  
/// and create a new Version title in RELEASES.md.  
pub fn body_text_from_releases_md(release_name: &str) -> Option<String> {
    create_releases_md_if_file_not_exist();
    let release_md = std::fs::read_to_string(RELEASES_MD).unwrap();
    // find the start of ## Unreleased
    let Some(pos_start_data) = crate::find_pos_start_data_after_delimiter(&release_md, 0, "## Unreleased\n") else {
        return None;
    };
    // find the beginning of the next ## Version
    let Some(pos_end_data) = crate::find_pos_end_data_before_delimiter(&release_md, pos_start_data, "## Version ") else {
        return None;
    };
    let body_md_text = release_md[pos_start_data..pos_end_data - 1].to_string();

    // create a new Version title after ## Unreleased in RELEASES.md
    let new_release_md = format!("{}\n## {}\n{}", &release_md[..pos_start_data], &release_name, &release_md[pos_start_data..]);
    std::fs::write(RELEASES_MD, new_release_md).unwrap();
    // return
    Some(body_md_text)
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
The ongoing changes that are not released, are visible in the git commits and github pull requests.  
The TODO section is part of the [README.md](https://github.com/{github_owner}/{project_name}).  

## Unreleased

## Version 0.0.1

"#
        );
        std::fs::write(RELEASES_MD, template).unwrap();
    }
}

/// Add commit message to Unreleased in RELEASES.md
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
