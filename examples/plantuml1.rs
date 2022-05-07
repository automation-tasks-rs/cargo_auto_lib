// examples/plantuml1.rs

/// $ cargo run --example plantuml1

fn main() {
    let text = "test";
    println!("{}", text);
    let hash = cargo_auto_lib::hash_for_filename(&text);
    println!("{}", hash);

    let path = std::path::Path::new("examples/plantuml");
    cargo_auto_lib::auto_plantuml_for_path(path);

}
