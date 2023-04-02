//! CSV serialization and deserialization for `no_std` crates.
//!
//! `serde-csv-core` builds upon [`csv_core`](https://crates.io/crates/csv-core) crate.
//! 
//! # Serialization
//! ```
//! #[derive(serde::Serialize)]
//! struct Data {
//!     number: f32,
//!     text: &'static str
//! }
//! 
//! let mut writer = csv_core::Writer::new();
//! let data = Data { number: 7.3214, text: "hello" };
//! let mut buf = [0; 32];
//! 
//! let nwritten = serde_csv_core::to_slice(&mut writer, &data, &mut buf).unwrap();
//!
//! assert_eq!(&buf[..nwritten], b"7.3214,hello\n");
//! ```
//! 
//! # Deserialization
//! Not yet implemented.
#![no_std]

pub mod ser;

#[doc(inline)]
pub use ser::to_slice;
#[cfg(feature = "heapless")]
#[doc(inline)]
pub use ser::to_vec;
