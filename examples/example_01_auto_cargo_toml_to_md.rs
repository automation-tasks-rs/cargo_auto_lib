// example_01_auto_cargo_toml_to_md

/// Includes data from Cargo.toml to README.md files
fn main() {
    println!("---start example_01_cargo_toml_to_md---");

    cargo_auto_lib::auto_cargo_toml_to_md();

    println!("---end example_01_cargo_toml_to_md---");
}
