//! Raw bindings to the [`lib3mf`][upstream] C++ library.
//!
//! # Examples
//!
//! The C API can be used directly.
//!
//! ```rust
//! unsafe {
//!     let mut model: Lib3MF_Model = std::ptr::null_mut();
//!
//!     // Create an empty model.
//!     assert_eq!(
//!         lib3mf_createmodel(&mut model),
//!         LIB3MF_SUCCESS as crate::Lib3MFResult,
//!     );
//!
//!     // create the reader
//!     let mut reader: Lib3MF_Reader = std::ptr::null_mut();
//!     assert_eq!(
//!         lib3mf_model_queryreader(model, "3mf\0".as_ptr().cast(), &mut reader),
//!         LIB3MF_SUCCESS as crate::Lib3MFResult,
//!     );
//!
//!     // Make sure we can read in one of lib3mf's test files
//!     let pyramid_file = Path::new(env!("CARGO_MANIFEST_DIR"))
//!         .join("vendor/lib3mf/Tests/TestFiles/Reader/Pyramid.3mf");
//!     let pyramid = std::fs::read(&pyramid_file).unwrap();
//!
//!     assert_eq!(
//!         lib3mf_reader_readfrombuffer(reader, pyramid.len() as _, pyramid.as_ptr()),
//!         LIB3MF_SUCCESS as crate::Lib3MFResult,
//!     );
//!
//!     // Don't forget to free everything afterwards
//!     assert_eq!(
//!         lib3mf_release(reader),
//!         LIB3MF_SUCCESS as crate::Lib3MFResult,
//!     );
//!     assert_eq!(lib3mf_release(model), LIB3MF_SUCCESS as crate::Lib3MFResult);
//! }
//! ```
//!
//! [upstream]: https://github.com/3MFConsortium/lib3mf/

#![allow(rustdoc::broken_intra_doc_links)]

mod bindings;

pub use crate::bindings::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const C_BINDINGS_HEADER: &str = env!("C_BINDINGS_HEADER");

    #[test]
    fn generated_bindings_are_up_to_date() {
        // Note: The header file ends in *.h, but clang will only treat it as
        // C++ is it ends in *.hpp.
        let header = Path::new(C_BINDINGS_HEADER).with_extension("hpp");
        std::fs::copy(C_BINDINGS_HEADER, &header).unwrap();

        let bindings = bindgen::builder()
            .header(header.display().to_string())
            .respect_cxx_access_specs(true)
            .allowlist_function("lib3mf.*")
            .allowlist_var("(?i).*3mf.*")
            .raw_line("#![allow(nonstandard_style)]")
            .generate()
            .unwrap()
            .to_string();
        let dest = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("bindings.rs");

        if let Ok(original) = std::fs::read_to_string(&dest) {
            if original == bindings {
                // All up to date
                return;
            }
        }

        if let Some(parent) = dest.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        std::fs::write(&dest, bindings.as_bytes()).unwrap();

        panic!(
            "\"{}\" was out-of-date. Re-run the test and commit the changes",
            dest.display()
        );
    }

    #[test]
    fn smoke_test() {
        unsafe {
            let mut model: Lib3MF_Model = std::ptr::null_mut();

            // Create an empty model.
            assert_eq!(
                lib3mf_createmodel(&mut model),
                LIB3MF_SUCCESS as crate::Lib3MFResult,
            );

            // create the reader
            let mut reader: Lib3MF_Reader = std::ptr::null_mut();
            assert_eq!(
                lib3mf_model_queryreader(model, "3mf\0".as_ptr().cast(), &mut reader),
                LIB3MF_SUCCESS as crate::Lib3MFResult,
            );

            let pyramid_file = Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("vendor/lib3mf/Tests/TestFiles/Reader/Pyramid.3mf");
            let pyramid = std::fs::read(&pyramid_file).unwrap();

            assert_eq!(
                lib3mf_reader_readfrombuffer(reader, pyramid.len() as _, pyramid.as_ptr()),
                LIB3MF_SUCCESS as crate::Lib3MFResult,
            );

            // Free everything afterwards
            assert_eq!(
                lib3mf_release(reader),
                LIB3MF_SUCCESS as crate::Lib3MFResult,
            );
            assert_eq!(lib3mf_release(model), LIB3MF_SUCCESS as crate::Lib3MFResult);
        }
    }
}
