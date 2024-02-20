// public_api_mod.rs

// The Public API of a library is a pain in the a...
// Every time I modify something I have to think how it will affect the users of the library.
// They could have tens or hundreds of places where they use the library. Breaking changes are terrible things.
// The developers are not willing to change their code every time a library changes slightly.
// Yes, there is the semver to show if the new library is API compatible. That helps a lot.
// It is dreaded if the first version of a function does not return a Result<>.
// Then later we will surely come to the point, that we need to return a Result<>. This is a terrible breaking change.
// It is wise to return a Result always. Even when that is not needed right now. It will surely be needed in the future.
// Another tactic is to make new functions with a different name and ornament the old functions as Obsolete.

// region: PUBLIC API defines externally accessible functions and struct.

// This library is used by the automation_tasks_rs executable.
// I want to have here the complete and exact definition of the public API.
// Therefore I will not use reexports like `pub use` or `pub mod`.
// This way I can always know easily if my public API has changed.
// Just compare the lib.rs file in git.
// Adding functions, structs, methods and enums is ok, it does not break the Public API.
// But modifying existing functions, methods or enums will break the compatibility.
// AFTERTHOUGHT: This is a very time-consuming process to do manually.
// Should use a utility, but that app is complicated to create. It must understand the Rust code.

// region: Public API constants
// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
pub const RED: &str = "\x1b[31m";
pub const YELLOW: &str = "\x1b[33m";
pub const GREEN: &str = "\x1b[32m";
pub const RESET: &str = "\x1b[0m";
// endregion: Public API constants

// region:: Public API structs and methods

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
// endregion:: Public API structs and methods

// region: Public API functions
/// find from_pos
pub fn find_from(text: &str, from_pos: usize, find: &str) -> Option<usize> {
    crate::utils_mod::find_from(text, from_pos, find)
}

/// return the position of end of the delimited data before the delimiter
pub fn find_pos_end_data_before_delimiter(
    md_text_content: &str,
    pos: usize,
    delimiter: &str,
) -> Option<usize> {
    crate::utils_mod::find_pos_end_data_before_delimiter(md_text_content, pos, delimiter)
}

/// return the position of start of the delimited data after the delimiter
pub fn find_pos_start_data_after_delimiter(
    md_text_content: &str,
    pos: usize,
    delimiter: &str,
) -> Option<usize> {
    crate::utils_mod::find_pos_start_data_after_delimiter(md_text_content, pos, delimiter)
}

/// the original `concat()` function does not have a delimiter
pub fn concatenate_vec_to_string(vec: &[String], delimiter: &str) -> String {
    crate::utils_mod::concatenate_vec_to_string(vec, delimiter)
}

/// Traverse dir and its sub-dir, but avoid excluded dirs.
/// The find_file and the exclude dir strings must start with /.
///
/// ## Example
///
/// ```
/// use std::path::Path;
///
/// let files = cargo_auto_lib::traverse_dir_with_exclude_dir(
///     Path::new("/home/project/src"),
///     "/*.rs",
///     // avoid big folders and other folders with *.crev
///     &vec![
///         "/.git".to_string(),
///         "/target".to_string(),
///         "/docs".to_string()
///     ]
/// ).unwrap();
/// for rs_file_name in files.iter() {
///     println!("{}", &rs_file_name);
/// }
/// ```
pub fn traverse_dir_with_exclude_dir(
    dir: &std::path::Path,
    find_file: &str,
    exclude_dirs: &[String],
) -> std::io::Result<Vec<String>> {
    crate::utils_mod::traverse_dir_with_exclude_dir(dir, find_file, exclude_dirs)
}

/// includes data from Cargo.toml to README.md files: version, authors,...
/// It works for workspaces and for single projects.  
/// To avoid out of sync data like version, authors and description in the README.md files, `auto_cargo_toml_to_md` includes this data from Cargo.toml.  
/// Run it on every build with [cargo auto](https://crates.io/crates/cargo-auto).  
/// It works also with other md files in the project, not only README.md.  
/// In the md file write these markers in markdown comments (invisible),  
/// don't copy the numbers 1 and 2:  
///
/// ```markdown
/// 1 [//]: # (auto_cargo_toml_to_md start)
/// 2 [//]: # (auto_cargo_toml_to_md end)
/// ```
///
/// `auto_cargo_toml_to_md` deletes the old lines between the markers and includes the Cargo.toml data:  
/// description, repository, version, &utc_now(), authors  
///
/// Run the example:  
///
/// ```bash
/// cargo run --example example_01_auto_cargo_toml_to_md
/// ```  
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
pub fn completion_return_one_or_more_sub_commands(
    sub_commands: Vec<&str>,
    word_being_completed: &str,
) {
    crate::auto_helper_functions_mod::completion_return_one_or_more_sub_commands(
        sub_commands,
        word_being_completed,
    )
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

/// inserts shield badges with lines_of_code into README.rs
/// the parameter Link will be used for shield badge. If empty_string, the git remote repository will be used.
/// Lines of code are not a "perfect" measurement of anything.\
/// Anybody can write a very big number of lines of useless code and comments.\
/// But for 95% of the cases they are good enough.\
/// Most of the developers use some "standard" coding practices and that is quantifiable and comparable.  
///
/// The `src_code_lines` is the most important count.\
/// That is actual code written for that project without  doc comments, comments, unit tests, integration tests and examples.\
/// Sometimes is great to see a big number here. It means there was a lot of work invested. But other times we want to see a small number. It
/// means the developer understands the problem very well and don't try to solve anything outside that scope.  
/// The `src_doc_comment_lines` counts doc comments. They will eventually become docs. The count of lines shows how many documentation is written.  
/// The `src_comment_lines` counts code comments. Code comments are important to understand the code. The count of lines shows how understandable is the code.  
/// The `tests_lines` counts lines in tests and shows how good is the code tested. Here are the unit tests and integration test combined.  
/// The `examples_lines` counts lines in examples and shows how good is explained how to use the code.  
///
/// ## Folder and file structure
///
/// The folder structure of a single Rust project is simple.\
/// The project starts in the folder that contains `Cargo.toml`.\
/// The `src/` folder contains all the rust `*.rs` files.\
/// The `tests/` folder contains integration tests.\
/// The `examples/` folder contains examples.\
/// Some rs files can be excluded from the count adding this line near the start of the file: // exclude from auto_lines_of_code
/// Inside a rs file the doc comment line start with `///` or `//!`.\
/// The normal comments start with `//` or `/!`.\
/// I will ignore the block comments. They are usually NOT used for comments, but to temporarily disable a piece of code. So I count this as code and not comments.  
/// The `src/*.rs` file can contain unit tests that start with `#[cfg(test)]`. I assume that these are always at the end of the file.
/// There should not be any normal code after `#[cfg(test)]`, only tests.  
/// All other files: `md`, `toml`, `html`, `js`, ... are not counted.  
///
/// ### Workspace
///
/// Workspaces have member projects, that are written in `Cargo.toml`.\
/// The program counts lines of every project and sums them together.  
///
/// ## Include into README.md
///
/// If the README.md file contains these markers (don't copy the numbers 1 and 2):  
///
/// 1. `[//]: # (auto_lines_of_code start)`  
/// 2. `[//]: # (auto_lines_of_code end)`  
///
/// the function will include the shield badges code between them.  
/// It will erase the previous content.  
/// Use git diff to see the change.  
pub fn auto_lines_of_code(link: &str) {
    crate::auto_lines_of_code_mod::auto_lines_of_code(link)
}

/// finds rs files with markers and include segments from md files
/// Includes segments of md files into rs files as doc comments.  
/// It works with workspaces and single projects.
/// From this doc comments `cargo doc` will generated the documentation and auto-completion.  
/// We don't want to manually copy this segments. We want them to be automatically in sync.  
/// We will just run this function before every `cargo doc` with an automation task.  
/// The `auto_md_to_doc_comments` function must be executed in the project root folder where is the Cargo.toml file.  
/// First it searches all the rs files in src, tests and examples folders.  
/// If they contain the markers, than finds the md file and the named segment and include it as doc comments into the rs file.  
/// The markers are always in pairs: start and end. So exactly the content in between is changed.
/// The markers are always comments, so they don't change the code.  
/// It works only for files with LF line delimiter. No CR and no CRLF.  
///
/// ## markers
///
/// In the rs file write these markers (don't copy the numbers 1 and 2 and x):  
///
/// ```code
/// 1. // xregion: auto_md_to_doc_comments include README.md //! A  
/// 2. // xendregion: auto_md_to_doc_comments include README.md //! A  
/// ```
///
/// In the md file put markers to mark the segment:  
///
/// ```markdown
/// 1. [//]: # (auto_md_to_doc_comments segment start A)  
/// 2. [//]: # (auto_md_to_doc_comments segment end A)  
/// ```
///
/// The marker must be exclusively in one line. No other text in the same line.  
/// auto_md_to_doc_comments will delete the old lines between the markers.  
/// It will find the md file and read the content between the markers.  
/// Before each line it will add the doc comment symbol as is defined in the marker.  
/// Finally it will include the new lines as doc comments in the rs file.  
/// Warning: Rustdoc introduced the #![doc=include_str!("../README.md")] macro and it is great for including the whole file.
/// But sometimes we need to include just a segment of a md file. Then we should still use this function.
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

/// Works for single projects and workspaces.  
/// new version as date is written to Cargo.toml and service_worker.js
/// In Cargo.toml writes the version as the date `yyyy.mmdd.HHMM` ex. `2019.1221.2359`.  
/// For non-library projects, the semver specification is not really useful.  
/// Having the version as the date is just fine for executables and much more human readable.  
/// The function must be executed in the root project folder of a single project or workspace where is the Cargo.toml.  
///
/// ### service_worker.js
///
/// Inside the PWA service worker javascript file is also needed to change the version.  
/// The program searches for `service_worker.js` and modify the version.  
///
/// ### no need to change version if no files changed
///
/// If src/*.rs or Cargo.toml files are not changed from last compile, than no need to change version.  
/// The dates of the files will be stored in the file .automation_tasks_rs_file_hashes.json near to Cargo.toml.
/// Warning: I don't check if the service worker has changed because it rarely does.  
/// To know if the projects has changed or not, this function saves the dates of all files into `.automation_tasks_rs_file_hashes.json` near Cargo.toml
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

/// creates a new github repository
pub fn github_api_repository_new(owner: &str, name: &str, description: &str) -> serde_json::Value {
    crate::auto_github_mod::github_api_repository_new(owner, name, description)
}
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

/// interactive ask to create a new remote GitHub repository
pub fn new_remote_github_repository() -> Option<String> {
    crate::auto_github_mod::new_remote_github_repository()
}

/// interactive ask to create a new local git repository
pub fn new_local_repository(message: &str) -> Option<()> {
    crate::auto_github_mod::new_local_repository(message)
}

/// check if this file is in ssh-add. Only the first 56 ascii characters are the fingerprint.
/// After is a description, not important and sometimes different.
/// if is not, then ssh-add and the user will enter the passcode.
pub fn ssh_add_if_needed(github_ssh_for_push: String) -> Option<()> {
    crate::auto_github_mod::ssh_add_if_needed(github_ssh_for_push)
}

/// parse the ~/.ssh/config. 99% probably there should be a record for github and there is the identity_file.
pub fn get_identity_from_ssh_config() -> String {
    crate::auto_github_mod::get_identity_from_ssh_config()
}

/// Ask the user for the filename of the ssh key used to push to GitHub.
/// The default is githubssh1.
pub fn ask_for_github_ssh_for_push() -> Option<std::path::PathBuf> {
    crate::auto_github_mod::ask_for_github_ssh_for_push()
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
pub fn github_api_create_new_release(
    owner: &str,
    repo: &str,
    tag_name_version: &str,
    name: &str,
    branch: &str,
    body_md_text: &str,
) -> String {
    crate::auto_github_mod::github_api_create_new_release(
        owner,
        repo,
        tag_name_version,
        name,
        branch,
        body_md_text,
    )
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
pub fn github_api_upload_asset_to_release(
    owner: &str,
    repo: &str,
    release_id: &str,
    path_to_file: &str,
) {
    crate::auto_github_mod::github_api_upload_asset_to_release(
        owner,
        repo,
        release_id,
        path_to_file,
    )
}

// endregion: Public API functions

// endregion: PUBLIC API defines externally accessible functions and struct
