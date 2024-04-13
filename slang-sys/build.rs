use std::fs;
use std::path::Path;
use bindgen::Formatter;

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .clang_arg("-I./vendor/slang")
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        .header("vendor/slang/slang.h")
        .formatter(Formatter::Rustfmt)
        .generate()
        .expect("Failed to generate bindings");

    fs::create_dir_all("gen").unwrap();
    fs::write(Path::new("gen/bindings.rs"), bindings.to_string()).expect("Failed to write bindings to file");
}

fn main() {
    generate_bindings();
}