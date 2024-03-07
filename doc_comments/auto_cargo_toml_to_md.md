[//]: # (auto_md_to_doc_comments segment start A)

This function includes data from Cargo.toml to markdown files.  

![auto_cargo_toml_to_md.png](https://github.com/automation-tasks-rs/cargo_auto_lib/blob/main/images/auto_cargo_toml_to_md.png?raw=true)

This is nice for avoiding out of sync data.  
Run it on every build with `automation_tasks_rs` and [cargo auto](https://crates.io/crates/cargo-auto).  
  
In the md file write these markers in invisible markdown comments.

```markdown
[//comment]: # (auto_cargo_toml_to_md start)

[//comment]: # (auto_cargo_toml_to_md end)

```

In this instructions I changed `[//]` to `[//comment]` to not process these markers.

`auto_cargo_toml_to_md` deletes the old lines between the markers and includes the Cargo.toml data:  
description, repository, version, utc_now, authors and creates badges for keywords and categories.

The words topics, keywords, hashtags and tags all mean the same concept.  
In cargo.toml we have keywords.  
In README.md I want to have badges with different color. And hashtags for SEO.  
In GitHub they are topics.

Some keywords have defined colors, others are orange like Rust.  
This can be expanded in the future.  

- Yellow: work-in-progress
- Green: maintained, ready-for-use
- Red: obsolete, archived

[//]: # (auto_md_to_doc_comments segment end A)
