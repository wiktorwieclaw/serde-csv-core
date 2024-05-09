# serde-csv-core
[![Crates.io](https://img.shields.io/crates/v/serde-csv-core.svg)](https://crates.io/crates/serde-csv-core)
[![Released API docs](https://docs.rs/serde-csv-core/badge.svg)](https://docs.rs/serde-csv-core)
[![Continuous integration](https://github.com/wiktorwieclaw/serde-csv-core/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/wiktorwieclaw/serde-csv-core/actions/workflows/ci.yaml)

CSV serialization and deserialization for `no_std` crates.

`serde-csv-core` builds upon [`csv-core`](https://crates.io/crates/csv-core) crate.
It doesn't require any memory allocations, which means that it's well-suited for embedded environments.

## Serialization
`Writer` serializes one record at a time.
```rust
use heapless::String;
use serde::Serialize;

#[derive(Serialize)]
struct Record {
    pub country: String<32>,
    pub city: String<32>,
    pub population: u32,
}

let records = [
    Record {
        country: "Poland".into(),
        city: "Cracow".into(),
        population: 766_683,
    },
    Record {
        country: "Japan".into(),
        city: "Tokyo".into(),
        population: 13_515_271,
    },
];

let mut writer = serde_csv_core::Writer::new();
let mut csv = [0; 128];
let mut nwritten = 0;
for record in &records {
    nwritten += writer.serialize(&record, &mut csv[nwritten..])?;
}

assert_eq!(&csv[..nwritten], b"Poland,Cracow,766683\nJapan,Tokyo,13515271\n");
```

## Deserialization
`Reader<N>` deserializes one record at a time.
`N` is a capacity of an internal buffer that's used to temporarily store unescaped fields.
```rust
use heapless::{String, Vec};
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct Record {
    pub country: String<32>,
    pub city: String<32>,
    pub population: u32,
}

let csv = b"Poland,Cracow,766683\nJapan,Tokyo,13515271\n";

let mut reader = serde_csv_core::Reader::<32>::new();
let mut records: Vec<Record, 2> = Vec::new();
let mut nread = 0;
while nread < csv.len() {
    let (record, n) = reader.deserialize::<Record>(&csv[nread..])?;
    records.push(record);
    nread += n;
}

assert_eq!(records, &[
    Record {
        country: "Poland".into(),
        city: "Cracow".into(),
        population: 766_683,
    },
    Record {
        country: "Japan".into(),
        city: "Tokyo".into(),
        population: 13_515_271,
    },
]);
```

## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
