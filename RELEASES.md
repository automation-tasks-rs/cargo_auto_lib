# Releases changelog of cargo_auto_lib

All notable changes to this project will be documented in this file.  
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).  
The library releases will be published on crates.io.  
The cargo-auto automation task will use the content of the section `## Unreleased` to create
the GitHub release consistently with this file.  
The ongoing changes that are not released, are visible in the git commits and GitHub pull requests.  
The TODO section is part of the [README.md](https://github.com/automation-tasks-rs/cargo_auto_lib).  

## Unreleased

- update automation

- thiserror instead of panic and unwrap

## Version 3.0.14 (2025-03-29)

- update automation

## Version 3.0.12 (2025-03-17)

- compatible with win-git-bash

- compatible win-git-bash

- test ok

- test ok

## Version 3.0.9 (2025-03-13)

- reexported inquire, tilde_expand

## Version 3.0.8 (2025-03-13)

- update automation
- clippy, win action
- docs
- reqwest 0.12.12 last version that compiles in git-bash

## Version 3.0.4 (2025-03-10)

- cargo_auto_config.json

## Version 3.0.3 (2025-03-10)

- crates io api

## Version 3.0.2 (2025-03-09)

- chrono:clock and ring cannot compile in windows

## Version 3.0.1 (2025-03-04)

- updated dependencies
- breaking changes with secrecy SecretString and SecretBox
- 3.0.1

## Version 2.4.8 (2024-04-23)

- warning for plain text

## Version 2.4.3 (2024-04-23)

- sanitize
- secret_token
- secret_arg

## Version 2.2.1 (2024-04-17)

- crates.io removed

## Version 2.1.4 (2024-04-17)

- run_shell_command_static

## Version 2.1.2 (2024-04-15)

- extracted github

## Version 2.0.8 (2024-03-30)

- pages workflow

## Version 2.0.6 (2024-03-29)

- github_owner

## Version 2.0.5 (2024-03-29)

- github_api_create_a_github_pages_site

## Version 2.0.4 (2024-03-07)

- docs

## Version 2.0.2 (2024-03-07)

### Breaking changes in 2.0.0

- Remove the old code for workspace
- It will be replaced by separate automation tasks for every member
- no workspace

## Version 1.4.8 (2024-03-07)

- doc comments
- coded
- cl::description_and_topics_to_github()

## Version 1.4.4 (2024-03-02)

- back to auto_md_to_doc_comments
- doc-comments
- better docs and hover

## Version 1.3.63 (2024-02-29)

- colors

## Version 1.3.62 (2024-02-29)

- crates io secret_token

## Version 1.3.59 (2024-02-29)

- YELLOW RED

## Version 1.3.40 (2024-02-29)

- expand

## Version 1.3.36 (2024-02-28)

- encrypt decrypt ~

## Version 1.3.35 (2024-02-28)

- encrypt decrypt

## Version 1.3.33 (2024-02-25)

- description_and_topics_to_github

## Version 1.3.32 (2024-02-25)

- description_and_topics_to_github

## Version 1.3.21 (2024-02-24)

- doc include for doc comments long

## Version 1.3.17 (2024-02-23)

- auto_playground_mod

## Version 1.3.6 (2024-02-23)

- add_message_to_unreleased

## Version 1.3.4 (2024-02-22)

- bug RELEASES_MD

## Version 1.3.3 (2024-02-22)

- ssh_add_resolve

## Version 1.2.13 (2024-02-20)

- auto_copy_files_to_strings_mod

## Version 1.2.4 (2024-02-20)

- GitHub release from RELEASES.md

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
