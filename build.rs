use std::env;
use std::path::Path;

fn write_header() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Cannot get manifest dir");
    let header_path = Path::new(&manifest_dir).join("header.h");

    let variants = if cfg!(feature = "v2") {
        vec!["B"]
    } else {
        vec!["A", "B"]
    };
    let variants_str = variants.join(",\n    ");

    std::fs::write(
        header_path,
        format!(r"
#pragma once
enum Variant {{
    {variants_str}
}};
")).expect("Cannot write header file");
}

fn main() {
    write_header();

    cxx_build::bridge("src/main.rs").flag("-std=c++17").compile("cxx-not-feature");
}
