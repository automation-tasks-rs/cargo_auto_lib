# Releases changelog of cargo_auto_lib

All notable changes to this project will be documented in this file.  
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).  
The library releases will be published on crates.io.  
The cargo-auto automation task will use the content of the section `## Unreleased` to create
the GitHub release consistently with this file.  
The ongoing changes that are not released, are visible in the git commits and github pull requests.  
The TODO section is part of the [README.md](https://github.com/bestia-dev/cargo_auto_lib).  

## Unreleased

## Version 1.3.6 (2024-02-23)

- add_message_to_unreleased

## Version 1.3.4 (2024-02-22)

- bug RELEASES_MD

## Version 1.3.3 (2024-02-22)

- ssh_add_resolve

## Version 1.2.13 (2024-02-20)

- auto_copy_files_to_strings_mod

## Version 1.2.4 (2024-02-20)

- github release from RELEASES.md

## Version 1.2.3 (2024-02-20)

- move here the functions from crate cargo_auto_github_lib
- github_api_create_new_release
- github_api_upload_asset_to_release
- automation add task github_new_release for library

### Breaking changes in v1.2.3

- rename api_call_repository_new to github_api_repository_new

## Version 1.1.35 (2024-02-20)

- functions for init repo and GitHub

## Version 1.1.32 (2024-02-18)

- update calculate hash

## Version 1.1.23 (2024-02-18)

- separate commit of docs if changed
- logo
- doc include_str
- tidy

## Version 1.1.2 (2024-02-05)

### Breaking changes in v1.1.2

- Researching a way to make the Public API consistent for future releases I made a breaking change.  
I incremented the minor version number and reset the patch number to signal it is a breaking change.  
These functions are rarely used, so it will not be a big deal.

I removed the exported module `utils_mod` from the Public API.  Its functions are now in the root module of the Public API.

How to refactor your code to resolve the issue:

- find `utils_mod::` and replace with empty string.

## Version 1.0.96 (2024-02-04)

- Refactor many `unwrap()` to error handling with `thiserror`.  
The problem is that it could introduce unwanted braking changes.  
I need to research how to make my Public API more manageable.
