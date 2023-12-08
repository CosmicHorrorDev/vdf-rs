//! Deserialize VDF text to Rust types

mod map;
mod seq;

use keyvalues_parser::{Key, Vdf};
use serde::{
    de::{self, DeserializeOwned, IntoDeserializer, Visitor},
    Deserialize,
};

use std::{
    borrow::Cow,
    io::Read,
    iter::Peekable,
    ops::{Deref, DerefMut},
    vec::IntoIter,
};

use crate::{
    de::{map::ObjEater, seq::SeqBuilder},
    error::{Error, Result},
    tokens::{Token, TokenStream},
};

pub fn from_reader<R: Read, T: DeserializeOwned>(rdr: R) -> Result<T> {
    from_reader_with_key(rdr).map(|(t, _)| t)
}

pub fn from_reader_with_key<R: Read, T: DeserializeOwned>(mut rdr: R) -> Result<(T, String)> {
    let mut buffer = String::new();
    rdr.read_to_string(&mut buffer)?;

    from_str_with_key(&buffer).map(|(t, key)| (t, key.into_owned()))
}

/// Attempts to deserialize a string of VDF text to some type T
pub fn from_str<'a, T: Deserialize<'a>>(s: &'a str) -> Result<T> {
    from_str_with_key(s).map(|(t, _)| t)
}

/// The same as [`from_str()`], but also returns the top level VDF key
pub fn from_str_with_key<'a, T: Deserialize<'a>>(s: &'a str) -> Result<(T, Key<'a>)> {
    let vdf = Vdf::parse(s)?;
    from_vdf_with_key(vdf)
}

pub fn from_vdf<'a, T: Deserialize<'a>>(vdf: Vdf<'a>) -> Result<T> {
    from_vdf_with_key(vdf).map(|(t, _)| t)
}

pub fn from_vdf_with_key<'a, T: Deserialize<'a>>(vdf: Vdf<'a>) -> Result<(T, Key<'a>)> {
    let (mut deserializer, key) = Deserializer::new_with_key(vdf)?;
    let t = T::deserialize(&mut deserializer)?;

    if deserializer.is_empty() {
        Ok((t, key))
    } else {
        Err(Error::TrailingTokens)
    }
}

/// The struct that handles deserializing VDF into Rust structs
///
/// This typically doesn't need to be invoked directly when [`from_str()`] and
/// [`from_str_with_key()`] can be used instead
#[derive(Debug)]
pub struct Deserializer<'de> {
    tokens: Peekable<IntoIter<Token<'de>>>,
}

impl<'de> Deserializer<'de> {
    /// Attempts to create a new VDF deserializer along with returning the top level VDF key
    pub fn new_with_key(vdf: Vdf<'de>) -> Result<(Self, Key<'de>)> {
        let token_stream = TokenStream::from(vdf);

        let key = if let Some(Token::Key(key)) = token_stream.first() {
            key.clone()
        } else {
            unreachable!("Tokenstream must start with key");
        };

        let tokens = token_stream.0.into_iter().peekable();
        Ok((Self { tokens }, key.clone()))
    }

    /// Returns if the internal tokenstream is empty
    pub fn is_empty(&mut self) -> bool {
        self.peek().is_none()
    }

    /// Returns if the next token is a value type (str, object, or sequence)
    pub fn peek_is_value(&mut self) -> bool {
        matches!(
            self.peek(),
            Some(Token::ObjBegin | Token::SeqBegin | Token::Str(_))
        )
    }

    /// Returns the next key or str if available
    pub fn next_key_or_str(&mut self) -> Option<Cow<'de, str>> {
        match self.peek() {
            Some(Token::Key(_) | Token::Str(_)) => match self.next() {
                Some(Token::Key(s) | Token::Str(s)) => Some(s),
                _ => unreachable!("Token was peeked"),
            },
            _ => None,
        }
    }

    /// Returns the next key or str or returns an appropriate error
    pub fn next_key_or_str_else_eof(&mut self) -> Result<Cow<'de, str>> {
        self.next_key_or_str()
            .ok_or(Error::EofWhileParsingKeyOrValue)
    }

    /// Returns the next finite float or returns an appropriate error
    pub fn next_finite_float_else_eof(&mut self) -> Result<f32> {
        let float: f32 = self.next_key_or_str_else_eof()?.parse()?;
        if float.is_finite() {
            Ok(float)
        } else {
            Err(Error::NonFiniteFloat(float))
        }
    }
}

impl<'de> Deref for Deserializer<'de> {
    type Target = Peekable<IntoIter<Token<'de>>>;

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl<'de> DerefMut for Deserializer<'de> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tokens
    }
}

macro_rules! forward_string_to_parse {
    ( $( ( $deserializer_name:ident, $visitor_name:ident ) ),* $(,)? ) => {
        $(
            fn $deserializer_name<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
                visitor.$visitor_name(self.next_key_or_str_else_eof()?.parse()?)
            }
        )*
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match self
            .peek()
            .expect("Tokenstream structure prevents premature end")
        {
            Token::ObjBegin => self.deserialize_map(visitor),
            Token::SeqBegin => self.deserialize_seq(visitor),
            // `Any` always falls back to a `str` when possible, because the VDF format doesn't
            // give any reasonable type information
            Token::Key(_) | Token::Str(_) => self.deserialize_str(visitor),
            Token::ObjEnd | Token::SeqEnd => unreachable!("End is always consumed with a Begin"),
        }
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let val = self.next_key_or_str_else_eof()?;
        match val.as_ref() {
            "0" => visitor.visit_bool(false),
            "1" => visitor.visit_bool(true),
            _ => Err(Error::InvalidBoolean),
        }
    }

    forward_string_to_parse!(
        (deserialize_i8, visit_i8),
        (deserialize_i16, visit_i16),
        (deserialize_i32, visit_i32),
        (deserialize_i64, visit_i64),
        (deserialize_i128, visit_i128),
        (deserialize_u8, visit_u8),
        (deserialize_u16, visit_u16),
        (deserialize_u32, visit_u32),
        (deserialize_u64, visit_u64),
        (deserialize_u128, visit_u128),
    );

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let float = self.next_finite_float_else_eof()?;
        visitor.visit_f32(float)
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let float = self.next_finite_float_else_eof()?;
        // Note: All floats are represented as through f32 since I believe that's what steam uses
        visitor.visit_f64(f64::from(float))
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let s = self.next_key_or_str_else_eof()?;
        let mut chars_iter = s.chars();
        match (chars_iter.next(), chars_iter.next()) {
            (Some(c), None) => visitor.visit_char(c),
            // Either there are no or multiple chars
            _ => Err(Error::InvalidChar),
        }
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let val = self.next_key_or_str_else_eof()?;
        match val {
            // The borrowed content can be tied to the original text's lifetime
            Cow::Borrowed(borrowed) => visitor.visit_borrowed_str(borrowed),
            // TODO: Owned strings don't actually make it to here. Find out where the owned data
            // becomes borrowed
            Cow::Owned(s) => visitor.visit_string(s),
        }
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string(self.next_key_or_str_else_eof()?.into_owned())
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        // It's unclear how `bytes` would be represented in vdf
        Err(Error::Unsupported("Bytes"))
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        // It's unclear how `byte buf` would be represented in vdf
        Err(Error::Unsupported("Byte Buf"))
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // It looks like vdf will just entirely omit values that aren't used, so if the field
        // appeared then it should be `Some`
        visitor.visit_some(self)
    }

    fn deserialize_unit<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        // It's unclear how a unit type would be represented in vdf (Empty string or obj?)
        Err(Error::Unsupported("Unit"))
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value> {
        // It's unclear how a unit type would be represented in vdf (Empty string or obj?)
        Err(Error::Unsupported("Unit Struct"))
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        // Considered just a wrapper over the contained value
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_seq(SeqBuilder::new(self).try_build()?)
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value> {
        visitor.visit_seq(SeqBuilder::new(self).length(len).try_build()?)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_seq(SeqBuilder::new(self).length(len).try_build()?)
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // A map is just an object containing a list of keyvalues
        visitor.visit_map(ObjEater::try_new(self)?)
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        // Enums are only supported to the extent of the `Str` value matching the variant. No
        // newtype, tuple, or struct variants supported due to ambiguity in vdf types
        // This is because there is no pretence (that I know of) for how enums are notated in the
        // externally, internally, and adjacently tagged enums, and the content of the data is far
        // too vague for untagged to make sense. Consider how deserializing
        // "Key"
        // {
        //      "inner"    "1"
        // }
        // with
        // ```
        // struct Outer { inner: SampleEnum, }
        // enum SampleEnum {
        //     Bool(bool),
        //     Int(i32),
        //     Optional(Option<u32>)
        //     Seq(Vec<u64>),
        //  }
        //  ```
        //  where each of these variants are equally valid for trying to determine "1".
        match self.next() {
            Some(Token::Str(s)) => visitor.visit_enum(s.into_deserializer()),
            Some(_) => Err(Error::ExpectedSomeValue),
            None => Err(Error::EofWhileParsingValue),
        }
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // An identifier is just a str
        self.deserialize_str(visitor)
    }

    // AFAIK this is just used for making sure that the deserializer travels through the right
    // amount of data so it's safe to ignore more finer grain types
    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match self
            .peek()
            .expect("Tokenstream structure prevents premature end")
        {
            Token::Key(_) | Token::Str(_) => {
                self.next().expect("Token was peeked");
                visitor.visit_none()
            }
            Token::ObjBegin => self.deserialize_map(visitor),
            Token::SeqBegin => self.deserialize_seq(visitor),
            Token::ObjEnd | Token::SeqEnd => unreachable!("End is always consumed with a Begin"),
        }
    }
}
