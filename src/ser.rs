//! Serialize a Rust data structure into CSV data.

#[cfg(feature = "heapless")]
use heapless::Vec;
use serde::{ser, Serialize};

/// Wrapper for [`csv_core::Writer`] that provides methods for serialization.
pub struct Writer {
    inner: csv_core::Writer,
}

impl Writer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::from_inner(csv_core::Writer::new())
    }

    pub fn from_inner(inner: csv_core::Writer) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> csv_core::Writer {
        self.inner
    }

    /// Serializes the given value as a CSV byte slice.
    ///
    /// Inserts record terminator after the serialized value.
    /// Flattens compound types (e.g. nested structs, tuples, vectors).
    /// On success, it returns the number of bytes written.
    ///
    /// # Example
    /// ```
    /// use heapless::String;
    ///
    /// #[derive(serde::Serialize)]
    /// struct Record {
    ///     pub country: String<32>,
    ///     pub city: String<32>,
    ///     pub population: u32,
    /// }
    ///
    /// let record = Record {
    ///     country: "Poland".into(),
    ///     city: "Cracow".into(),
    ///     population: 766_683,
    /// };
    ///
    /// let mut writer = serde_csv_core::Writer::new();
    /// let mut buf = [0; 32];
    /// let len = writer.serialize_to_slice(&record, &mut buf)?;
    ///
    /// assert_eq!(&buf[..len], b"Poland,Cracow,766683\n");
    /// # Ok::<(), serde_csv_core::ser::Error>(())
    /// ```
    pub fn serialize_to_slice<T>(&mut self, value: &T, output: &mut [u8]) -> Result<usize>
    where
        T: Serialize + ?Sized,
    {
        let mut nwritten = 0;

        let mut serializer = Serializer::new(&mut self.inner, output);
        value.serialize(&mut serializer)?;
        nwritten += serializer.bytes_written();

        let (result, n) = self.inner.terminator(&mut output[nwritten..]);
        if result == csv_core::WriteResult::OutputFull {
            return Err(Error::Overflow);
        }
        nwritten += n;

        Ok(nwritten)
    }

    /// Serializes the given value as a CSV byte vector.
    ///
    /// Inserts record terminator after the serialized value.
    /// Flattens compound types (e.g. nested structs, tuples, vectors).
    ///
    /// # Example
    /// ```
    /// use heapless::{String, Vec};
    ///
    /// #[derive(serde::Serialize)]
    /// struct Record {
    ///     pub country: String<32>,
    ///     pub city: String<32>,
    ///     pub population: u32
    /// }
    ///
    /// let record = Record {
    ///     country: "Poland".into(),
    ///     city: "Cracow".into(),
    ///     population: 766_683
    /// };
    ///
    /// let mut writer = serde_csv_core::Writer::new();
    /// let buf: Vec<u8, 32> = writer.serialize_to_vec(&record)?;
    ///
    /// assert_eq!(&buf, b"Poland,Cracow,766683\n");
    /// # Ok::<(), serde_csv_core::ser::Error>(())
    /// ```
    /// Serializes the given value as a CSV byte vector.
    ///
    /// Inserts record terminator after the serialized value.
    /// Flattens compound types (e.g. nested structs, tuples, vectors).
    #[cfg(feature = "heapless")]
    pub fn serialize_to_vec<T, const N: usize>(&mut self, value: &T) -> Result<Vec<u8, N>>
    where
        T: Serialize + ?Sized,
    {
        let mut buf: Vec<u8, N> = Vec::new();
        // SAFETY:
        // always safe since buf has capacity N
        unsafe { buf.resize_default(N).unwrap_unchecked() };

        let len = self.serialize_to_slice(value, &mut buf)?;
        buf.truncate(len);
        Ok(buf)
    }
}

/// This type represents all possible errors that can occur when serializing CSV data.
#[derive(Debug)]
pub enum Error {
    /// Buffer overflow.
    Overflow,
}

/// Alias for a `core::result::Result` with the error type `serde_csv_core::ser::Error`.
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

/// A structure for serializing Rust values into CSV.
pub struct Serializer<'a> {
    writer: &'a mut csv_core::Writer,
    output: &'a mut [u8],
    nwritten: usize,
}

impl<'a> Serializer<'a> {
    /// Creates a new CSV serializer.
    pub fn new(writer: &'a mut csv_core::Writer, output: &'a mut [u8]) -> Self {
        Self {
            writer,
            output,
            nwritten: 0,
        }
    }

    /// Returns the number of bytes written.
    pub fn bytes_written(&self) -> usize {
        self.nwritten
    }

    fn field(&mut self, input: impl AsRef<[u8]>) -> Result<()> {
        let (r, _, n) = self
            .writer
            .field(input.as_ref(), &mut self.output[self.nwritten..]);
        self.nwritten += n;
        if r == csv_core::WriteResult::OutputFull {
            return Err(Error::Overflow);
        }
        Ok(())
    }

    fn delimiter(&mut self) -> Result<()> {
        let (r, n) = self.writer.delimiter(&mut self.output[self.nwritten..]);
        self.nwritten += n;
        if r == csv_core::WriteResult::OutputFull {
            return Err(Error::Overflow);
        }
        Ok(())
    }
}

impl<'a, 'b> ser::Serializer for &'a mut Serializer<'b> {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Compound<'a, 'b>;

    type SerializeTuple = Compound<'a, 'b>;

    type SerializeTupleStruct = Compound<'a, 'b>;

    type SerializeTupleVariant = Unreachable;

    type SerializeMap = Unreachable;

    type SerializeStruct = Compound<'a, 'b>;

    type SerializeStructVariant = Unreachable;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        if v {
            self.field(b"true")
        } else {
            self.field(b"false")
        }
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        let mut buffer = itoa::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        let mut buffer = ryu::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        let mut buffer = ryu::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.field(v.encode_utf8(&mut [0; 4]))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.field(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.field(v)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        self.field([])
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        self.field([])
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        self.field(name)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.field(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(Compound::new(self))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(Compound::new(self))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(Compound::new(self))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!("serialize_tuple_variant is not supported");
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!("serialize_map is not supported");
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(Compound::new(self))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!("serialize_struct_variant is not supported");
    }

    fn collect_str<T: ?Sized>(self, _value: &T) -> Result<Self::Ok>
    where
        T: core::fmt::Display,
    {
        unimplemented!("collect_str is not supported");
    }
}

#[doc(hidden)]
pub struct Compound<'a, 'b> {
    serializer: &'a mut Serializer<'b>,
    nfields: usize,
}

impl<'a, 'b> Compound<'a, 'b> {
    fn new(serializer: &'a mut Serializer<'b>) -> Self {
        Self {
            serializer,
            nfields: 0,
        }
    }

    fn element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        if self.nfields > 0 {
            self.serializer.delimiter()?;
        }
        self.nfields += 1;
        value.serialize(&mut *self.serializer)
    }
}

impl ser::SerializeSeq for Compound<'_, '_> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        self.element(value)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl ser::SerializeTuple for Compound<'_, '_> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        self.element(value)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl ser::SerializeTupleStruct for Compound<'_, '_> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        self.element(value)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl ser::SerializeStruct for Compound<'_, '_> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        self.element(value)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

#[doc(hidden)]
pub struct Unreachable;

impl ser::SerializeTupleVariant for Unreachable {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok> {
        unreachable!()
    }
}

impl ser::SerializeMap for Unreachable {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok> {
        unreachable!()
    }
}

impl ser::SerializeStructVariant for Unreachable {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ser::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok> {
        unreachable!()
    }
}
