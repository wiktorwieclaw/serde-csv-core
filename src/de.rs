use serde::{de::DeserializeSeed, Deserialize};

pub struct Reader<const N: usize> {
    inner: csv_core::Reader,
    field_buffer: [u8; N],
}

impl<const N: usize> Reader<N> {
    pub fn new() -> Self {
        Self::from_inner(csv_core::Reader::new())
    }

    pub fn from_inner(inner: csv_core::Reader) -> Self {
        Self {
            inner,
            field_buffer: [0; N],
        }
    }

    pub fn into_inner(self) -> csv_core::Reader {
        self.inner
    }

    pub fn deserialize_from_slice<'de, T>(&mut self, input: &[u8]) -> Result<T>
    where
        T: Deserialize<'de>,
    {
        let mut deserializer = Deserializer {
            reader: self,
            input,
            nread: 0,
            record_end: false,
        };
        T::deserialize(&mut deserializer)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Overflow,
    Parse,
}

pub type Result<T> = core::result::Result<T, Error>;

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Deserialization error")
    }
}

impl serde::de::StdError for Error {}

impl serde::de::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        unimplemented!("custom is not supported");
    }
}

struct Deserializer<'a, const N: usize> {
    reader: &'a mut Reader<N>,
    input: &'a [u8],
    nread: usize,
    record_end: bool,
}

impl<'a, const N: usize> Deserializer<'a, N> {
    fn read_field(&mut self) -> Result<usize> {
        let (result, r, w) = self
            .reader
            .inner
            .read_field(&self.input[self.nread..], &mut self.reader.field_buffer);
        self.nread += r;
        match result {
            csv_core::ReadFieldResult::InputEmpty => {}
            csv_core::ReadFieldResult::OutputFull => return Err(Error::Overflow),
            csv_core::ReadFieldResult::Field { record_end } => self.record_end = record_end,
            csv_core::ReadFieldResult::End => {}
        }
        Ok(w)
    }
}

impl<'de, 'a, 'b, const N: usize> serde::de::Deserializer<'de> for &'a mut Deserializer<'b, N> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!("deserialize_any is not supported");
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let nwritten = self.read_field()?;
        match &self.reader.field_buffer[..nwritten] {
            b"true" => visitor.visit_bool(true),
            b"false" => visitor.visit_bool(false),
            _ => Err(Error::Parse),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let nwritten = self.read_field()?;
        let value = atoi::atoi(&self.reader.field_buffer[..nwritten]).ok_or(Error::Parse)?;
        visitor.visit_i8(value)
    }

    fn deserialize_i16<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let nwritten = self.read_field()?;
        if nwritten > 0 {
            return Err(Error::Parse);
        }
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> core::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }
}

impl<'de, 'a, 'b, const N: usize> serde::de::SeqAccess<'de> for &'a mut Deserializer<'b, N> {
    type Error = Error;

    fn next_element_seed<U: DeserializeSeed<'de>>(&mut self, seed: U) -> Result<Option<U::Value>> {
        if self.record_end {
            Ok(None)
        } else {
            seed.deserialize(&mut **self).map(Some)
        }
    }
}
