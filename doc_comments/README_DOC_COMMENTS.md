# doc_comments

Doc-comments in Rust are used to generate the documentation.  
The short doc-comments are not a problem.  
But sometimes they can get pretty long. This is cumbersome to write in the Rust code.  
It is easier to write a proper markdown file and include it in the Rust code.  
`#[doc=include_str!()]` works fine for doc generation, but...  

Rust analyzer is my greatest friend. I cannot code without it.  
Sadly, the "relatively new" `#[doc=include_str!()]` is not recognized by the Rust analyzer.  
Fortunately, I can just go back to my old style how to include text segments from `md` files to `rs` files:  

In the `rs` file write these markers:  

```code
//comment region: auto_md_to_doc_comments include README.md A ///
//comment endregion: auto_md_to_doc_comments include README.md A ///
```

In your rust code, change the word `comment` with a double slash `//`.  
Use the /// for functions and //! for modules doc-comments.  
In the `md` file put markers to mark the segment:  

```markdown
[//comment]: # (auto_md_to_doc_comments segment start A)
[//comment]: # (auto_md_to_doc_comments segment end A)
```

In this instructions I changed `[//]` to `[//comment]` to not process these markers.

The name of the file could be the name of the module or function.  
Also in my style of explicitly defining the public API, I have 2 functions or structs with identical doc-comments.  
The md_to_doc_comments comes in handy to use the same text segment multiple times.

## Disable markdown linter rule

The first line of doc-comments is special, but is not a markdown header.  
It must not have the header symbol #.  
In `.markdownlint.json` write to disable the Rule MD041 - First line in a file should be a top-level heading:

```json
{
    "MD041": false
}
```
