[//]: # (auto_md_to_doc_comments segment start A)

Includes the link to playground with the rust code in a parameter.

Search in markdown files for markersand include a link to Rust playground.

```markdown
[comment]: # (auto_playground start)

Run this code in the [Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20m%0A%7D):

'''Rust
fn main(){
    println!("Hello World!");
}
'''

[comment]: # (auto_playground end)
```

In your markdown, change the word `[comment]` with double slash `[//]`. And the single quotes with ticks.

Between the start marker and the first triple backtick is the link in "()" parentheses.  
Encode the code with url_encoding.

Process code for playground for Rust code segments in all md files

[//]: # (auto_md_to_doc_comments segment end A)
