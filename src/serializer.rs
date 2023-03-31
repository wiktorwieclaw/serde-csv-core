use crate::Error;
use serde::ser;

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

    pub fn bytes_written(&self) -> usize {
        self.nwritten
    }

    fn field(&mut self, input: impl AsRef<[u8]>) -> Result<(), Error> {
        let (r, _, n) = self
            .writer
            .field(input.as_ref(), &mut self.output[self.nwritten..]);
        self.nwritten += n;
        if r == csv_core::WriteResult::OutputFull {
            return Err(Error::Overflow);
        }
        Ok(())
    }

    fn delimiter(&mut self) -> Result<(), Error> {
        let (r, n) = self.writer.delimiter(&mut self.output[self.nwritten..]);
        self.nwritten += n;
        if r == csv_core::WriteResult::OutputFull {
            return Err(Error::Overflow);
        }
        Ok(())
    }

    fn terminator(&mut self) -> Result<(), Error> {
        let (r, n) = self.writer.terminator(&mut self.output[self.nwritten..]);
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

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        if v {
            self.field(b"true")
        } else {
            self.field(b"false")
        }
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        let mut buffer = itoa::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        let mut buffer = itoa::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        let mut buffer = itoa::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let mut buffer = itoa::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        let mut buffer = itoa::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        let mut buffer = itoa::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        let mut buffer = itoa::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        let mut buffer = itoa::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        let mut buffer = ryu::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        let mut buffer = ryu::Buffer::new();
        self.field(buffer.format(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.field(v.encode_utf8(&mut [0; 4]))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.field(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.field(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.field([])
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.field([])
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.field(name)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.field(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
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
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Compound::new(self))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(Compound::new(self))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(Compound::new(self))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!("serialize_tuple_variant is not supported");
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!("serialize_map is not supported");
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Compound::new(self))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!("serialize_struct_variant is not supported");
    }

    fn collect_str<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
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

    fn element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
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

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        self.element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.terminator()
    }
}

impl ser::SerializeTuple for Compound<'_, '_> {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        self.element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.terminator()
    }
}

impl ser::SerializeTupleStruct for Compound<'_, '_> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        self.element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.terminator()
    }
}

impl ser::SerializeStruct for Compound<'_, '_> {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        self.element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.serializer.terminator()
    }
}

pub struct Unreachable;

impl ser::SerializeTupleVariant for Unreachable {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl ser::SerializeMap for Unreachable {
    type Ok = ();

    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}

impl ser::SerializeStructVariant for Unreachable {
    type Ok = ();

    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unreachable!()
    }
}
