use std::path::PathBuf;

fn main() {
    if pkg_config::probe_library("3mf").is_ok() || vcpkg::find_package("3mf").is_ok() {
        // It was found on the system somewhere. Prefer that version.
        return;
    }

    // fall back to compiling from source.
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let lib3mf_dir = crate_dir.join("vendor").join("lib3mf");
    let dest = cmake::build(&lib3mf_dir);

    println!("cargo:rustc-link-search={}", dest.join("lib").display());
    println!("cargo:rustc-link-lib=3mf");
}
