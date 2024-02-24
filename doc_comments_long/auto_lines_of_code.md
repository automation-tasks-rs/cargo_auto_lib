# auto_lines_of_code

This functino inserts shield badges with lines_of_code into README.rs.  
The parameter Link will be used for shield badge. If empty_string, the git remote repository will be used$1  $2Lines of code are not a "perfect" measurement of anything.\
Anybody can write a very big number of lines of useless code and comments.\
But for 95% of the cases they are good enough.\
Most of the developers use some "standard" coding practices and that is quantifiable and comparable.  

The `src_code_lines` is the most important count.\
That is actual code written for that project without  doc comments, comments, unit tests, integration tests and examples.\
Sometimes is great to see a big number here. It means there was a lot of work invested. But other times we want to see a small number. It
means the developer understands the problem very well and don't try to solve anything outside that scope.  
The `src_doc_comment_lines` counts doc comments. They will eventually become docs. The count of lines shows how many documentation is written.  
The `src_comment_lines` counts code comments. Code comments are important to understand the code. The count of lines shows how understandable is the code.  
The `tests_lines` counts lines in tests and shows how good is the code tested. Here are the unit tests and integration test combined.  
The `examples_lines` counts lines in examples and shows how good is explained how to use the code.  


## Folder and file structure

The folder structure of a single Rust project is simple.\
The project starts in the folder that contains `Cargo.toml`.\
The `src/` folder contains all the rust `*.rs` files.\
The `tests/` folder contains integration tests.\
The `examples/` folder contains examples.\
Some rs files can be excluded from the count adding this line near the start of the file: // exclude from auto_lines_of_code
Inside a rs file the doc comment line start with `///` or `//!`.\
The normal comments start with `//` or `/!`.\
I will ignore the block comments. They are usually NOT used for comments, but to temporarily disable a piece of code. So I count this as code and not comments.  
The `src/*.rs` file can contain unit tests that start with `#[cfg(test)]`. I assume that these are always at the end of the file.  
There should not be any normal code after `#[cfg(test)]`, only tests.  
All other files: `md`, `toml`, `html`, `js`, ... are not counted.  

### Workspace

Workspaces have member projects, that are written in `Cargo.toml`.\
The program counts lines of every project and sums them together.  

## Include into README.md

If the README.md file contains these markers (don't copy the numbers 1 and 2):  

```md
[comment]: # (auto_lines_of_code start)

[comment]: # (auto_lines_of_code end)
```

In your markdown, change the word `[comment]` with double slash `[//]`.  

The function will include the shield badges code between them.  
It will erase the previous content.  
Use git diff to see the change.  
