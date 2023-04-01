use serde::{ser, Serialize};

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

pub struct Serializer<'a> {
    writer: csv_core::Writer,
    output: &'a mut [u8],
    nwritten: usize,
}

impl<'out> Serializer<'out> {
    pub fn new(writer: csv_core::Writer, output: &'out mut [u8]) -> Self {
        Self {
            writer,
            output,
            nwritten: 0,
        }
    }

    pub fn into_writer(self) -> csv_core::Writer {
        self.writer
    }

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

impl<'ser, 'out> ser::Serializer for &'ser mut Serializer<'out> {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Compound<'ser, 'out>;

    type SerializeTuple = Compound<'ser, 'out>;

    type SerializeTupleStruct = Compound<'ser, 'out>;

    type SerializeTupleVariant = Unreachable;

    type SerializeMap = Unreachable;

    type SerializeStruct = Compound<'ser, 'out>;

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

pub struct Compound<'ser, 'out> {
    serializer: &'ser mut Serializer<'out>,
    nfields: usize,
}

impl<'ser, 'out> Compound<'ser, 'out> {
    fn new(serializer: &'ser mut Serializer<'out>) -> Self {
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
