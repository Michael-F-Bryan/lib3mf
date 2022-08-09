//! Raw bindings to the [`lib3mf`][upstream] C++ library.
//!
//! [upstream]: https://github.com/3MFConsortium/lib3mf/

#![allow(rustdoc::broken_intra_doc_links)]

cfg_if::cfg_if! {
    if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
        #[path = "bindings.x86_64.linux.rs"]
        mod bindings;
    } else if #[cfg(all(target_os = "macos", target_arch = "x86_64"))] {
        #[path = "bindings.x86_64.macos.rs"]
        mod bindings;
    } else {
        // Fall back to the linux bindings
        #[path = "bindings.x86_64.linux.rs"]
        mod bindings;
    }
}

pub use crate::bindings::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn smoke_test() {
        let filename = std::env::args()
            .nth(1)
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                // Fall back to using one of lib3mf's test files
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("vendor/lib3mf/Tests/TestFiles/Reader/Pyramid.3mf")
            });

        unsafe {
            let mut model: Lib3MF_Model = std::ptr::null_mut();

            // Create an empty model.
            assert_eq!(
                lib3mf_createmodel(&mut model),
                LIB3MF_SUCCESS as Lib3MFResult,
            );

            // create the reader
            let mut reader: Lib3MF_Reader = std::ptr::null_mut();
            assert_eq!(
                lib3mf_model_queryreader(model, "3mf\0".as_ptr().cast(), &mut reader),
                LIB3MF_SUCCESS as Lib3MFResult,
            );

            let buffer = std::fs::read(&filename).unwrap();
            assert_eq!(
                lib3mf_reader_readfrombuffer(reader, buffer.len() as _, buffer.as_ptr()),
                LIB3MF_SUCCESS as Lib3MFResult,
            );

            println!(
                "Successfully read \"{}\" ({} bytes)",
                filename.display(),
                buffer.len()
            );

            // Don't forget to free everything afterwards
            assert_eq!(lib3mf_release(reader), LIB3MF_SUCCESS as Lib3MFResult,);
            assert_eq!(lib3mf_release(model), LIB3MF_SUCCESS as Lib3MFResult);
        }
    }
}
