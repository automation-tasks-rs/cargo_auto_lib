[comment]: # (auto_md_to_doc_comments segment start A)

# cargo_auto_lib

[comment]: # (auto_cargo_toml_to_md start)

**Library crate for common tasks when building rust projects. Intended for use with cargo-auto.**  
***[repo](https://github.com/LucianoBestia/cargo_auto_lib); version: 0.1.13  date: 2021-08-13 authors: Luciano Bestia***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)

[comment]: # (auto_lines_of_code end)

## Try it

In your rust project root directory (where the Cargo.toml is)  
first install [cargo-auto](https://crates.io/crates/cargo-auto) and generate a new helper project:

```bash
cargo install cargo-auto
cargo auto new
```

In a new editor open the generated directory `automation_tasks_rs` as an independent rust project.  
Add to Cargo.toml dependencies:  

```toml
cargo_auto_lib="0.1.13"
```

In the beginning of the function `task_build()` add:  

```rust
cargo_auto_lib::auto_cargo_toml_to_md();
```

Add markers to the beginning of README.md (don't copy the numbers 1 and 2):  

```md
1 [comment]: # (auto_cargo_toml_to_md start)
2 [comment]: # (auto_cargo_toml_to_md end)
```

Run:

```bash
cargo auto build
```

With a little luck, it included the data of Cargo.toml into the `README.md` inside the markers.  






## function auto_cargo_toml_to_md()

To avoid out of sync data like version, authors and description in the README.md files, `auto_cargo_toml_to_md` includes this data from Cargo.toml.  
Run it on every build with [cargo auto](https://crates.io/crates/cargo-auto).  

It works only for single projects and not for workspaces yet.  
It works also with other md files in the project, not only README.md.  
In the md file write these markers in markdown comments (invisible),  
don't copy the numbers 1 and 2:  

```markdown
1 [comment]: # (auto_cargo_toml_to_md start)
2 [comment]: # (auto_cargo_toml_to_md end)
```

`auto_cargo_toml_to_md` deletes the old lines between the markers and includes the cargo.toml data:  
description, repository, version, &utc_now(), authors  

Run the example:  

```bash
cargo run --example example_01_auto_cargo_toml_to_md
```





## cargo crev reviews and advisory

We leave in times of danger with `supply chain attacks`.  
It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
You can also read reviews quickly on the web. Example for the crate `num-traits`:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  

## open-source free and free as a beer

My open-source projects are free and free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer or two donating on my [paypal](https://www.paypal.com/paypalme/LucianoBestia). You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) !
# cargo_auto_lib
