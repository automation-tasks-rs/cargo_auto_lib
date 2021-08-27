[comment]: # (auto_md_to_doc_comments segment start A)

# cargo_auto_lib

[comment]: # (auto_cargo_toml_to_md start)

**Library crate for common tasks when building rust projects. Intended for use with cargo-auto: automation tasks written in Rust language.**  
***[repository](https://github.com/LucianoBestia/cargo_auto_lib); version: 0.7.19  date: 2021-08-27 authors: Luciano Bestia***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1001-green.svg)](https://github.com/LucianoBestia/cargo_auto_lib/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-380-blue.svg)](https://github.com/LucianoBestia/cargo_auto_lib/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-118-purple.svg)](https://github.com/LucianoBestia/cargo_auto_lib/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/cargo_auto_lib/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-11-orange.svg)](https://github.com/LucianoBestia/cargo_auto_lib/)

[comment]: # (auto_lines_of_code end)

[![crates.io](https://img.shields.io/crates/v/cargo_auto_lib.svg)](https://crates.io/crates/cargo_auto_lib) [![Documentation](https://docs.rs/cargo_auto_lib/badge.svg)](https://docs.rs/cargo_auto_lib/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/cargo_auto_lib.svg)](https://web.crev.dev/rust-reviews/crate/cargo_auto_lib/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/cargo_auto_lib/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/cargo_auto_lib/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/cargo_auto_lib/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/cargo_auto_lib/)  

## Try it

In your rust project root directory (where the Cargo.toml is)  
first install [cargo-auto](https://crates.io/crates/cargo-auto) and generate a new helper project:

```bash
cargo install cargo-auto
cargo auto new with_lib
```

In a new editor open the generated directory `automation_tasks_rs` as an independent rust project. There is already this dependency in `Cargo.toml`:  

```toml
cargo_auto_lib="0.7.*"
```

Preview the code and observe all the `auto_*` functions from `cargo_auto_lib`.  
Example:  

```rust
fn task_release() {
    auto_semver_increment_patch();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");
    auto_build();
}
```

Go back to your main rust project.  
Add markers to the beginning of README.md (don't copy the numbers 1 and 2):  

```md
1 [comment]: # (auto_cargo_toml_to_md start)
2 [comment]: # (auto_cargo_toml_to_md end)
```

Run (in your main rust project):

```bash
cargo auto release
```

With a little luck, it included the data of Cargo.toml into the `README.md` inside the markers:  

![auto_cargo_toml_to_md](https://github.com/LucianoBestia/cargo_auto_lib/raw/main/images/auto_cargo_toml_to_md.png "auto_cargo_toml_to_md")

## Functions

All the functions have extensive hep/docs to describe how they work.  
It is nice when you use a code editor with IntelliSense like VSCode.  
Here is a list of some of them:  

- `auto_cargo_toml_to_md()` - includes data from Cargo.toml to README.md files: version, authors,...
- `auto_delete_old_js_snippets()` - deletes old js snippets when working with wasm-pack
- `auto_lines_of_code()` - inserts shield badges with lines_of_code into README.rs
- `auto_md_to_doc_comments()` - Finds rs files with markers and include segments from md files
- `auto_semver_increment_minor()` - increments semver version minor part
- `auto_semver_increment_patch()` - increments semver version patch part
- `auto_version_from_date()` - new version as date is written toCargo.toml and service_worker.js
- `package_authors_string_without_emails()` - Cargo.toml package authors as string without emails
- `package_description()` - Cargo.toml package repository
- `package_name()` - Cargo.toml package name
- `package_repository()` - Cargo.toml package repository
- `package_version()` - Cargo.toml package version
- `run_shell_command()` - run one shell command
- `run_shell_commands()` - run shell commands from a vector of strings.

## Caveats

This crate will attempt to edit Cargo.toml. Unfortunately there's no great robust way right now to edit TOML file preserving formatting and comments and such, so right now I use just regex to do this.
If you find that the heuristics don't work for you though please let me know and I'll try to check in a fix!

## learn something new every day

I needed to copy large text into doc comments.  
It means every line must get a prefix like `///`.  
In VSCode I selected the text, press  
`alt+shift+i`
Now I have `multiple cursors` on the end of every selected lines.  
I press the key
`home`
And now I have multiple cursors on the beginning of every line.  
I type (insert):
`///`  
and it's done ! Great !

## TODO

Automate badges for crates.io, doc.rs, lib.rs, license, crev review. Check if they exist and create badges.

## cargo crev reviews and advisory

We leave in times of danger with `supply chain attacks`.  
It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
You can also read reviews quickly on the web. Example for the crate `num-traits`:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  

## open-source free and free as a beer

My open-source projects are free and free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer or two donating on my [paypal](https://www.paypal.com/paypalme/LucianoBestia). You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) !

[comment]: # (auto_md_to_doc_comments segment end A)
