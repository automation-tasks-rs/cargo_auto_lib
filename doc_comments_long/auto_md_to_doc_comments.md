[//]: # (auto_md_to_doc_comments segment start A)

This function finds rs files with markers and include segments from md files as doc comments.  

From this doc comments `cargo doc` will generated the documentation and auto-completion.  
We don't want to manually copy this segments. We want them to be automatically in sync.  
We will just run this function before every `cargo doc` with an automation task.  
The `auto_md_to_doc_comments` function must be executed in the project root folder where is the Cargo.toml file.  
First it searches all the rs files in src, tests and examples folders.  
If they contain the markers, than finds the md file and the named segment and include it as doc comments into the rs file.  
The markers are always in pairs: start and end. So exactly the content in between is changed.  
The markers are always comments, so they don't change the code.  
It works only for files with LF line delimiter. No CR and no CRLF.  

## markers

In the rs file write these markers:  

```code
comment region: auto_md_to_doc_comments include README.md A ///
comment endregion: auto_md_to_doc_comments include README.md A ///
```

In your rust code, change the word `comment` with double slash `//`.  
In the md file put markers to mark the segment:  

```markdown
[comment]: # (auto_md_to_doc_comments segment start A)  
[comment]: # (auto_md_to_doc_comments segment end A)  
```

In your markdown, change the word `[comment]` with double slash `[//]`.

The marker must be exclusively in one line. No other text in the same line.  
auto_md_to_doc_comments will delete the old lines between the markers.  
It will find the md file and read the content between the markers.  
Before each line it will add the doc comment symbol as is defined in the marker.  
Finally it will include the new lines as doc comments in the rs file.

[//]: # (auto_md_to_doc_comments segment end A)
