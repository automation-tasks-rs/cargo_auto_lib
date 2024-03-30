# Development details

## CRUSTDE - Containerized Rust Development Environment

I recommend using the CRUSTDE - Containerized Rust Development Environment to write Rust projects.  
Follow the instructions here  
<https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod>.  

It is an isolated development environment that will not mess with you system.
It will work on Linux (tested on Debian) and inside WSL (Windows Subsystem for Linux).

You just need to install the Docker newer alternative: [Podman](https://podman.io/).  
Then you download the prepared container image from DockerHub (3GB).  
And then a little juggling with SSH keys.  
All this is simplified by running a few bash scripts.  
Just follow the easy instructions.  

The container image contains cargo, rustc, wasm-pack, basic-http-server, cargo-auto and other utils that a Rust project needs.  

## Workflow with automation_tasks_rs

Automation tasks that are already coded in the sub-project `automation_tasks_rs`. This is a basic workflow:

```bash
cargo auto build
cargo auto release
cargo auto doc
cargo auto test
cargo auto commit_and push
cargo auto publish_to_crates_io
cargo auto github_new_release
```

Every task finishes with instructions how to proceed.  
The [cargo-auto](https://github.com/automation-tasks-rs/cargo-auto) and [dev_bestia_cargo_completion](https://github.com/automation-tasks-rs/dev_bestia_cargo_completion) are already installed inside the CRUSTDE container.

You can open the automation sub-project in VSCode and then code your own tasks in Rust.

```bash
code automation_tasks_rs
```

## Based on simple functions

All the functions of `cargo_auto_lib` have extensive help/docs to describe how they work.  
This is nice when you use a code editor with IntelliSense like VSCode.  
Inside the `automation_tasks_rs` you can write your own code. No limits there. It is just Rust.  

## Caveats writing Cargo.toml

This crate will attempt to edit `Cargo.toml`. Unfortunately, there's no great robust way right now to edit TOML file preserving formatting and comments and such, so right now I use just regex to do this.  
If you find that the heuristics don't work for you though please let me know and I'll try to check in a fix!
