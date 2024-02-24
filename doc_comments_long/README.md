# doc_comments_long

Doc-comments in Rust are used to generate the documentation.  
The short doc-comments are not a problem.  
But sometimes they can get pretty long. This is cumbersome to write in the Rust code.  
It is easier to write a proper markdown file and include it in the Rust code.  
The name of the file could be the name of the file concatenated with the name of the function.  
In the Rust code then we write just this macro:

```rust
#![doc=include_str!("../doc_comments_long/file_function.md")]
```
