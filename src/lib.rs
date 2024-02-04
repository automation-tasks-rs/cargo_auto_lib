// region: auto_md_to_doc_comments include README.md A //!
//! # cargo_auto_lib
//!
//! **Library crate for common tasks when building rust projects. Intended for use with cargo-auto - automation tasks written in Rust language.**  
//! ***version: 1.0.78 date: 2024-02-03 author: [Bestia.dev](https://bestia.dev) repository: [Github](https://github.com/bestia-dev/cargo_auto_lib)***  
//!
//!  [![crates.io](https://img.shields.io/crates/v/cargo_auto_lib.svg)](https://crates.io/crates/cargo_auto_lib)
//!  [![Documentation](https://docs.rs/cargo_auto_lib/badge.svg)](https://docs.rs/cargo_auto_lib/)
//!  [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/cargo_auto_lib.svg)](https://web.crev.dev/rust-reviews/crate/cargo_auto_lib/)
//!   [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/cargo_auto_lib/)
//!  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo_auto_lib/blob/master/LICENSE)
//!  [![Rust](https://github.com/bestia-dev/cargo_auto_lib/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//!  ![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/276360626.svg)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1458-green.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-394-blue.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-179-purple.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-30-yellow.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-94-orange.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//!
//! Hashtags: #rustlang #buildtool #developmenttool  
//! My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).
//!
//! ## Try it
//!
//! In your rust project root directory (where the `Cargo.toml` is) first, install [cargo-auto](https://crates.io/crates/cargo-auto) and generate a new helper project:
//!
//! ```bash
//! cargo install cargo-auto
//! cargo auto new
//! ```
//!
//! In a new editor open the generated directory `automation_tasks_rs` as an independent rust project. There is already this dependency inside `Cargo.toml`:  
//!
//! ```toml
//! cargo_auto_lib="0.8.60"
//! ```
//!
//! Preview the code and observe all the `auto_*` functions from `cargo_auto_lib`.  
//! Example:  
//!
//! ```rust ignore
//! fn task_release() {  
//!     auto_semver_increment_patch();  
//!     auto_cargo_toml_to_md();  
//!     auto_lines_of_code("");  
//!     auto_build();  
//! }  
//! ```
//!
//! Go back to your main rust project.  
//! Add markers to the beginning of README.md (don't copy the numbers 1 and 2):  
//!
//! ```md
//! 1 [//]: # (auto_cargo_toml_to_md start)
//! 2 [//]: # (auto_cargo_toml_to_md end)
//! ```
//!
//! Run (in your main rust project):
//!
//! ```bash
//! cargo auto release
//! ```
//!
//! With a little luck, it included the data of `Cargo.toml` into the `README.md` inside the markers:  
//!
//! ![auto_cargo_toml_to_md](https://github.com/bestia-dev/cargo_auto_lib/raw/main/images/auto_cargo_toml_to_md.png "auto_cargo_toml_to_md")
//!
//! ## based on simple functions
//!
//! All the functions have extensive help/docs to describe how they work.  
//! It is nice when you use a code editor with IntelliSense like VSCode.  
//!
//! ## Caveats
//!
//! This crate will attempt to edit `Cargo.toml`. Unfortunately, there's no great robust way right now to edit TOML file preserving formatting and comments and such, so right now I use just regex to do this.
//! If you find that the heuristics don't work for you though please let me know and I'll try to check in a fix!
//!
//! ## learn something new every day
//!
//! I needed to copy large text into doc comments.  
//! It means every line must get a prefix like `///`.  
//! In VSCode I selected the text, press  
//! `alt+shift+i`
//! Now I have `multiple cursors` on the end of every selected line.  
//! I press the key
//! `home`
//! And now I have multiple cursors at the beginning of every line.  
//! I type (insert):
//! `///`  
//! and it's done! Great!
//!
//! ## TODO
//!
//! Automate badges for crates.io, doc.rs, lib.rs, license, crev review. Check if they exist and create badges.
//!
//! ## cargo crev reviews and advisory
//!
//! We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).  
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
//! to verify the trustworthiness of each of your dependencies.  
//! Please, spread this info.  
//! You can also read reviews quickly on the web:  
//! <https://web.crev.dev/rust-reviews/crates/>  
//!
//! ## Open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻
//!
//! [//bestia.dev](https://bestia.dev)  
//! [//github.com/bestia-dev](https://github.com/bestia-dev)  
//! [//bestiadev.substack.com](https://bestiadev.substack.com)  
//! [//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

// region: mod, extern and use statements
mod auto_cargo_toml_mod;
mod auto_cargo_toml_to_md_mod;
mod auto_check_micro_xml_mod;
mod auto_delete_old_js_snippets_mod;
mod auto_helper_functions_mod;
mod auto_lines_of_code_mod;
mod auto_md_to_doc_comments_mod;
mod auto_plantuml_mod;
mod auto_semver_mod;
mod auto_semver_or_date_mod;
mod auto_version_from_date_mod;
mod error_mod;

pub mod utils_mod;

// reexport functions for callers of the library
// TODO: It would be nice to have a test that checks that this exported functions did not change the signature or api
// Once the library is deployed it is bad to change the api.
// There should be a way to check that the new api didn't change in an incompatible way.
// Adding functions, structs and enums is ok. Modifying existing one will break the compatibility.
pub use auto_cargo_toml_mod::CargoToml;
pub use auto_cargo_toml_to_md_mod::auto_cargo_toml_to_md;
pub use auto_check_micro_xml_mod::auto_check_micro_xml;
pub use auto_delete_old_js_snippets_mod::auto_delete_old_js_snippets;
pub use auto_helper_functions_mod::completion_return_one_or_more_sub_commands;
pub use auto_helper_functions_mod::exit_if_not_run_in_rust_project_root_directory;
pub use auto_helper_functions_mod::run_shell_command;
pub use auto_helper_functions_mod::run_shell_commands;
pub use auto_lines_of_code_mod::auto_lines_of_code;
pub use auto_md_to_doc_comments_mod::auto_md_to_doc_comments;
pub use auto_plantuml_mod::auto_plantuml;
pub use auto_plantuml_mod::auto_plantuml_for_path;
pub use auto_plantuml_mod::hash_for_filename;
pub use auto_semver_mod::auto_semver_increment_minor;
pub use auto_semver_mod::auto_semver_increment_minor_forced;
pub use auto_semver_mod::auto_semver_increment_patch;
pub use auto_semver_mod::auto_semver_increment_patch_forced;
pub use auto_semver_or_date_mod::auto_version_increment_semver_or_date;
pub use auto_semver_or_date_mod::auto_version_increment_semver_or_date_forced;
pub use auto_version_from_date_mod::auto_version_from_date;
pub use auto_version_from_date_mod::auto_version_from_date_forced;
pub use error_mod::LibError;

pub use auto_helper_functions_mod::GREEN;
pub use auto_helper_functions_mod::RED;
pub use auto_helper_functions_mod::RESET;
pub use auto_helper_functions_mod::YELLOW;
