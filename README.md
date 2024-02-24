# cargo_auto_lib

[//]: # (auto_cargo_toml_to_md start)

**Library crate for common tasks when building rust projects. Intended for use with cargo-auto - automation tasks written in Rust language.**  
***version: 1.3.17 date: 2024-02-23 author: [Bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/cargo_auto_lib)***  

[//]: # (auto_cargo_toml_to_md end)

 ![logo](https://raw.githubusercontent.com/bestia-dev/cargo-auto/main/images/logo/logo_cargo_auto.svg)
 ![status](https://img.shields.io/badge/maintained-green)
 ![status](https://img.shields.io/badge/ready_for_use-green)

 [![crates.io](https://img.shields.io/crates/v/cargo_auto_lib.svg)](https://crates.io/crates/cargo_auto_lib)
 [![Documentation](https://docs.rs/cargo_auto_lib/badge.svg)](https://docs.rs/cargo_auto_lib/)
 [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/cargo_auto_lib.svg)](https://web.crev.dev/rust-reviews/crate/cargo_auto_lib/)
 [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/cargo_auto_lib/)

 [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/cargo_auto_lib/blob/master/LICENSE)
 [![Rust](https://github.com/bestia-dev/cargo_auto_lib/workflows/rust_fmt_auto_build_test/badge.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
 ![cargo_auto_lib](https://bestia.dev/webpage_hit_counter/get_svg_image/276360626.svg)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-2069-green.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-621-blue.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-342-purple.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-29-yellow.svg)](https://github.com/bestia-dev/cargo_auto_lib/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-79-orange.svg)](https://github.com/bestia-dev/cargo_auto_lib/)

[//]: # (auto_lines_of_code end)

Hashtags: #rustlang #buildtool #developmenttool  
My projects on GitHub are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Try it

First, install the tool for task automation in Rust projects:

```bash
cargo install cargo-auto
```

Generate a new Rust CLI project:

```bash
cargo auto new_cli hello_world
```

Open the `hello_world` project in VSCode:

```bash
code hello_world
```

Open the generated directory `automation_tasks_rs` as an independent rust project in VSCode.

```bash
code hello_world/automation_tasks_rs
```

Now we can analyze the automation code. There is already the dependency inside `Cargo.toml` for our library:  

```toml
cargo_auto_lib="1.1.2"
```

Review the code in `automation_tasks_rs/main.rs`. The `cl::` namespace is the alias for `cargo_auto_lib`.  
Example:  

```rust ignore
/// cargo build --release
fn task_release() {
//    let cargo_toml = CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");

    cl::run_shell_command("cargo fmt");
    cl::run_shell_command("cargo build --release");
    println!(
        r#"{YELLOW}
    After `cargo auto release`, run examples and tests
    if ok, then,{RESET}{GREEN}
cargo auto doc{RESET}{YELLOW}
{RESET}"#
    );
    print_examples_cmd();
}
```

You can see this function will increment the version in Cargo.toml.  
Then it will copy some data from Cargo.toml to README.md (title, description, version, author,...).  
It will count the lines of code and create badges in README.md.  
Then comes the traditional Rust part: cargo fmt and cargo build --release.  
Finally, it will show on the screen the instructions on how to continue developing.  

Run (in your main rust project):

```bash
cargo auto release
```

Now open the README.md and you will see the data that this automation task copied from other places. Therefore you change this data only in one place, the automation task copies them wherever needed.

## Caveats

This crate will attempt to edit `Cargo.toml`. Unfortunately, there's no great robust way right now to edit TOML file preserving formatting and comments and such, so right now I use just regex to do this.  
If you find that the heuristics don't work for you though please let me know and I'll try to check in a fix!

## Development details

Read the development details in a separate md file:  
[DEVELOPMENT.md](https://github.com/bestia-dev/cargo_auto_lib/blob/main/DEVELOPMENT.md)

## Releases changelog

Read the release changelog in a separate md file:  
[RELEASES.md](https://github.com/bestia-dev/cargo_auto_lib/blob/main/RELEASES.md)

## TODO

Nothing big for now.

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
