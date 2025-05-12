//! Deserialize CSV data into a Rust data structure.

use core::borrow::Borrow;
use lexical_parse_float::FromLexical;
use serde::{de::DeserializeSeed, Deserialize};

/// Wrapper for [`csv_core::Reader`] that provides methods for deserialization using [`serde`].
///
/// `N` is a capacity of an internal buffer that's used to temporarily store unescaped fields.
#[derive(Debug)]
pub struct Reader<const N: usize> {
    inner: csv_core::Reader,
    field_buffer: [u8; N],
}

impl<const N: usize> Default for Reader<N> {
    fn default() -> Self {
        Self::from_builder(csv_core::ReaderBuilder::new())
    }
}

impl<const N: usize> Reader<N> {
    /// Constructs a new reader.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructs a new reader from [`csv_core::ReaderBuilder`].
    ///
    /// # Example
    /// ```
    /// use serde_csv_core::csv_core;
    ///
    /// let reader = serde_csv_core::Reader::<16>::from_builder(
    ///     csv_core::ReaderBuilder::new()
    ///         .delimiter(b'-')
    /// );
    /// ```
    pub fn from_builder(builder: impl Borrow<csv_core::ReaderBuilder>) -> Self {
        Self {
            inner: builder.borrow().build(),
            field_buffer: [0; N],
        }
    }

    /// Deserializes a given CSV byte slice into a value of type `T`.
    ///
    /// The second element of the resulting tuple is a number of bytes read.
    ///
    /// # Example
    /// ```
    /// use heapless::String;
    /// use serde::Deserialize;
    ///
    /// #[derive(Debug, PartialEq, Eq, Deserialize)]
    /// struct Record {
    ///     pub country: String<32>,
    ///     pub city: String<32>,
    ///     pub population: u32,
    /// }
    ///
    /// let csv = b"Poland,Cracow,766683\n";
    ///
    /// let mut reader = serde_csv_core::Reader::<32>::new();
    /// let (record, nread)  = reader.deserialize::<Record>(&csv[..])?;
    ///
    /// assert_eq!(record, Record {
    ///     country: "Poland".into(),
    ///     city: "Cracow".into(),
    ///     population: 766_683,
    /// });
    /// assert_eq!(nread, 21);
    /// # Ok::<(), serde_csv_core::de::Error>(())
    /// ```
    pub fn deserialize<'de, T>(&mut self, input: &[u8]) -> Result<(T, usize)>
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
    /// Buffer overflow.
    Overflow,
    /// Expected an empty field.
    ExpectedEmpty,
    /// Invalid boolean value. Expected either `true` or `false`.
    InvalidBool,
    /// Invalid integer.
    InvalidInt,
    /// Invalid floating-point number.
    InvalidFloat,
    /// Invalid UTF-8 encoded character.
    InvalidUtf8Char,
    /// Invalid UTF-8 encoded string.
    InvalidUtf8String,
    /// Error with a custom message had to be discarded.
    Custom,
}

macro_rules! impl_format {
    ($self:ident, $write:ident, $f:ident) => {
        match $self {
            Self::Overflow => $write!($f, "Buffer overflow."),
            Self::ExpectedEmpty => $write!($f, "Expected an empty field."),
            Self::InvalidBool => {
                $write!(
                    $f,
                    "Invalid boolean value. Expected either `true` or `false`."
                )
            }
            Self::InvalidInt => $write!($f, "Invalid integer."),
            Self::InvalidFloat => $write!($f, "Invalid floating-point number."),
            Self::InvalidUtf8Char => $write!($f, "Invalid UTF-8 encoded character."),
            Self::InvalidUtf8String => $write!($f, "Invalid UTF-8 encoded string."),
            Self::Custom => $write!($f, "CSV does not match deserializer's expected format."),
        }
    };
}

/// Alias for a `core::result::Result` with the error type `serde_csv_core::de::Error`.
pub type Result<T> = core::result::Result<T, Error>;

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        impl_format!(self, write, f)
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
    fn format(&self, f: defmt::Formatter) {
        use defmt::write;
        impl_format!(self, write, f)
    }
}

#[derive(Debug)]
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
        atoi::atoi(self.read_bytes()?).ok_or(Error::InvalidInt)
    }

    fn read_float<T: FromLexical>(&mut self) -> Result<T> {
        T::from_lexical(self.read_bytes()?).map_err(|_| Error::InvalidFloat)
    }

    fn read_str(&mut self) -> Result<&str> {
        core::str::from_utf8(self.read_bytes()?).map_err(|_| Error::InvalidUtf8String)
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
            _ => Err(Error::InvalidBool),
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
        let c = iter.next().ok_or(Error::InvalidUtf8Char)?;
        if iter.next().is_some() {
            return Err(Error::InvalidUtf8Char);
        }
        visitor.visit_char(c)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_str(self.read_str()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_str(self.read_str()?)
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
        unimplemented!("`Deserializer::deserialize_byte_buf` is not supported");
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
            return Err(Error::ExpectedEmpty);
        }
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        unimplemented!("`Deserializer::deserialize_newtype_struct` is not supported");
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
        unimplemented!("`VariantAccess::newtype_variant_seed` is not supported");
    }

    fn tuple_variant<V: serde::de::Visitor<'de>>(
        self,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value> {
        unimplemented!("`VariantAccess::tuple_variant` is not supported");
    }

    fn struct_variant<V: serde::de::Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value> {
        unimplemented!("`VariantAccess::struct_variant` is not supported");
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
