// auto_github_mod

use crate::CargoTomlPublicApiMethods;
use crate::SecretString;
use crate::GREEN;
use crate::RED;
use crate::RESET;
use crate::YELLOW;

// region: bring traits in scope

use zeroize::Zeroize;

// endregion: bring traits in scope

pub const RELEASES_MD: &str = "RELEASES.md";
// file contains github token encrypted with github_com_ssh_1
pub const GITHUB_TOKEN_PATH: &str = "~/.ssh/github_com_data_1.ssh";

/// create new release on Github  
/// return release_id  
/// <https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token>  
/// ```ignore
///       let release_id =  github_create_new_release(&owner, &repo, &version, &name, branch, body_md_text);  
///       println!("release_id={release_id}");
///       upload_asset_to_github_release(&owner, &repo, &release_id, &path_to_file);  
///       println!("Asset uploaded.");    
/// ```
pub fn github_api_create_new_release(owner: &str, repo: &str, tag_name_version: &str, name: &str, branch: &str, body_md_text: &str) -> String {
    /*
    https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#create-a-release
    Request like :
    curl -L \
    -X POST \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer <YOUR-TOKEN>"\
    -H "X-GitHub-Api-Version: 2022-11-28" \
    https://api.github.com/repos/OWNER/REPO/releases \
    -d '
    {
        "tag_name":"v1.0.0",
        "target_commitish":"master",
        "name":"v1.0.0",
        "body":"Description of the release",
        "draft":false,
        "prerelease":false,
        "generate_release_notes":false
    }'

    Response (short)
    {
    "id": 1,
    ...
    }
    */
    let mut token = check_or_get_github_token().unwrap();
    let releases_url = format!("https://api.github.com/repos/{owner}/{repo}/releases");
    let body = serde_json::json!({
        "tag_name": tag_name_version,
        "target_commitish":branch,
        "name":name,
        "body":body_md_text,
        "draft":false,
        "prerelease":false,
        "generate_release_notes":false,
    });
    let body = body.to_string();

    let response_text = reqwest::blocking::Client::new()
        .post(releases_url.as_str())
        .header("Content-Type", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", token.0))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
        .body(body)
        .send()
        .unwrap()
        .text()
        .unwrap();

    token.0.zeroize();

    let parsed: serde_json::Value = serde_json::from_str(&response_text).unwrap();
    let new_release_id = parsed.get("id").unwrap().as_i64().unwrap().to_string();
    new_release_id
}

/// upload asset to github release  
/// release_upload_url example: <https://uploads.github.com/repos/owner/repo/releases/48127727/assets>  
/// <https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token>  
/// async function can be called from sync code:  
/// ```ignore
///       let release_id =  github_create_new_release(&owner, &repo, &version, &name, branch, body_md_text);  
///       println!("release_id={release_id}");
///       upload_asset_to_github_release(&owner, &repo, &release_id, &path_to_file);  
///       println!("Asset uploaded.");  
/// ```
pub fn github_api_upload_asset_to_release(owner: &str, repo: &str, release_id: &str, path_to_file: &str) {
    let mut token = check_or_get_github_token().unwrap();

    println!("    {YELLOW}Uploading file to GitHub release: {path_to_file}{RESET}");
    let file = std::path::Path::new(&path_to_file);
    let file_name = file.file_name().unwrap().to_str().unwrap();

    let release_upload_url = format!("https://uploads.github.com/repos/{owner}/{repo}/releases/{release_id}/assets");
    let mut release_upload_url = <url::Url as std::str::FromStr>::from_str(&release_upload_url).unwrap();
    release_upload_url.set_query(Some(format!("{}={}", "name", file_name).as_str()));
    let file_size = std::fs::metadata(file).unwrap().len();
    println!("    {YELLOW}It can take some time to upload. File size: {file_size}. Wait...{RESET}");
    // region: async code made sync locally
    use tokio::runtime::Runtime;
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        let file = tokio::fs::File::open(file).await.unwrap();
        let stream = tokio_util::codec::FramedRead::new(file, tokio_util::codec::BytesCodec::new());
        let body = reqwest::Body::wrap_stream(stream);

        let _response = reqwest::Client::new()
            .post(release_upload_url.as_str())
            .header("Content-Type", "application/octet-stream")
            .header("Content-Length", file_size.to_string())
            .header("Authorization", format!("Bearer {}", token.0))
            .body(body)
            .send()
            .await
            .unwrap();

        token.0.zeroize();

        // dbg!(response);
    });
    // endregion: async code made sync locally
}

/// creates a new github repository
pub fn github_api_repository_new(owner: &str, name: &str, description: &str) -> serde_json::Value {
    /*
    https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#create-a-repository-for-the-authenticated-user

    Request like :
    curl -L \
    -X POST \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer <YOUR-TOKEN>" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    https://api.github.com/user/repos \
    -d '{
        "name":"Hello-World",
        "description":"This is your first repo!",
        "homepage":"https://github.com",
        "private":false,
        "is_template":true
    }'

    Response (short)
    {
    "id": 1296269,
    ...
    }
    */
    let mut token = check_or_get_github_token().unwrap();
    let repos_url = format!("https://api.github.com/user/repos");
    let body = serde_json::json!({
        "name": name,
        "description": description,
        "homepage": format!("https://{owner}.github.io/{name}"),
        "private":false,
        "has_issues":true,
        "has_projects":false,
        "has_wiki":false,
        // more settings...
        "has_discussions" :true
    });
    // Sadly there is no way in the API to set the settings: releases, packages and deployments
    let body = body.to_string();

    let response_text = reqwest::blocking::Client::new()
        .post(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", token.0))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
        .body(body)
        .send()
        .unwrap()
        .text()
        .unwrap();
    //pretty_dbg!(&response_text);
    token.0.zeroize();

    let parsed: serde_json::Value = serde_json::from_str(&response_text).unwrap();
    // return
    parsed
}

/// init repository if needed. A new local git repository and remote GitHub repository.
pub fn init_repository_if_needed(message: &str) -> bool {
    // Find the filename of the identity_file for ssh connection to host_name, to find out if need ssh-add or not.
    // parse the ~/.ssh/config. 99% probably there should be a record for host_name and there is the identity_file.
    // else ask user for filename, then run ssh-add. The identity added with ssh-add will remain also in the parent process.
    let _token = check_or_get_github_token().unwrap();

    // crate new local git repository
    if !crate::git_is_local_repository() {
        new_local_repository(message).unwrap();
    }

    let mut is_init_repository = false;
    // create new remote GitHub repository
    if !crate::git_has_remote() {
        let repo_html_url = new_remote_github_repository().unwrap();
        is_init_repository = true;
        println!(
            r#"
    {YELLOW}You have successfully created a new local repository and remote GitHub repository.{RESET}
    {YELLOW}Open the browser on the GitHub repo:{RESET}
{GREEN}{repo_html_url}{RESET}
    {YELLOW}Now continue coding happily and then{RESET}
{GREEN}cargo auto build{RESET}
"#
        );
    }
    // return
    is_init_repository
}

/// Parse the ~/.ssh/config file and finds the record for host_name and there is the identity_file_path.
/// If not found, ask user for identity_file_path,
/// Check if this identity is already in ssh-agent and if not then run ssh-add to add it.
/// This ssh-add will stay even after the process ends, so the parent process will still have it.
/// returns: fingerprint or None and identity_file_name
pub fn ssh_add_resolve(host_name: &str, default_identity_file_path: &str) -> Option<(crate::auto_ssh_mod::FingerprintString, crate::auto_ssh_mod::IdentityFilePathString)> {
    // I must find the filename of the identity_file for ssh connection to host_name,
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

/// interactive ask to create a new remote GitHub repository
pub fn new_remote_github_repository() -> Option<String> {
    // ask interactive
    println!("    {YELLOW}This project does not have a remote GitHub repository.{RESET}");
    let answer = inquire::Text::new("Do you want to create a new remote GitHub repository? (y/n)").prompt().unwrap();
    if answer.to_lowercase() != "y" {
        // early exit
        return None;
    }
    // continue if answer is "y"

    let cargo_toml = crate::CargoToml::read();
    let name = cargo_toml.package_name();
    let owner = cargo_toml.github_owner().unwrap();
    let description = cargo_toml.package_description().unwrap();
    let json: serde_json::Value = crate::auto_github_mod::github_api_repository_new(&owner, &name, &description);
    // get just the name, description and html_url from json
    println!("name: {}", json.get("name").unwrap().as_str().unwrap());
    println!("description: {}", json.get("description").unwrap().as_str().unwrap());
    let repo_html_url = json.get("html_url").unwrap().as_str().unwrap().to_string();
    println!("url: {}", &repo_html_url);

    // add this GitHub repository to origin remote over SSH (use sshadd for passcode)
    crate::run_shell_command(&format!("git remote add origin git@github.com:{owner}/{name}.git"));
    crate::run_shell_command("git push -u origin main");
    Some(repo_html_url)
}

/// decrypt the token from GITHUB_TOKEN_PATH file
/// or ask user interactively to type it, then encrypt and save to file
fn check_or_get_github_token() -> Option<SecretString> {
    // ssh_add_resolve(host_name: &str, default_identity_file_path: &str)
    let (_fingerprint, identity_file_path) = ssh_add_resolve("github.com", "~/.ssh/github_com_ssh_1").unwrap();

    let mut token: Option<SecretString> = None;
    let github_token_json_path_expanded = crate::utils_mod::file_path_home_expand(GITHUB_TOKEN_PATH);
    if std::path::Path::new(&github_token_json_path_expanded).exists() {
        token = crate::auto_encrypt_decrypt_with_ssh_mod::decrypt_with_ssh_from_file(&github_token_json_path_expanded);
    }
    if token.is_none() {
        println!(
            r#"{RED}Cannot find the file with encrypted github token.{RESET}
    {YELLOW}The token is required to work with GitHub API to work with your repositories.
    You can generate the token at https://github.com/settings/tokens.
    It needs the permission scope: Full control of private repositories.
    The token is a secret just like a password, use it with caution.{RESET}
"#
        );
        // encrypt and save to file
        crate::auto_encrypt_decrypt_with_ssh_mod::encrypt_with_ssh_interactive_save_file(&identity_file_path, &github_token_json_path_expanded);
        // now decrypt
        token = crate::auto_encrypt_decrypt_with_ssh_mod::decrypt_with_ssh_from_file(&github_token_json_path_expanded);
    }
    // return
    token
}

/// interactive ask to create a new local git repository
pub fn new_local_repository(message: &str) -> Option<()> {
    // ask interactive
    println!("    {YELLOW}This project is not yet a Git repository.{RESET}");
    let answer = inquire::Text::new("Do you want to initialize a new local git repository? (y/n)").prompt().unwrap();
    // continue if answer is "y"
    if answer.to_lowercase() != "y" {
        // early exit
        return None;
    }
    // create new local git repository and commit all on branch main
    crate::run_shell_command("git config --global init.defaultBranch main");
    crate::run_shell_command("git init");
    crate::run_shell_command("git add .");
    crate::run_shell_command(&format!(r#"git commit -m "{message}""#));
    crate::run_shell_command("git branch -M main");
    Some(())
}

/// identity_private_file_path contains the path of the private key like: `~/.ssh/github_com_ssh_1`
/// check if this file is in ssh-add.
/// if is not, then ask user to ssh-add and enter passcode.
pub fn ssh_add_if_needed(identity_private_file_path: &str) -> Option<crate::auto_ssh_mod::FingerprintString> {
    let fingerprint_from_file = crate::auto_ssh_mod::get_fingerprint_from_file(identity_private_file_path);
    let mut ssh_agent_client = crate::auto_ssh_mod::crate_ssh_agent_client();
    // returns the public_key inside ssh-add or None
    match crate::auto_ssh_mod::ssh_add_list_contains_fingerprint(&mut ssh_agent_client, &fingerprint_from_file) {
        Some(_key) => (),
        None => {
            // ssh-add if it is not contained in the ssh-agent
            eprintln!("{RED}Ssh key for GitHub push is not in the ssh-agent.{RESET}");
            println!("    {YELLOW}Add ssh identity with ssh-add to use with GitHub push.{RESET}");
            let cmd = format!("ssh-add -t 1h {}", identity_private_file_path);
            if !crate::run_shell_command_success(&cmd) {
                eprintln!("{RED}ssh-add was not successful! Exiting.{RESET}",);
                // early exit
                return None;
            }
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
                            identity_file_path = identity_files[0].to_string_lossy().to_string();
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
    println!("    {YELLOW}Identity_file_path for ssh is {identity_file_path}{RESET}");

    Some(identity_file_path)
}

/// Ask the user for the filename of the ssh key used to connect with SSH/git to a server.
/// host_name is like: github.com or bestia.dev, default like ~/.ssh/github_com_ssh_1 and ~/.ssh/bestia_dev_ssh_1
/// returns PathBuf to identity_file_path or None
pub fn ask_for_identity_file_path_for_ssh(host_name: &str, default_identity_file_path: &str) -> Option<String> {
    println!(
        r#"{RED}Cannot find identity file in ~/.ssh/config.{RESET}
    {YELLOW}It should contain the filepath of the ssh key used for ssh connection or git to {host_name}.
    The filepath itself is not a secret. Just the content of the file is a secret.
    Without this filepath I cannot check if it is ssh-added to the ssh-agent.
    If you create the file ~/.ssh/config with content like this: 
    <https://github.com/bestia-dev/docker_rust_development/raw/main/docker_rust_development_install/ssh_config_template>
    you will never be asked again to enter this filepath.{RESET}
"#,
    );
    let identity_file_for_ssh = inquire::Text::new(&format!("Which filepath has the ssh identity for {host_name}?"))
        .with_initial_value(default_identity_file_path)
        .prompt()
        .unwrap();
    if identity_file_for_ssh.is_empty() {
        // early exit
        eprintln!("{RED}The filepath for the ssh key was not given. Exiting.{RESET}");
        return None;
    }

    // check if the file exists
    let identity_file_for_ssh = identity_file_for_ssh.replace("~", crate::home_dir().to_string_lossy().as_ref());
    let identity_file_for_ssh = std::path::Path::new(&identity_file_for_ssh).to_owned();
    if !identity_file_for_ssh.exists() {
        eprintln!("{RED}File {} does not exist! Exiting.{RESET}", identity_file_for_ssh.to_string_lossy());
        // early exit
        return None;
    }
    let identity_file_for_ssh = identity_file_for_ssh.to_string_lossy().to_string();
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

/// create RELEASES.md if file not exist
fn create_releases_md_if_file_not_exist() {
    if !std::path::Path::new(RELEASES_MD).exists() {
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

/// the UTC date in iso standard 2024-12-31
pub fn now_utc_date_iso() -> String {
    chrono::Utc::now().format("%Y-%m-%d").to_string()
}

/// add commit message to Unreleased in RELEASES.md
pub fn add_message_to_unreleased(message: &str) {
    create_releases_md_if_file_not_exist();
    let release_md = std::fs::read_to_string(RELEASES_MD).unwrap();
    // find the beginning of the first ## Version
    let Some(pos_end_data) = crate::find_pos_end_data_before_delimiter(&release_md, 0, "## Version ") else {
        return;
    };
    // add before the first ## Version
    // I expect only one empty line before ## Version
    let added_message_md = format!("{}- {}\n{}", &release_md[..pos_end_data - 1], message, &release_md[pos_end_data - 1..]);
    std::fs::write(RELEASES_MD, added_message_md).unwrap();
}

/// Check and modify the description and topics on Github
/// The words topics, keywords and tags all mean the same concept.
/// In cargo.toml we have keywords.
/// In README.md I want to have badges for tags
/// In GitHub they are topics.
/// Topic must be only one word: lowercase letters, hyphens(-) or numbers, less then 35 characters.
pub fn description_and_topics_to_github() {
    let cargo_toml = crate::CargoToml::read();
    let repo_name = cargo_toml.package_name();
    let owner = cargo_toml.github_owner().unwrap();
    let description = cargo_toml.package_description().unwrap();
    let keywords = cargo_toml.package_keywords();

    // get data from GitHub
    let json: serde_json::Value = github_api_get_repository(&owner, &repo_name);
    // get just the description and topis from json
    let gh_description = json.get("description").unwrap().as_str().unwrap();
    let gh_topics = json.get("topics").unwrap().as_array().unwrap();
    let gh_topics: Vec<String> = gh_topics.into_iter().map(|value| value.as_str().unwrap().to_string()).collect();

    // are description and topics both equal?
    if gh_description != description {
        github_api_update_description(&owner, &repo_name, &description);
    }

    // all elements must be equal, but not necessary in the same order
    let topics_is_equal = if gh_topics.len() == keywords.len() {
        let mut elems_is_equal = true;
        'outer: for x in gh_topics.iter() {
            let mut has_element = false;
            'inner: for y in keywords.iter() {
                if y == x {
                    has_element = true;
                    break 'inner;
                }
            }
            if !has_element {
                elems_is_equal = false;
                break 'outer;
            }
        }
        elems_is_equal
    } else {
        false
    };

    if !topics_is_equal {
        github_api_replace_all_topics(&owner, &repo_name, &keywords);
    }
}

/// github api get repository
fn github_api_get_repository(owner: &str, repo_name: &str) -> serde_json::Value {
    /*
        https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#get-a-repository

        curl -L \
        -H "Accept: application/vnd.github+json" \
        -H "Authorization: Bearer <YOUR-TOKEN>" \
        -H "X-GitHub-Api-Version: 2022-11-28" \
        https://api.github.com/repos/OWNER/REPO
    */
    let mut token = check_or_get_github_token().unwrap();
    let repos_url = format!("https://api.github.com/repos/{owner}/{repo_name}");
    let response_text = reqwest::blocking::Client::new()
        .get(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", token.0))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
        .send()
        .unwrap()
        .text()
        .unwrap();
    //pretty_dbg!(&response_text);
    token.0.zeroize();

    let parsed: serde_json::Value = serde_json::from_str(&response_text).unwrap();
    // return
    parsed
}

/// github api update description and topics
pub fn github_api_update_description(owner: &str, repo_name: &str, description: &str) {
    /*
    https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#update-a-repository

    curl -L \
    -X PATCH \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer <YOUR-TOKEN>" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    https://api.github.com/repos/OWNER/REPO \
    -d '{
        "name":"Hello-World",
        "description":"This is your first repository",
        "homepage":"https://github.com",
        "private":true,
        "has_issues":true,
        "topics": [
            "cat",
            "atom",
            "electron",
            "api"
            ],
        "has_projects":true,
        "has_wiki":true}'

    Response (short)
    {
    "id": 1296269,
    ...
    }
    */
    println!("    {YELLOW}Updating GitHub description{RESET}");
    let mut token = check_or_get_github_token().unwrap();
    let repos_url = format!("https://api.github.com/repos/{owner}/{repo_name}");
    let body = serde_json::json!({
        "description": description,
    });
    let body = body.to_string();

    let response_text = reqwest::blocking::Client::new()
        .patch(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", token.0))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
        .body(body)
        .send()
        .unwrap()
        .text()
        .unwrap();
    //pretty_dbg!(&response_text);
    token.0.zeroize();

    let _parsed: serde_json::Value = serde_json::from_str(&response_text).unwrap();
}

fn github_api_replace_all_topics(owner: &str, repo_name: &str, topics: &Vec<String>) {
    /*
    https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#replace-all-repository-topics
    curl -L \
      -X PUT \
      -H "Accept: application/vnd.github+json" \
      -H "Authorization: Bearer <YOUR-TOKEN>" \
      -H "X-GitHub-Api-Version: 2022-11-28" \
      https://api.github.com/repos/OWNER/REPO/topics \
      -d '{"names":["cat","atom","electron","api"]}'
     */
    println!("    {YELLOW}Updating GitHub topics.{RESET}");
    let mut token = check_or_get_github_token().unwrap();
    let repos_url = format!("https://api.github.com/repos/{owner}/{repo_name}/topics");
    let body = serde_json::json!({
        "names": topics,
    });
    let body = body.to_string();

    let response_text = reqwest::blocking::Client::new()
        .put(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {}", token.0))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "cargo_auto_lib")
        .body(body)
        .send()
        .unwrap()
        .text()
        .unwrap();
    //pretty_dbg!(&response_text);
    token.0.zeroize();

    let _parsed: serde_json::Value = serde_json::from_str(&response_text).unwrap();
}
