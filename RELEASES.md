# Releases of cargo_auto_lib

All notable changes to this project will be documented in this file.  
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).  
The library releases will be published on crates.io.  
A git tag and github release will be created with the same content as this file.  
I will use the cargo-auto automation tasks to use the content of this file to create a tag and github release consistently.  

## [Unreleased]

The ongoing changes that are not released are visible in the git commits and github pull requests.  
The TODO section is part of the README.md.

## Version 1.1.2 (2024-02-05)

Release published to [crates.io](https://crates.io/crates/cargo_auto_lib/1.1.2).

- In `automation_tasks_rs/Cargo.toml` replace for example `cargo_auto_lib = "1.0.96"` to `cargo_auto_lib = "1.1.2"`  

### Breaking changes in v1.1.2

- Researching a way to make the Public API consistent for future releases I made a breaking change.  
I incremented the minor version number and reset the patch number to signal it is a breaking change.  
These functions are rarely used, so it will not be a big deal.

I removed the exported module `utils_mod` from the Public API.  Its functions are now in the root module of the Public API.

How to refactor your code to resolve the issue:

- find `utils_mod::` and replace with empty string.

## Version 1.0.96 (2024-02-04)

Release published to [crates.io](https://crates.io/crates/cargo_auto_lib/1.0.96).

- In `automation_tasks_rs/Cargo.toml` replace for example `cargo_auto_lib = "0.8.60"` to `cargo_auto_lib = "1.0.96"`
  
### Changes in v1.0.96

- Refactor many `unwrap()` to error handling with `thiserror`.  
The problem is that it could introduce unwanted braking changes.  
I need to research how to make my Public API more manageable.
