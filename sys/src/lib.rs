//! Raw bindings to the [`lib3mf`][upstream] C++ library.
//!
//! # Examples
//!
//! The C API can be used directly.
//!
//! ```rust
//! use lib3mf_sys::*;
//! use std::path::Path;
//!
//! unsafe {
//!     let mut model: Lib3MF_Model = std::ptr::null_mut();
//!
//!     // Create an empty model.
//!     assert_eq!(
//!         lib3mf_createmodel(&mut model),
//!         LIB3MF_SUCCESS as Lib3MFResult,
//!     );
//!
//!     // create the reader
//!     let mut reader: Lib3MF_Reader = std::ptr::null_mut();
//!     assert_eq!(
//!         lib3mf_model_queryreader(model, "3mf\0".as_ptr().cast(), &mut reader),
//!         LIB3MF_SUCCESS as Lib3MFResult,
//!     );
//!
//!     // Make sure we can read in one of lib3mf's test files
//!     let pyramid_file = Path::new(env!("CARGO_MANIFEST_DIR"))
//!         .join("vendor/lib3mf/Tests/TestFiles/Reader/Pyramid.3mf");
//!     let pyramid = std::fs::read(&pyramid_file).unwrap();
//!
//!     assert_eq!(
//!         lib3mf_reader_readfrombuffer(reader, pyramid.len() as _, pyramid.as_ptr()),
//!         LIB3MF_SUCCESS as Lib3MFResult,
//!     );
//!
//!     // Don't forget to free everything afterwards
//!     assert_eq!(
//!         lib3mf_release(reader),
//!         LIB3MF_SUCCESS as Lib3MFResult,
//!     );
//!     assert_eq!(lib3mf_release(model), LIB3MF_SUCCESS as Lib3MFResult);
//! }
//! ```
//!
//! [upstream]: https://github.com/3MFConsortium/lib3mf/

#![allow(rustdoc::broken_intra_doc_links)]

cfg_if::cfg_if! {
    if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
        #[path = "bindings.x86_64.linux.rs"]
        mod bindings;
    } else {
        // Fall back to the linux bindings
        #[path = "bindings.x86_64.linux.rs"]
        mod bindings;
    }
}

pub use crate::bindings::*;
