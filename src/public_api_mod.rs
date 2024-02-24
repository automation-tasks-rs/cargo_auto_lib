// public_api_mod.rs

#![doc=include_str!("../doc_comments_long/public_api_mod.md")]

// region: Public API constants
// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
pub const RED: &str = "\x1b[31m";
pub const YELLOW: &str = "\x1b[33m";
pub const GREEN: &str = "\x1b[32m";
pub const RESET: &str = "\x1b[0m";
// endregion: Public API constants

// region: Public API structs and methods

/// I use thiserror to return errors from the library.
pub use crate::error_mod::ResultWithLibError;

pub use crate::auto_helper_functions_mod::ShellOutput;

// reexporting a struct needs to export the trait to also reexports all the methods
pub use crate::auto_cargo_toml_mod::CargoToml;

// Just for making the struct methods obvious as public methods
// I created a special trait for this struct
// AFTERTHOUGHT: This makes me think if most the functions are methods,
// then it is consistent how to make the public API definition.
// There is a downside: the caller must bring the trait into scope. A little annoying.
pub trait CargoTomlPublicApiMethods {
    /// read Cargo.toml, for workspaces it is the Cargo.toml of the first member
    fn read() -> Self;
    /// Cargo.toml package name
    fn package_name(&self) -> String;
    /// Cargo.toml package version
    fn package_version(&self) -> String;
    /// Cargo.toml package authors as string
    fn package_authors_string(&self) -> String;
    /// Cargo.toml package authors as string without emails
    fn package_author_name(&self) -> String;
    /// Cargo.toml package repository
    fn package_repository(&self) -> Option<String>;
    /// Cargo.toml package description
    fn package_description(&self) -> Option<String>;
    /// Cargo.toml package homepage
    fn package_homepage(&self) -> String;
    /// Cargo.toml workspace members
    fn workspace_members(&self) -> Option<Vec<String>>;
    /// github owner from package.repository
    fn github_owner(&self) -> Option<String>;
}
// endregion: Public API structs and methods

// region: Public API functions
/// find from_pos
pub fn find_from(text: &str, from_pos: usize, find: &str) -> Option<usize> {
    crate::utils_mod::find_from(text, from_pos, find)
}

/// return the position of end of the delimited data before the delimiter
pub fn find_pos_end_data_before_delimiter(md_text_content: &str, pos: usize, delimiter: &str) -> Option<usize> {
    crate::utils_mod::find_pos_end_data_before_delimiter(md_text_content, pos, delimiter)
}

/// return the position of start of the delimited data after the delimiter
pub fn find_pos_start_data_after_delimiter(md_text_content: &str, pos: usize, delimiter: &str) -> Option<usize> {
    crate::utils_mod::find_pos_start_data_after_delimiter(md_text_content, pos, delimiter)
}

/// the original `concat()` function does not have a delimiter
pub fn concatenate_vec_to_string(vec: &[String], delimiter: &str) -> String {
    crate::utils_mod::concatenate_vec_to_string(vec, delimiter)
}

#[doc=include_str!("../doc_comments_long/traverse_dir_with_exclude_dir.md")]
pub fn traverse_dir_with_exclude_dir(dir: &std::path::Path, find_file: &str, exclude_dirs: &[String]) -> std::io::Result<Vec<String>> {
    crate::utils_mod::traverse_dir_with_exclude_dir(dir, find_file, exclude_dirs)
}

#[doc=include_str!("../doc_comments_long/auto_cargo_toml_to_md.md")]
pub fn auto_cargo_toml_to_md() {
    crate::auto_cargo_toml_to_md_mod::auto_cargo_toml_to_md()
}

/// I want html pages to be correct microXML when I use them for single page application.
/// Before build or release this function will check for correctness.
pub fn auto_check_micro_xml(path_to_html_pages: &str) {
    crate::auto_check_micro_xml_mod::auto_check_micro_xml(path_to_html_pages)
}

/// deletes old js snippets when working with wasm-pack  
/// The old folders for `js snippets` are not automatically deleted on building with `wasm-pack`.  
/// This utils do that.  
/// The function must be executed in the root project folder where is the Cargo.toml.  
pub fn auto_delete_old_js_snippets() {
    crate::auto_delete_old_js_snippets_mod::auto_delete_old_js_snippets()
}

/// println one, more or all sub_commands
pub fn completion_return_one_or_more_sub_commands(sub_commands: Vec<&str>, word_being_completed: &str) {
    crate::auto_helper_functions_mod::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed)
}

/// check if run in rust project root directory error
/// there must be Cargo.toml and the directory automation_tasks_rs
/// exit with error message if not
pub fn exit_if_not_run_in_rust_project_root_directory() {
    crate::auto_helper_functions_mod::exit_if_not_run_in_rust_project_root_directory()
}

/// run one shell command
/// Stops task execution if the command has Exit Status != 0
pub fn run_shell_command(shell_command: &str) {
    crate::auto_helper_functions_mod::run_shell_command(shell_command)
}

/// run shell commands from a vector of strings.
/// Stops task execution if oe of the commands has Exit Status != 0
pub fn run_shell_commands(shell_commands: Vec<&str>) {
    crate::auto_helper_functions_mod::run_shell_commands(shell_commands)
}

#[doc=include_str!("../doc_comments_long/auto_lines_of_code.md")]
pub fn auto_lines_of_code(link: &str) {
    crate::auto_lines_of_code_mod::auto_lines_of_code(link)
}

#[doc=include_str!("../doc_comments_long/auto_md_to_doc_comments.md")]
pub fn auto_md_to_doc_comments() {
    crate::auto_md_to_doc_comments_mod::auto_md_to_doc_comments()
}

/// process plantuml in current directory
/// finds markers (auto_plantuml start) and (auto_plantuml end) in md files
/// if needed calls the web service and saves the svg file
/// Between markers adds the link to the svg file
/// repo_url like <https://github.com/bestia-dev/sey_currency_converter_pwa>
/// so the image file link is from the repository and accessible everywhere
pub fn auto_plantuml(repo_url: &str) {
    crate::auto_plantuml_mod::auto_plantuml(repo_url)
}

/// process plantuml for all md files
/// for test and examples I need to provide the path
pub fn auto_plantuml_for_path(path: &std::path::Path, repo_url: &str) {
    crate::auto_plantuml_mod::auto_plantuml_for_path(path, repo_url)
}

pub fn hash_for_filename(text: &str) -> String {
    crate::auto_plantuml_mod::hash_for_filename(text)
}

/// Increments the minor version in Cargo.toml file only if files are changed
pub fn auto_semver_increment_minor() {
    crate::auto_semver_mod::auto_semver_increment_minor()
}

/// Increments the minor version in Cargo.toml file even if files are not changed
pub fn auto_semver_increment_minor_forced() {
    crate::auto_semver_mod::auto_semver_increment_minor_forced()
}

/// Increments the patch version in Cargo.toml file only if files are changed
pub fn auto_semver_increment_patch() {
    crate::auto_semver_mod::auto_semver_increment_patch()
}

/// Increments the patch version in Cargo.toml file even if files are not changed
pub fn auto_semver_increment_patch_forced() {
    crate::auto_semver_mod::auto_semver_increment_patch_forced()
}

/// increments the version in Cargo.toml.
/// if the major version is greater than 2000, it is a date version
/// else it is semver and increments the patch part
pub fn auto_version_increment_semver_or_date() {
    crate::auto_semver_or_date_mod::auto_version_increment_semver_or_date()
}

/// increments the version in Cargo.toml.
/// if the major version is greater than 2000, it is a date version
/// forced is used in workspaces to force all members to have the same date version
/// else it is semver and increments the patch part
pub fn auto_version_increment_semver_or_date_forced() {
    crate::auto_semver_or_date_mod::auto_version_increment_semver_or_date_forced()
}

#[doc=include_str!("../doc_comments_long/auto_version_from_date.md")]
pub fn auto_version_from_date() {
    crate::auto_version_from_date_mod::auto_version_from_date()
}

/// Works for single projects and workspaces.  
/// Just like auto_version_from_date(), but force the new version even if no files are changed.
/// For workspaces `release` I want to have the same version in all members.  
/// It is slower, but easier to understand when deployed.
pub fn auto_version_from_date_forced() {
    crate::auto_version_from_date_mod::auto_version_from_date_forced()
}

/// The HTML generated by `cargo doc` is ugly and difficult to `git diff`
/// tidy HTML is a HTML checker and formatter installed on most Linuxes.
/// If it is not installed run: `sudo apt install -y tidy`
/// From the bash you can install it inside the podman container like this:
/// `podman exec --user root rust_dev_vscode_cnt apt install -y tidy`
pub fn auto_doc_tidy_html() -> ResultWithLibError<()> {
    crate::auto_doc_tidy_html_mod::auto_doc_tidy_html()
}

/// has git remote
pub fn git_has_remote() -> bool {
    crate::auto_git_mod::git_has_remote()
}

/// check if this folder is a local Git repository
pub fn git_is_local_repository() -> bool {
    crate::auto_git_mod::git_is_local_repository()
}

/// run one shell command and return ShellOutput {exit_status,stdout,stderr}
pub fn run_shell_command_output(shell_command: &str) -> ShellOutput {
    crate::auto_helper_functions_mod::run_shell_command_output(shell_command)
}

/// home_dir() using the home crate.
/// panics if HOME not found
pub fn home_dir() -> std::path::PathBuf {
    crate::auto_helper_functions_mod::home_dir()
}

/// init repository if needed. A new local git repository and remote GitHub repository.
pub fn init_repository_if_needed(message: &str) -> bool {
    crate::auto_github_mod::init_repository_if_needed(message)
}

/// Find the filename of the identity_file for ssh connection to host_name, to find out if need ssh-add or not.
/// parse the ~/.ssh/config. 99% probably there should be a record for host_name and there is the identity_file.
/// else ask user for filename, then run ssh-add
pub fn ssh_add_resolve(host_name: &str, default_host_name: &str) {
    crate::auto_github_mod::ssh_add_resolve(host_name, default_host_name)
}

/// create new release on Github  
/// return release_id  
/// it needs env variable `export GITHUB_TOKEN=paste_github_personal_authorization_token_here`  
/// <https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token>  
/// ```ignore
///       let release_id =  github_create_new_release(&owner, &repo, &version, &name, branch, body_md_text);  
///       println!("release_id={release_id}");
///       upload_asset_to_github_release(&owner, &repo, &release_id, &path_to_file);  
///       println!("Asset uploaded.");    
/// ```
pub fn github_api_create_new_release(owner: &str, repo: &str, tag_name_version: &str, name: &str, branch: &str, body_md_text: &str) -> String {
    crate::auto_github_mod::github_api_create_new_release(owner, repo, tag_name_version, name, branch, body_md_text)
}

/// upload asset to github release  
/// release_upload_url example: <https://uploads.github.com/repos/owner/repo/releases/48127727/assets>  
/// it needs env variable `export GITHUB_TOKEN=paste_github_personal_authorization_token_here`  
/// <https://docs.github.com/en/github/authenticating-to-github/keeping-your-account-and-data-secure/creating-a-personal-access-token>  
/// async function can be called from sync code:  
/// ```ignore
///       let release_id =  github_create_new_release(&owner, &repo, &version, &name, branch, body_md_text);  
///       println!("release_id={release_id}");
///       upload_asset_to_github_release(&owner, &repo, &release_id, &path_to_file);  
///       println!("Asset uploaded.");  
/// ```
pub fn github_api_upload_asset_to_release(owner: &str, repo: &str, release_id: &str, path_to_file: &str) {
    crate::auto_github_mod::github_api_upload_asset_to_release(owner, repo, release_id, path_to_file)
}

/// sync, check, create, push git tag
pub fn git_tag_sync_check_create_push(version: &str) -> String {
    crate::auto_github_mod::git_tag_sync_check_create_push(version)
}

/// First, the user must write the content into file RELEASES.md in the section ## Unreleased.
/// Then the automation task will copy the content to GitHub release
/// and create a new Version title in RELEASES.md.
pub fn body_text_from_releases_md(release_name: &str) -> Option<String> {
    crate::auto_github_mod::body_text_from_releases_md(release_name)
}

/// the UTC date in iso standard 2024-12-31
pub fn now_utc_date_iso() -> String {
    crate::auto_github_mod::now_utc_date_iso()
}

/// copy all files from the folder into a module as strings (static &str)
/// the module has the markers: region: files copied into strings by automation tasks and endregion:
/// the string will be in a vector with the file name
/// first we create the complete text, then we check if the old text needs to be replaced
/// ext_for_binary_files=vec![".ico",".jpg",".png",".woff2"];
/// exclude_big_folders = vec!["/.git","/target","/docs"];
pub fn copy_folder_files_into_module(folder_path: &std::path::Path, module_path: &std::path::Path, ext_for_binary_files: &[&str], exclude_big_folders: &[String]) {
    crate::auto_copy_files_to_strings_mod::copy_folder_files_into_module(folder_path, module_path, ext_for_binary_files, exclude_big_folders)
}

/// add commit message to Unreleased in RELEASES.md
pub fn add_message_to_unreleased(message: &str) {
    crate::auto_github_mod::add_message_to_unreleased(message)
}

/// process code for playground for Rust code segments in all md files
pub fn auto_playground_run_code() {
    crate::auto_playground_mod::auto_playground_run_code()
}

// endregion: Public API functions
