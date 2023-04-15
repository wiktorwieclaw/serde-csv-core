//! CSV serialization and deserialization for `no_std` crates.
//!
//! `serde-csv-core` builds upon [`csv-core`](https://crates.io/crates/csv-core) crate.
//!
//! # Serialization
//! [`Writer::serialize_to_slice`] serializes one record at a time.
//! ```
//! use heapless::String;
//!
//! #[derive(serde::Serialize)]
//! struct Record {
//!     pub country: String<32>,
//!     pub city: String<32>,
//!     pub population: u32,
//! }
//!
//! let records = [
//!     Record {
//!         country: "Poland".into(),
//!         city: "Cracow".into(),
//!         population: 766_683,
//!     },
//!     Record {
//!         country: "Japan".into(),
//!         city: "Tokyo".into(),
//!         population: 13_515_271,
//!     },
//! ];
//!
//! let mut writer = serde_csv_core::Writer::new();
//! let mut buf = [0; 128];
//! let mut len = 0;
//!
//! for record in records {
//!     len += writer.serialize_to_slice(&record, &mut buf[len..])?;
//! }
//!
//! assert_eq!(&buf[..len], b"Poland,Cracow,766683\nJapan,Tokyo,13515271\n");
//! # Ok::<(), serde_csv_core::ser::Error>(())
//! ```
//!
//! # Deserialization
//! Not yet implemented.
#![no_std]

pub mod de;
pub mod ser;

#[doc(inline)]
pub use de::Reader;
#[doc(inline)]
pub use ser::Writer;

pub use csv_core;
#[cfg(feature = "heapless")]
pub use heapless;
