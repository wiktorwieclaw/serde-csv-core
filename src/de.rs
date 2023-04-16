//! Deserialize CSV data into a Rust data structure.

use lexical_parse_float::FromLexical;
use serde::{de::DeserializeSeed, Deserialize};

/// Wrapper for [`csv_core::Reader`] that provides methods for deserialization.
///
/// `N` is a capacity of an internal buffer that's used to temporarily store unescaped fields.
pub struct Reader<const N: usize> {
    inner: csv_core::Reader,
    field_buffer: [u8; N],
}

impl<const N: usize> Reader<N> {
    #[allow(clippy::new_without_default)]
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

    pub fn deserialize_from_slice<'de, T>(&mut self, input: &[u8]) -> Result<(T, usize)>
    where
        T: Deserialize<'de>,
    {
        let mut deserializer = Deserializer::new(self, input);
        let value = T::deserialize(&mut deserializer)?;
        Ok((value, deserializer.bytes_read()))
    }
}

/// This type represents all possible errors that can occur when deserializing CSV data.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Overflow,
    Parse,
    Unexpected,
    Custom,
}

/// Alias for a `core::result::Result` with the error type `serde_csv_core::de::Error`.
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
        Self::Custom
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Error {
    fn format(&self, fmt: defmt::Formatter) {
        use defmt::write;
        match self {
            Self::Overflow => write!(fmt, "Buffer overflow"),
            Self::Parse => write!(fmt, "Failed to parse field"),
            Self::Unexpected => write!(fmt, "Unexpected error"),
            Self::Custom => write!(fmt, "Custom error"),
        }
    }
}

struct Deserializer<'a, const N: usize> {
    reader: &'a mut Reader<N>,
    input: &'a [u8],
    nread: usize,
    record_end: bool,
    peeked: Option<usize>,
}

impl<'a, const N: usize> Deserializer<'a, N> {
    pub fn new(reader: &'a mut Reader<N>, input: &'a [u8]) -> Self {
        Self {
            reader,
            input,
            nread: 0,
            record_end: false,
            peeked: None,
        }
    }

    pub fn bytes_read(&self) -> usize {
        self.nread
    }

    fn read_bytes_impl(&mut self) -> Result<usize> {
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

    fn peek_bytes(&mut self) -> Result<&[u8]> {
        let len = match self.peeked {
            Some(len) => len,
            None => {
                let len = self.read_bytes_impl()?;
                self.peeked = Some(len);
                len
            }
        };
        Ok(&self.reader.field_buffer[..len])
    }

    fn read_bytes(&mut self) -> Result<&[u8]> {
        let len = match self.peeked.take() {
            Some(len) => len,
            None => self.read_bytes_impl()?,
        };
        Ok(&self.reader.field_buffer[..len])
    }

    fn read_int<T: atoi::FromRadix10SignedChecked>(&mut self) -> Result<T> {
        atoi::atoi(self.read_bytes()?).ok_or(Error::Parse)
    }

    fn read_float<T: FromLexical>(&mut self) -> Result<T> {
        T::from_lexical(self.read_bytes()?).map_err(|_| Error::Parse)
    }

    fn read_str(&mut self) -> Result<&str> {
        core::str::from_utf8(self.read_bytes()?).map_err(|_| Error::Parse)
    }
}

impl<'de, 'a, 'b, const N: usize> serde::de::Deserializer<'de> for &'a mut Deserializer<'b, N> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!("`Deserializer::deserialize_any` is not supported");
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.read_bytes()? {
            b"true" => visitor.visit_bool(true),
            b"false" => visitor.visit_bool(false),
            _ => Err(Error::Parse),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.read_int().and_then(|v| visitor.visit_i8(v))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.read_int().and_then(|v| visitor.visit_i16(v))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.read_int().and_then(|v| visitor.visit_i32(v))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.read_int().and_then(|v| visitor.visit_i64(v))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.read_int().and_then(|v| visitor.visit_u8(v))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.read_int().and_then(|v| visitor.visit_u16(v))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.read_int().and_then(|v| visitor.visit_u32(v))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.read_int().and_then(|v| visitor.visit_u64(v))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.read_float().and_then(|v| visitor.visit_f32(v))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.read_float().and_then(|v| visitor.visit_f64(v))
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let str = self.read_str()?;
        let mut iter = str.chars();
        let c = iter.next().ok_or(Error::Parse)?;
        if iter.next().is_some() {
            return Err(Error::Parse);
        }
        visitor.visit_char(c)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_str(self.read_str()?)
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!("`Deserializer::deserialize_any` is not supported");
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.read_bytes().and_then(|v| visitor.visit_bytes(v))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!("`Deserializer::deserialize_buf` is not supported");
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let bytes = self.peek_bytes()?;
        if bytes.is_empty() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let bytes = self.read_bytes()?;
        if !bytes.is_empty() {
            return Err(Error::Parse);
        }
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_enum(self)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!("`Deserializer::deserialize_identifier` is not supported");
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        let _ = self.read_bytes()?;
        visitor.visit_unit()
    }
}

impl<'de, 'a, 'b, const N: usize> serde::de::VariantAccess<'de> for &'a mut Deserializer<'b, N> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<U: DeserializeSeed<'de>>(self, _seed: U) -> Result<U::Value> {
        Err(Error::Unexpected)
    }

    fn tuple_variant<V: serde::de::Visitor<'de>>(
        self,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value> {
        Err(Error::Unexpected)
    }

    fn struct_variant<V: serde::de::Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value> {
        Err(Error::Unexpected)
    }
}

impl<'de, 'a, 'b, const N: usize> serde::de::EnumAccess<'de> for &'a mut Deserializer<'b, N> {
    type Error = Error;

    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        use serde::de::IntoDeserializer;
        let variant_name = self.read_bytes()?;
        seed.deserialize(variant_name.into_deserializer())
            .map(|v| (v, self))
    }
}

impl<'de, 'a, 'b, const N: usize> serde::de::SeqAccess<'de> for &'a mut Deserializer<'b, N> {
    type Error = Error;

    fn next_element_seed<V>(&mut self, seed: V) -> Result<Option<V::Value>>
    where
        V: DeserializeSeed<'de>,
    {
        if self.record_end {
            Ok(None)
        } else {
            seed.deserialize(&mut **self).map(Some)
        }
    }
}
