<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

Copy all files from the folder into a module as strings (static &str).

The Rust code to modify has the markers:

```Rust ignore
//comment region: files copied into strings by automation tasks

//comment endregion: files copied into strings by automation tasks

```

In this instructions I changed `[//]` to `[//comment]` to not process these markers.

First we create the complete text, then we check if the old text needs to be replaced.

Binary files need a special treatment:

```Rust ignore
ext_for_binary_files=vec![".ico",".jpg",".png",".woff2"];
```

Exclude big folders:

```Rust ignore
exclude_big_folders = vec!["/.git","/target","/docs"];
```

[//]: # (auto_md_to_doc_comments segment end A)
