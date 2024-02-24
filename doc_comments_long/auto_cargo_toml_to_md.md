# auto_cargo_toml_to_md

This function includes data from Cargo.toml to markdown files$1  $2This is nice for avoiding out of sync data.

Run it on every build with `automation_tasks_rs` and [cargo auto](https://crates.io/crates/cargo-auto).  
  
In the md file write these markers in invisible markdown comments.

```markdown
[comment]: # (auto_cargo_toml_to_md start)

[comment]: # (auto_cargo_toml_to_md end)
```

In your markdown, change the word `[comment]` with double slash `[//]`.

`auto_cargo_toml_to_md` deletes the old lines between the markers and includes the Cargo.toml data:  
description, repository, version, &utc_now(), authors  

Run the example:  

```bash
cargo run --example example_01_auto_cargo_toml_to_md
```
