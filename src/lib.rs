#![no_std]

pub mod serializer;

pub use serializer::Serializer;

use serde::Serialize;

pub fn to_byte_slice(v: impl Serialize, output: &mut [u8]) -> Result<usize> {
    let writer = csv_core::Writer::new();
    let mut s = Serializer::new(writer, output);
    v.serialize(&mut s)?;
    Ok(s.bytes_written())
}

#[derive(Debug)]
pub enum Error {
    Overflow,
}

pub type Result<T> = core::result::Result<T, Error>;

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Buffer overflow")
    }
}

impl serde::ser::StdError for Error {}

impl serde::ser::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        unimplemented!("custom is not supported")
    }
}
