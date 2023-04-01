#![no_std]

pub mod serializer;

pub use serializer::Serializer;

use serde::Serialize;

#[cfg(feature = "heapless")]
pub fn to_vec<T, const N: usize>(v: &T) -> Result<heapless::Vec<u8, N>>
where
    T: Serialize + ?Sized,
{
    use heapless::Vec;

    let mut buf: Vec<u8, N> = Vec::new();
    unsafe { buf.resize_default(N).unwrap_unchecked() };

    let len = to_slice(v, &mut buf)?;
    buf.truncate(len);
    Ok(buf)
}

pub fn to_slice<T>(v: &T, output: &mut [u8]) -> Result<usize>
where
    T: Serialize + ?Sized,
{
    let writer = csv_core::Writer::new();
    let mut nwritten = 0;

    let mut serializer = Serializer::new(writer, output);
    v.serialize(&mut serializer)?;
    nwritten += serializer.bytes_written();

    let mut writer = serializer.into_writer();

    let (result, n) = writer.terminator(&mut output[nwritten..]);
    if result == csv_core::WriteResult::OutputFull {
        return Err(Error::Overflow);
    }
    nwritten += n;

    let (result, n) = writer.finish(&mut output[nwritten..]);
    if result == csv_core::WriteResult::OutputFull {
        return Err(Error::Overflow);
    }
    nwritten += n;

    Ok(nwritten)
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
