//! Serialize Rust types to VDF text

use keyvalues_parser::Vdf;
use serde::{ser, Serialize};

use std::io::Write;

use crate::{
    error::{Error, Result},
    tokens::{NaiveToken, NaiveTokenStream},
};

/// The struct for serializing Rust values into VDF text
///
/// This typically doesn't need to be invoked directly when [`to_writer()`] and
/// [`to_writer_with_key()`] can be used instead
#[derive(Default)]
pub struct Serializer {
    tokens: NaiveTokenStream,
}

impl Serializer {
    /// Creates a new VDF serializer
    pub fn new() -> Self {
        Self::default()
    }
}

/// Serialize the `value` into an IO stream of VDF text
///
/// # Errors
///
/// This will return an error if the input can't be represented with valid VDF
pub fn to_writer<W, T>(writer: &mut W, value: &T) -> Result<()>
where
    W: Write,
    T: Serialize,
{
    _to_writer(writer, value, None)
}

/// Serialize the `value` into an IO stream of VDF text with a custom top level VDF key
///
/// # Errors
///
/// This will return an error if the input can't be represented with valid VDF
pub fn to_writer_with_key<W, T>(writer: &mut W, value: &T, key: &str) -> Result<()>
where
    W: Write,
    T: Serialize,
{
    _to_writer(writer, value, Some(key))
}

// Serialization process goes as follows:
// value: &T
// -> NaiveTokenStream
// -> Vdf (fails on invalid VDF structure like nested sequences)
// -> Formatted
// Which is a bit of a long-winded process just to serialize some text, but it comes with
// validation (NaiveTokenStream -> Vdf) and reuses portions from the parser (Vdf -> Formatted)
fn _to_writer<W, T>(writer: &mut W, value: &T, maybe_key: Option<&str>) -> Result<()>
where
    W: Write,
    T: Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;

    if let Some(key) = maybe_key {
        match serializer.tokens.first() {
            // Replace the old key
            Some(NaiveToken::Str(_old_key)) => {
                serializer.tokens[0] = NaiveToken::Str(key.to_owned());
            }
            // Push on the key
            Some(_) => serializer.tokens.insert(0, NaiveToken::Str(key.to_owned())),
            None => {}
        }
    }

    let vdf = Vdf::try_from(&serializer.tokens)?;
    write!(writer, "{}", vdf)?;

    Ok(())
}

/// Attempts to serialize some input to VDF text
///
/// # Errors
///
/// This will return an error if the input can't be represented with valid VDF
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut buffer = Vec::new();
    to_writer(&mut buffer, value)?;
    let s = String::from_utf8(buffer).expect("Input was all valid UTF-8");

    Ok(s)
}

/// Attempts to serialize some input to VDF text with a custom top level VDF key
///
/// # Errors
///
/// This will return an error if the input can't be represented with valid VDF
pub fn to_string_with_key<T>(value: &T, key: &str) -> Result<String>
where
    T: Serialize,
{
    let mut buffer = Vec::new();
    to_writer_with_key(&mut buffer, value, key)?;
    let s = String::from_utf8(buffer).expect("Input was all valid UTF-8");

    Ok(s)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.serialize_str(if v { "1" } else { "0" })
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_i128(self, v: i128) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_u128(self, v: u128) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        if v.is_finite() {
            self.serialize_str(&v.to_string())
        } else {
            Err(Error::NonFiniteFloat(v))
        }
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        // TODO: include this and empty vecs and nested Option<Vec> in potential pitfalls
        // TODO: look into this more, might be the other way around if the wiki is wrong
        // Note: I believe floats in VDF are considered f32 so even when you use an f64 it will get
        // converted to an f32 when serialized
        self.serialize_f32(v as f32)
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.tokens.push(NaiveToken::str(v));
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        Err(Error::Unsupported("Bytes"))
    }

    fn serialize_none(self) -> Result<()> {
        self.tokens.push(NaiveToken::Null);
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // Just serializes the contained value
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Err(Error::Unsupported("Unit Type"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error::Unsupported("Unit Struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        // Just pass the variant name for unit variant enums
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // Just a wrapper over the contained value
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Unsupported("Enum Newtype Variant"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.tokens.push(NaiveToken::SeqBegin);
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::Unsupported("Enum Tuple Variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.tokens.push(NaiveToken::ObjBegin);
        Ok(self)
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        // The top level key is the name of the struct
        if self.tokens.is_empty() {
            self.serialize_str(name)?;
        }

        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::Unsupported("Enum Struct Variant"))
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.tokens.push(NaiveToken::SeqEnd);
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.tokens.push(NaiveToken::SeqEnd);
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.tokens.push(NaiveToken::SeqEnd);
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Unsupported("Enum Variant"))
    }

    fn end(self) -> Result<()> {
        Err(Error::Unsupported("Enum Variant"))
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.tokens.push(NaiveToken::ObjEnd);
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.tokens.push(NaiveToken::ObjEnd);
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Unsupported("Enum Variant"))
    }

    fn end(self) -> Result<()> {
        Err(Error::Unsupported("Enum Variant"))
    }
}
