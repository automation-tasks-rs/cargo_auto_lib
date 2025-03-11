<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

Include the link to run code in Rust playground.

The function searches in all markdown files for markers like this:

```markdown
[//comment]: # (auto_playground start)

Run this code in the [Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=fn%20m%0A%7D):

'''Rust ignore
fn main(){
    println!("Hello World!");
}
'''

[//comment]: # (auto_playground end)
```

In this instructions I changed `[//]` to `[//comment]` and  ticks to single quotes to not process these markers.

Between the start marker and the first triple backtick there is the link in "()" parentheses. The link to Rust playground encodes the code with url_encoding (percents) and sends it as an Url parameter.

Info: docs.rs has already a functionality that shows the `Run` button on your code and can run code the playground if you put this line at the top of lib.rs:

``` Rust ignore
#![doc(html_playground_url = "https://play.rust-lang.org")]
```

But it works only on docs.rs.  
I want to run my code examples from everywhere: from GitHub README.md, GitHub pages and crates.io.  

[//]: # (auto_md_to_doc_comments segment end A)
