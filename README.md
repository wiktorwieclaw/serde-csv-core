# serde-csv-core
[![Crates.io](https://img.shields.io/crates/v/serde-csv-core.svg)](https://crates.io/crates/serde-csv-core)
[![Released API docs](https://docs.rs/serde-csv-core/badge.svg)](https://docs.rs/serde-csv-core)
[![Continuous integration](https://github.com/wiktorwieclaw/serde-csv-core/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/wiktorwieclaw/serde-csv-core/actions/workflows/ci.yaml)

CSV serialization and deserialization for `no_std` crates.

`serde-csv-core` builds upon [`csv-core`](https://crates.io/crates/csv-core) crate.

## Serialization
`to_slice` serializes one record at a time.
```rust
use heapless::String;

#[derive(serde::Serialize)]
struct Record {
    pub country: String<32>,
    pub city: String<32>,
    pub population: u32
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

let mut writer = csv_core::Writer::new();
let mut buf = [0; 128];
let mut len = 0;

for record in records {
    len += serde_csv_core::to_slice(&mut writer, &record, &mut buf[len..])?;
}

assert_eq!(&buf[..len], b"Poland,Cracow,766683\nJapan,Tokyo,13515271\n");
```

## Deserialization
Not yet implemented.

## License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.