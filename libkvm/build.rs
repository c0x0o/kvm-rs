use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=platform.c");
    println!("cargo:rerun-if-changed=platform.h");

    cc::Build::new()
        .file("src/platform.c")
        .compile("platform");
    let bindings = bindgen::Builder::default()
        .header("src/platform.h")
        .allowlist_type("libkvm_.*")
        .allowlist_var("KVM_EXIT_.*")
        .allowlist_function("libkvm_.+")
        .blocklist_type(".*uint.*")
        .generate()
        .expect("Unable to generate platform bindings");
    let output_path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("No output dir specified"))
            .join("src/platform.rs");
    bindings
        .write_to_file(output_path)
        .expect("Write binds to file failed");
}
