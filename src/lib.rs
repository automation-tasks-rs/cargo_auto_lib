// region: auto_md_to_doc_comments include README.md A //!
//! # cargo_auto_lib
//!
//! **Library crate for common tasks when building rust projects. Intended for use with cargo-auto - automation tasks written in Rust language.**  
//! ***version: 1.1.20 date: 2024-02-09 author: [Bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/cargo_auto_lib)***  
//!
//!  ![status](https://img.shields.io/badge/maintained-green)
//!  ![status](https://img.shields.io/badge/ready_for_use-green)
//!
//!  [![crates.io](https://img.shields.io/crates/v/cargo_auto_lib.svg)](https://crates.io/crates/cargo_auto_lib)
//!  [![Documentation](https://docs.rs/cargo_auto_lib/badge.svg)](https://docs.rs/cargo_auto_lib/)
//!  [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/cargo_auto_lib.svg)](https://web.crev.dev/rust-reviews/crate/cargo_auto_lib/)
//!  [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/cargo_auto_lib/)
//!
//!  [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo_auto_lib/blob/master/LICENSE)
//!  [![Rust](https://github.com/bestia-dev/cargo_auto_lib/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//!  ![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/276360626.svg)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1623-green.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-624-blue.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-231-purple.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-37-yellow.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-105-orange.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
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
//! ## Add a git tag for a past commit
//!
//! How to add a tag for an old commit with its GIT_COMMITTER_DATE?  
//! <https://stackoverflow.com/questions/4404172/how-to-tag-an-older-commit-in-git>
//!
//! ```bash
//! # Set the HEAD to the old commit that we want to tag
//! git checkout e408fcd6efb21e6f0df55f3d49afd0f3917738fd
//!
//! # temporarily set the date to the date of the HEAD commit, and add the tag
//! GIT_COMMITTER_DATE="$(git show --format=%aD | head -1)" \
//! git tag -a v1.0.96 -m"v1.0.96"
//!
//! # push to origin
//! git push origin --tags
//!
//! # set HEAD back
//! git checkout main
//!
//! ```
//!
//! ## TODO
//!
//! Automate badges for crates.io, doc.rs, lib.rs, license, crev review. Check if they exist and create badges.  
//! Create a git tag, a github release and in the same time CHANGELOG.md or RELEASES.md after publishing on crates.io.
//! There is no binary upload for library releases.
//!
//! Separate commit for docs. To not interfere with actual code commits.
//! The docs html does not have enought newlines, so the diff is very bad.
//! The file_hashes json files don't have enough newlines, so the diff is very bad.
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
mod public_api_mod;
mod utils_mod;
// endregion

// only the Public API is exported (functions, structs, methods, enums, traits)
pub use public_api_mod::*;
