// public_api_mod.rs

// region: auto_md_to_doc_comments include doc_comments_long/public_api_mod.md A //!
//! # public_api_mod
//!
//! The Public API of a library is a pain in the a...  
//! Every time I modify something I have to think how it will affect the users of the library.  
//! They could have tens or hundreds of places where they use the library. Breaking changes are terrible things.  
//! The developers are not willing to change their code every time a library changes slightly.  
//! Yes, there is the semver to show if the new library is API compatible. That helps a lot.  
//! It is dreaded if the first version of a function does not return a Result<>.  
//! Then later we will surely come to the point, that we need to return a Result<>. This is a terrible breaking change.  
//! It is wise to return a Result always. Even when that is not needed right now. It will surely be needed in the future.  
//! Another tactic is to make new functions with a different name and ornament the old functions as Obsolete.
//!
//! This library is used by the automation_tasks_rs executable.  
//! I want to have here the complete and exact definition of the public API.  
//! Therefore I will not use reexports like `pub use` or `pub mod`.  
//! This way I can always know easily if my public API has changed.  
//! Just compare the lib.rs file in git.  
//! Adding functions, structs, methods and enums is ok, it does not break the Public API.  
//! But modifying existing functions, methods or enums will break the compatibility.  
//! AFTERTHOUGHT: This is a very time-consuming process to do manually.  
//! Should use a utility, but that app is complicated to create. It must understand the Rust code.
//!
// endregion: auto_md_to_doc_comments include doc_comments_long/public_api_mod.md A //!

// region: Public API constants
// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
/// ANSI color
pub const RED: &str = "\x1b[31m";
/// ANSI color
pub const YELLOW: &str = "\x1b[33m";
/// ANSI color
pub const GREEN: &str = "\x1b[32m";
/// ANSI color
pub const RESET: &str = "\x1b[0m";
// endregion: Public API constants

// region: Public API structs and methods

pub use crate::error_mod::ResultWithLibError;

pub use crate::auto_helper_functions_mod::ShellOutput;
/// A string that contains secret text
pub use crate::auto_ssh_mod::SecretString;

// reexporting a struct needs to export the trait to also reexports all the methods
pub use crate::auto_cargo_toml_mod::CargoToml;

// Just for making the struct methods obvious as public methods
// I created a special trait for this struct
// AFTERTHOUGHT: This makes me think if most the functions are methods,
// then it is consistent how to make the public API definition.
// There is a downside: the caller must bring the trait into scope. A little annoying.

/// trait with methods to read data from Cargo.toml
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
    /// Cargo.toml package keywords
    fn package_keywords(&self) -> Vec<String>;
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

// region: auto_md_to_doc_comments include doc_comments_long/traverse_dir_with_exclude_dir.md A ///
/// Traverse dir and its sub-dir, but avoid excluded dirs.
///
/// The find_file and the exclude dir strings must start with /.
///
/// ## Example
///
/// ```Rust
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
///
// endregion: auto_md_to_doc_comments include doc_comments_long/traverse_dir_with_exclude_dir.md A ///
pub fn traverse_dir_with_exclude_dir(dir: &std::path::Path, find_file: &str, exclude_dirs: &[String]) -> std::io::Result<Vec<String>> {
    crate::utils_mod::traverse_dir_with_exclude_dir(dir, find_file, exclude_dirs)
}

// region: auto_md_to_doc_comments include doc_comments_long/auto_cargo_toml_to_md.md A ///
/// This function includes data from Cargo.toml to markdown files.  
///
/// This is nice for avoiding out of sync data.  
/// Run it on every build with `automation_tasks_rs` and [cargo auto](https://crates.io/crates/cargo-auto).  
///   
/// In the md file write these markers in invisible markdown comments.
///
/// ```markdown
/// [comment]: # (auto_cargo_toml_to_md start)
///
/// [comment]: # (auto_cargo_toml_to_md end)
///
/// ```
///
/// In your markdown, change the word `[comment]` with double slash `[//]`.
///
/// `auto_cargo_toml_to_md` deletes the old lines between the markers and includes the Cargo.toml data:  
/// description, repository, version, &utc_now(), authors and creates badges for keywords and categories.
///
/// The words topics, keywords and tags all mean the same concept.
/// In cargo.toml we have keywords.
/// In README.md I want to have badges, but I don't know the color yet.
/// In GitHub they are topics.
///
/// Some keywords have defined colors, others are orange like Rust.
/// This can be expanded in the future.
/// Yellow: work-in-progress
/// Green: maintained, ready-for-use
/// Red: obsolete, archived
///
/// Run the example:  
///
/// ```bash
/// cargo run --example example_01_auto_cargo_toml_to_md
/// ```
///
// endregion: auto_md_to_doc_comments include doc_comments_long/auto_cargo_toml_to_md.md A ///
pub fn auto_cargo_toml_to_md() {
    crate::auto_cargo_toml_to_md_mod::auto_cargo_toml_to_md()
}

/// I want html pages to be correct microXML when I use them for single page application.
///
/// Before build or release this function will check for correctness.
pub fn auto_check_micro_xml(path_to_html_pages: &str) {
    crate::auto_check_micro_xml_mod::auto_check_micro_xml(path_to_html_pages)
}

/// deletes old js snippets when working with wasm-pack  
///
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

// region: auto_md_to_doc_comments include doc_comments_long/exit_if_not_run_in_rust_project_root_directory.md A ///
/// Check if the code was run inside the Rust project root directory.  
///
/// There must be the `Cargo.toml` file and the directory `automation_tasks_rs`  
/// If not, exit with error message.  
///
// endregion: auto_md_to_doc_comments include doc_comments_long/exit_if_not_run_in_rust_project_root_directory.md A ///
pub fn exit_if_not_run_in_rust_project_root_directory() {
    crate::auto_helper_functions_mod::exit_if_not_run_in_rust_project_root_directory()
}

/// Run one shell command
///
/// Exit task execution if the command has Exit Status != 0.
pub fn run_shell_command(shell_command: &str) {
    crate::auto_helper_functions_mod::run_shell_command(shell_command)
}

// region: auto_md_to_doc_comments include doc_comments_long/auto_lines_of_code.md A ///
/// This function inserts shield badges with lines_of_code into README.rs.  
///
/// The parameter Link will be used for shield badge. If empty_string, the git remote repository will be used.  
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
/// ```md
/// [comment]: # (auto_lines_of_code start)
///
/// [comment]: # (auto_lines_of_code end)
/// ```
///
/// In your markdown, change the word `[comment]` with double slash `[//]`.  
///
/// The function will include the shield badges code between them.  
/// It will erase the previous content.  
/// Use git diff to see the change.  
///
// endregion: auto_md_to_doc_comments include doc_comments_long/auto_lines_of_code.md A ///
pub fn auto_lines_of_code(link: &str) {
    crate::auto_lines_of_code_mod::auto_lines_of_code(link)
}

// region: auto_md_to_doc_comments include doc_comments_long/auto_md_to_doc_comments.md A ///
/// This function finds rs files with markers and include segments from md files as doc comments.  
///
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
/// In the rs file write these markers:  
///
/// ```code
/// comment region: auto_md_to_doc_comments include README.md A ///
/// comment endregion: auto_md_to_doc_comments include README.md A ///
/// ```
///
/// In your rust code, change the word `comment` with double slash `//`.  
/// In the md file put markers to mark the segment:  
///
/// ```markdown
/// [comment]: # (auto_md_to_doc_comments segment start A)  
/// [comment]: # (auto_md_to_doc_comments segment end A)  
/// ```
///
/// In your markdown, change the word `[comment]` with double slash `[//]`.
///
/// The marker must be exclusively in one line. No other text in the same line.  
/// auto_md_to_doc_comments will delete the old lines between the markers.  
/// It will find the md file and read the content between the markers.  
/// Before each line it will add the doc comment symbol as is defined in the marker.  
/// Finally it will include the new lines as doc comments in the rs file.
///
// endregion: auto_md_to_doc_comments include doc_comments_long/auto_md_to_doc_comments.md A ///
pub fn auto_md_to_doc_comments() {
    crate::auto_md_to_doc_comments_mod::auto_md_to_doc_comments()
}

/// process plantuml in current directory
///
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

/// hash for file
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

/// Increments the version in Cargo.toml
///
/// If the major version is greater than 2000, it is a date version  
/// else it is semver and increments the patch part.  
pub fn auto_version_increment_semver_or_date() {
    crate::auto_semver_or_date_mod::auto_version_increment_semver_or_date()
}

/// Increments the version in Cargo.toml
///
/// if the major version is greater than 2000, it is a date version
/// forced is used in workspaces to force all members to have the same date version
/// else it is semver and increments the patch part
pub fn auto_version_increment_semver_or_date_forced() {
    crate::auto_semver_or_date_mod::auto_version_increment_semver_or_date_forced()
}

// region: auto_md_to_doc_comments include doc_comments_long/auto_version_from_date.md A ///
/// New version from date is written to Cargo.toml and service_worker.js
///
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
///
// endregion: auto_md_to_doc_comments include doc_comments_long/auto_version_from_date.md A ///
pub fn auto_version_from_date() {
    crate::auto_version_from_date_mod::auto_version_from_date()
}

/// Just like auto_version_from_date(), but force the new version even if no files are changed.
///
/// For workspaces `release` I want to have the same version in all members.  
/// It is slower, but easier to understand when deployed.
pub fn auto_version_from_date_forced() {
    crate::auto_version_from_date_mod::auto_version_from_date_forced()
}

/// Pretty HTML for docs
///
/// The HTML generated by `cargo doc` is ugly and difficult to `git diff`.
/// Tidy HTML is a HTML checker and formatter installed on most Linuxes.
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

/// run one shell command and return ShellOutput {exit_status, stdout, stderr}
pub fn run_shell_command_output(shell_command: &str) -> ShellOutput {
    crate::auto_helper_functions_mod::run_shell_command_output(shell_command)
}

/// run one shell command and return true if success
pub fn run_shell_command_success(shell_command: &str) -> bool {
    crate::auto_helper_functions_mod::run_shell_command_success(shell_command)
}

/// home_dir() using the home crate.
///
/// panics if HOME not found
pub fn home_dir() -> std::path::PathBuf {
    crate::auto_helper_functions_mod::home_dir()
}

/// Init repository if needed
///
/// A new local git repository and remote GitHub repository will be crated.
pub fn init_repository_if_needed(message: &str) -> bool {
    crate::auto_github_mod::init_repository_if_needed(message)
}

/// Create new release on Github
pub fn github_api_create_new_release(owner: &str, repo: &str, tag_name_version: &str, name: &str, branch: &str, body_md_text: &str) -> String {
    crate::auto_github_mod::github_api_create_new_release(owner, repo, tag_name_version, name, branch, body_md_text)
}

/// Upload asset to github release  
pub fn github_api_upload_asset_to_release(owner: &str, repo: &str, release_id: &str, path_to_file: &str) {
    crate::auto_github_mod::github_api_upload_asset_to_release(owner, repo, release_id, path_to_file)
}

/// sync, check, create, push git tag
pub fn git_tag_sync_check_create_push(version: &str) -> String {
    crate::auto_github_mod::git_tag_sync_check_create_push(version)
}

/// Get release text from RELEASES.md
///
/// First, the user must write the content into file RELEASES.md in the section ## Unreleased.  
/// Then the automation task will copy the content to GitHub release  
/// and create a new Version title in RELEASES.md.  
pub fn body_text_from_releases_md(release_name: &str) -> Option<String> {
    crate::auto_github_mod::body_text_from_releases_md(release_name)
}

/// UTC  date in iso standard like 2024-12-31
pub fn now_utc_date_iso() -> String {
    crate::auto_github_mod::now_utc_date_iso()
}

/// copy all files from the folder into a module as strings (static &str)
///
/// the module has the markers: region: files copied into strings by automation tasks and endregion:
/// the string will be in a vector with the file name
/// first we create the complete text, then we check if the old text needs to be replaced
/// ext_for_binary_files=vec![".ico",".jpg",".png",".woff2"];
/// exclude_big_folders = vec!["/.git","/target","/docs"];
pub fn copy_folder_files_into_module(folder_path: &std::path::Path, module_path: &std::path::Path, ext_for_binary_files: &[&str], exclude_big_folders: &[String]) {
    crate::auto_copy_files_to_strings_mod::copy_folder_files_into_module(folder_path, module_path, ext_for_binary_files, exclude_big_folders)
}

/// Add commit message to Unreleased in RELEASES.md
pub fn add_message_to_unreleased(message: &str) {
    crate::auto_github_mod::add_message_to_unreleased(message)
}

/// process code for playground for Rust code segments in all md files
pub fn auto_playground_run_code() {
    crate::auto_playground_mod::auto_playground_run_code()
}

/// Check and modify the description and topics on Github
///
/// The words topics, keywords and tags all mean the same concept.
/// In cargo.toml we have keywords.
/// In README.md I want to have badges for tags
/// In GitHub they are topics.
/// Topic must be only one word: lowercase letters, hyphens(-) or numbers, less then 35 characters.
pub fn description_and_topics_to_github() {
    crate::auto_github_mod::description_and_topics_to_github()
}

/// Publish to crates.io
///
/// Encrypt/decrypt the crates.io token with the GitHub ssh key.
/// Then call the `cargo publish --token token` command.
/// Never show the secret token anywhere.
pub fn publish_to_crates_io_with_secret_token() {
    crate::auto_crates_io_mod::publish_to_crates_io_with_secret_token()
}

// endregion: Public API functions
