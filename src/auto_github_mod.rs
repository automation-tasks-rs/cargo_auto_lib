// auto_github_mod

use crate::CargoTomlPublicApiMethods;
use crate::GREEN;
use crate::RED;
use crate::RESET;
use crate::YELLOW;

/// creates a new github repository
pub fn api_call_repository_new(owner: &str, name: &str, description: &str) -> serde_json::Value {
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
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
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
    // APIs are very hard to change, so I expect it wil never change. Discussion (or lacking of):
    // https://github.com/orgs/community/discussions/39800
    // Add API ability to toggle releases/packages/environments from homepage of repo
    // Maybe the solution is to use a template for creating the repository???
    let body = body.to_string();

    let response_text = reqwest::blocking::Client::new()
        .post(repos_url.as_str())
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {token}"))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "github_repository_settings")
        .body(body)
        .send()
        .unwrap()
        .text()
        .unwrap();
    //pretty_dbg!(&response_text);

    let parsed: serde_json::Value = serde_json::from_str(&response_text).unwrap();
    // return
    parsed
}

/// init repository if needed. A new local git repository and remote GitHub repository.
pub fn init_repository_if_needed(message: &str) -> bool {
    let mut is_init_repository = false;

    // I must find the filename of the identity_file for ssh connection to github.com, to find out if I need ssh-add or not.
    // 1. parse the ~/.ssh/config. 99% probably there should be a record for github and there is the identity_file.
    let mut github_ssh_for_push = get_identity_from_ssh_config();

    // 2. if not found in ssh/config then ask the user to provide the filename
    if github_ssh_for_push.is_empty() {
        if let Some(filename) = ask_for_github_ssh_for_push() {
            github_ssh_for_push = filename.to_string_lossy().to_string();
        }
    }
    // ssh-add only if needed
    if !github_ssh_for_push.is_empty() {
        ssh_add_if_needed(github_ssh_for_push).unwrap();
    }
    // crate new local git repository
    if !crate::git_is_local_repository() {
        new_local_repository(message).unwrap();
    }
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

/// interactive ask to create a new remote GitHub repository
pub fn new_remote_github_repository() -> Option<String> {
    // ask interactive
    println!("{YELLOW}This project does not have a remote GitHub repository.{RESET}");
    let answer = inquire::Text::new("Do you want to create a new remote GitHub repository? (y/n)")
        .prompt()
        .unwrap();
    if answer.to_lowercase() != "y" {
        // early exit
        return None;
    }
    // continue if answer is "y"

    // region: check if env var GITHUB_TOKEN exists
    // read ENV variable GITHUB_TOKEN
    // if it does not exist, ask for it here.
    match std::env::var("GITHUB_TOKEN") {
        Ok(_g) => {}
        Err(_err) => {
            println!(
                r#"{RED}Cannot find the GITHUB_TOKEN env variable.{RESET}
GITHUB_TOKEN env variable is required to work with GitHub API to create a new repository.
You can generate the token at https://github.com/settings/tokens.
It needs the permission scope: Full control of private repositories.
The token is a secret just like a password, use it with caution.
"#
            );
            let answer = inquire::Password::new("Enter the GitHub token:")
                .without_confirmation()
                .prompt()
                .unwrap();
            if answer.is_empty() {
                // early exit
                eprintln!("{RED}The GITHUB_TOKEN was not given. Exiting.{RESET}");
                return None;
            }
            // set the env var for the token, but just for this process
            // The parent process will still be without this env var.
            std::env::set_var("GITHUB_TOKEN", answer);
        }
    }
    // endregion: check if env var GITHUB_TOKEN exists

    let cargo_toml = crate::CargoToml::read();
    let name = cargo_toml.package_name();
    let owner = cargo_toml.github_owner().unwrap();
    let description = cargo_toml.package_description().unwrap();
    let json: serde_json::Value = crate::api_call_repository_new(&owner, &name, &description);
    // get just the name, description and html_url from json
    println!("name: {}", json.get("name").unwrap().as_str().unwrap());
    println!(
        "description: {}",
        json.get("description").unwrap().as_str().unwrap()
    );
    let repo_html_url = json.get("html_url").unwrap().as_str().unwrap().to_string();
    println!("url: {}", &repo_html_url);

    // add this GitHub repository to origin remote over SSH (use sshadd for passcode)
    crate::run_shell_command(&format!(
        "git remote add origin git@github.com:{owner}/{name}.git"
    ));
    crate::run_shell_command("git push -u origin main");
    Some(repo_html_url)
}

/// interactive ask to create a new local git repository
pub fn new_local_repository(message: &str) -> Option<()> {
    // ask interactive
    println!("{YELLOW}This project is not yet a Git repository.{RESET}");
    let answer = inquire::Text::new("Do you want to initialize a new local git repository? (y/n)")
        .prompt()
        .unwrap();
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

/// check if this file is in ssh-add. Only the first 56 ascii characters are the fingerprint.
/// After is a description, not important and sometimes different.
/// if is not, then ssh-add and the user will enter the passcode.
pub fn ssh_add_if_needed(github_ssh_for_push: String) -> Option<()> {
    println!("Get a list of fingerprints already in ssh-add.");
    let ssh_added = crate::run_shell_command_output("ssh-add -l").stdout;

    println!(
        "Calculate the fingerprint of the identity file to check if it is already in ssh-add."
    );
    let fingerprint =
        crate::run_shell_command_output(&format!("ssh-keygen -lf {}", &github_ssh_for_push)).stdout
            [0..55]
            .to_string();

    // ssh-add if it is not contained in the ssh-agent
    if !ssh_added.contains(&fingerprint) {
        println!("{YELLOW}Add ssh identity with ssh-add to use with GitHub push.{RESET}");
        let cmd = format!("ssh-add -h github.com {}", github_ssh_for_push);
        let shell_output = crate::run_shell_command_output(&cmd);
        if !shell_output.stderr.contains("Identity added") {
            eprintln!("{RED}ssh-add was not successful! Exiting.{RESET}",);
            // early exit
            return None;
        } else {
            println!("{}", shell_output.stdout);
        }
    } else {
        println!("Key for GitHub push is already in ssh-add.");
    }
    Some(())
}

/// parse the ~/.ssh/config. 99% probably there should be a record for github and there is the identity_file.
pub fn get_identity_from_ssh_config() -> String {
    let mut github_ssh_for_push = String::new();
    if let Ok(config) = ssh2_config::SshConfig::parse_default_file(ssh2_config::ParseRule::STRICT) {
        // find the filename
        for x in config.get_hosts().iter() {
            if let Some(host_name) = x.params.host_name.as_ref() {
                if host_name == "github.com" {
                    if let Some(identity_files) = x.params.identity_file.as_ref() {
                        if !identity_files.is_empty() {
                            // there can be more identity_files, but I will use only the first
                            github_ssh_for_push = identity_files[0].to_string_lossy().to_string();
                        }
                    }
                    break;
                }
            }
        }
        if !github_ssh_for_push.is_empty() {
            println!("github_ssh_for_push is {github_ssh_for_push}");
        }
    }
    github_ssh_for_push
}

/// Ask the user for the filename of the ssh key used to push to GitHub.
/// The default is githubssh1.
pub fn ask_for_github_ssh_for_push() -> Option<std::path::PathBuf> {
    println!(
        r#"{RED}Cannot find identity in ~/.ssh/config.{RESET}
It should contain the filename of the ssh key used to push to GitHub.
The filename itself is not a secret. Just the content of the file is a secret.
Without this filename I cannot check if it is ssh-added to the ssh-agent.
If you create the file ~/.ssh/config with content like this: 
<https://github.com/bestia-dev/docker_rust_development/raw/main/docker_rust_development_install/ssh_config_template>
You will not be asked to enter this filename manually every time.
"#,
    );
    let github_ssh_for_push =
        inquire::Text::new("What file in the .ssh folder has the ssh key for push to GitHub? ")
            .with_initial_value("githubssh1")
            .prompt()
            .unwrap();
    if github_ssh_for_push.is_empty() {
        // early exit
        eprintln!("{RED}The filename for the ssh key was not given. Exiting.{RESET}");
        return None;
    }

    // check if the file exists
    let github_ssh_for_push = crate::home_dir().join(".ssh").join(github_ssh_for_push);
    if !github_ssh_for_push.exists() {
        eprintln!(
            "{RED}File {} does not exist! Exiting.{RESET}",
            github_ssh_for_push.to_string_lossy()
        );
        // early exit
        return None;
    }
    // return
    Some(github_ssh_for_push)
}
