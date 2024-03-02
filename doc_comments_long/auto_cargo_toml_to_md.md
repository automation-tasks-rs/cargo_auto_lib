[//]: # (auto_md_to_doc_comments segment start A)

This function includes data from Cargo.toml to markdown files.  

This is nice for avoiding out of sync data.  
Run it on every build with `automation_tasks_rs` and [cargo auto](https://crates.io/crates/cargo-auto).  
  
In the md file write these markers in invisible markdown comments.

```markdown
[comment]: # (auto_cargo_toml_to_md start)

[comment]: # (auto_cargo_toml_to_md end)

```

In your markdown, change the word `[comment]` with double slash `[//]`.

`auto_cargo_toml_to_md` deletes the old lines between the markers and includes the Cargo.toml data:  
description, repository, version, &utc_now(), authors and creates badges for keywords and categories.

The words topics, keywords and tags all mean the same concept.
In cargo.toml we have keywords.
In README.md I want to have badges, but I don't know the color yet.
In GitHub they are topics.

Some keywords have defined colors, others are orange like Rust.
This can be expanded in the future.
Yellow: work-in-progress
Green: maintained, ready-for-use
Red: obsolete, archived

Run the example:  

```bash
cargo run --example example_01_auto_cargo_toml_to_md
```

[//]: # (auto_md_to_doc_comments segment end A)
