//! CSV serialization and deserialization for `no_std` crates.
//!
//! `serde-csv-core` builds upon [`csv-core`](https://crates.io/crates/csv-core) crate.
//! It doesn't require any memory allocations, which means that it's well-suited for embedded environments.
//!
//! # Serialization
//! [`Writer`] serializes one record at a time.
//! ```
//! use heapless::String;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
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
//! let mut csv = [0; 128];
//! let mut nwritten = 0;
//!
//! for record in &records {
//!     nwritten += writer.serialize_to_slice(&record, &mut csv[nwritten..])?;
//! }
//!
//! assert_eq!(&csv[..nwritten], b"Poland,Cracow,766683\nJapan,Tokyo,13515271\n");
//!
//! # Ok::<(), serde_csv_core::ser::Error>(())
//! ```
//!
//! # Deserialization
//! [`Reader<N>`] deserializes one record at a time.
//! `N` is a capacity of an internal buffer that's used to temporarily store unescaped fields.
//! ```
//! use heapless::{String, Vec};
//! use serde::Deserialize;
//!
//! #[derive(Debug, PartialEq, Eq, Deserialize)]
//! struct Record {
//!     pub country: String<32>,
//!     pub city: String<32>,
//!     pub population: u32,
//! }
//!
//! let csv = b"Poland,Cracow,766683\nJapan,Tokyo,13515271\n";
//!
//! let mut reader = serde_csv_core::Reader::<32>::new();
//! let mut records: Vec<Record, 2> = Vec::new();
//! let mut nread = 0;
//!
//! while nread < csv.len() {
//!     let (record, n)  = reader.deserialize_from_slice::<Record>(&csv[nread..])?;
//!     records.push(record);
//!     nread += n;
//! }
//!
//! assert_eq!(records, &[
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
//! ]);
//! # Ok::<(), serde_csv_core::de::Error>(())
//! ```
//!
//! # Configuration
//! Both [`Writer`] and [`Reader`] are wrappers for [`csv_core::Writer`]
//! and [`csv_core::Reader`], respectively. You can use [`csv_core::WriterBuilder`]
//! and [`csv_core::ReaderBuilder`] in combination with `from_inner` constructors
//! to tweak things like field delimiters etc.
//! ```
//! use serde_csv_core::csv_core;
//!
//! let inner = csv_core::WriterBuilder::new()
//!     .delimiter(b'-')
//!     .build();
//! let writer = serde_csv_core::Writer::from_inner(inner);
//! ```
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
