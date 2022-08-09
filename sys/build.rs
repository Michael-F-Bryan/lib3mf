use std::path::PathBuf;

fn main() {
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let vendor_dir = crate_dir.join("vendor").join("lib3mf");

    let dest = cmake::build(&vendor_dir);

    println!("cargo:rustc-link-search={}", dest.join("lib").display());
    println!("cargo:rustc-link-lib=3mf");

    println!("cargo:include={}", dest.join("include").display());
    println!("cargo:lib={}", dest.join("lib").display());

    let header_file = dest
        .join("include")
        .join("Bindings")
        .join("C")
        .join("lib3mf.h");
    println!(
        "cargo:rustc-env=C_BINDINGS_HEADER={}",
        header_file.display()
    );
}
