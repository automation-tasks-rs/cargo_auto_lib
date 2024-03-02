[//]: # (auto_md_to_doc_comments segment start A)

Traverse dir and its sub-dir, but avoid excluded dirs.

The find_file and the exclude dir strings must start with /.

## Example

```Rust

let files = cargo_auto_lib::traverse_dir_with_exclude_dir(
    Path::new("/home/project/src"),
    "/*.rs",
    // avoid big folders and other folders with *.crev
    &vec![
        "/.git".to_string(),
        "/target".to_string(),
        "/docs".to_string()
    ]
).unwrap();
for rs_file_name in files.iter() {
    println!("{}", &rs_file_name);
}
```

[//]: # (auto_md_to_doc_comments segment end A)
