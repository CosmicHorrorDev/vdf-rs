// TODO: will tuples that end in a seq early break things? I.e. a tuple of two values when there
// are more

use keyvalues_parser::core::{Value, Vdf};
use regex::Regex;
use serde::{
    de::{self, DeserializeSeed, IntoDeserializer, MapAccess, SeqAccess, Visitor},
    Deserialize,
};

use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use crate::error::{Error, Result};

// I've been struggling to get serde to play nice with using a more complex internal structure in a
// `Deserializer`. I think the easiest solution I can come up with is to flatten out the `Vdf` into
// a stream of tokens that serde can consume. In this way the Deserializer can just work on
// munching through all the tokens instead of trying to mutate a more complex nested structure
// containing different types
/// A stream of tokens representing vdf. I think an example is the easiest way to understand the
/// structure so something like
/// ```no_test
/// "Outer Key" "Outer Value"
/// "Outer Key"
/// {
///     "Inner Key" "Inner Value"
/// }
/// ```
/// will be transformed into
/// ```no_test
/// Vdf({
///     "Outer Key": [
///         Str("Outer Value"),
///         Obj(
///             Vdf({
///                 "Inner Key": [
///                     Str("Inner Value")
///                 ]
///             })
///         )
///     ]
/// })
/// ```
/// which has the following token stream
/// ```no_test
/// TokenStream([
///     Key("Outer Key"),
///     SeqBegin,
///     Str("Outer Value"),
///     ObjBegin,
///     Key("Inner Key"),
///     Str("Inner Value"),
///     ObjEnd,
///     SeqEnd,
/// )]
/// ```
/// So in this way it's a linear sequence of keys and values where the value is either a str or
/// an object.
#[derive(Clone, Debug, PartialEq)]
pub struct TokenStream<'a>(Vec<Token<'a>>);

impl<'a> TokenStream<'a> {
    fn peek(&self) -> Option<&Token<'a>> {
        self.get(0)
    }

    fn peek_is_key(&self) -> bool {
        matches!(self.peek(), Some(Token::Key(_)))
    }

    fn peek_is_str(&self) -> bool {
        matches!(self.peek(), Some(Token::Str(_)))
    }

    fn peek_is_value(&self) -> bool {
        matches!(
            self.peek(),
            Some(Token::ObjBegin) | Some(Token::SeqBegin) | Some(Token::Str(_))
        )
    }

    // This is pretty bad for performance. If it's an issue we can flip the direction of the tokens
    // when we store it so that we can pop off the back instead
    fn next(&mut self) -> Option<Token<'a>> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove(0))
        }
    }

    fn next_key(&mut self) -> Option<Cow<'a, str>> {
        if self.peek_is_key() {
            if let Some(Token::Key(s)) = self.next() {
                Some(s)
            } else {
                unreachable!("Key was peeked");
            }
        } else {
            None
        }
    }

    fn next_str(&mut self) -> Option<Cow<'a, str>> {
        if self.peek_is_str() {
            if let Some(Token::Str(s)) = self.next() {
                Some(s)
            } else {
                unreachable!("Str was peeked");
            }
        } else {
            None
        }
    }

    fn next_key_or_str(&mut self) -> Option<Cow<'a, str>> {
        self.next_key().or_else(|| self.next_str())
    }
}

impl<'a> Deref for TokenStream<'a> {
    type Target = Vec<Token<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for TokenStream<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> From<Vdf<'a>> for TokenStream<'a> {
    fn from(vdf: Vdf<'a>) -> Self {
        let mut inner = Vec::new();

        for (key, values) in vdf.0.into_iter() {
            inner.push(Token::Key(key));

            // For ease of use a sequence is only marked for keys that have
            // more than one values (zero shouldn't be allowed)
            let num_values = values.len();
            if num_values != 1 {
                inner.push(Token::SeqBegin);
            }

            for value in values {
                inner.extend(TokenStream::from(value).0);
            }

            if num_values != 1 {
                inner.push(Token::SeqEnd);
            }
        }

        Self(inner)
    }
}

impl<'a> From<Value<'a>> for TokenStream<'a> {
    fn from(value: Value<'a>) -> Self {
        let mut inner = Vec::new();

        match value {
            Value::Str(s) => {
                inner.push(Token::Str(s));
            }
            Value::Obj(obj) => {
                inner.push(Token::ObjBegin);
                inner.extend(Self::from(obj).0);
                inner.push(Token::ObjEnd);
            }
        }

        Self(inner)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'a> {
    Key(Cow<'a, str>),
    Str(Cow<'a, str>),
    ObjBegin,
    ObjEnd,
    SeqBegin,
    SeqEnd,
}

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s)?;
    let t = T::deserialize(&mut deserializer)?;

    if deserializer.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingTokens)
    }
}

#[derive(Debug)]
pub struct Deserializer<'de> {
    tokens: TokenStream<'de>,
}

impl<'de> Deserializer<'de> {
    // TODO: this can really return an error from parsing here
    pub fn from_str(input: &'de str) -> Result<Self> {
        let vdf = Vdf::parse(input)?;
        let tokens = TokenStream::from(vdf);
        Ok(Self { tokens })
    }
}

impl<'de> Deref for Deserializer<'de> {
    type Target = TokenStream<'de>;

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl<'de> DerefMut for Deserializer<'de> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tokens
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.peek() {
            Some(Token::ObjBegin) => self.deserialize_map(visitor),
            Some(Token::SeqBegin) => self.deserialize_seq(visitor),
            Some(Token::Key(s)) | Some(Token::Str(s)) => {
                // Falls back to using a regex to match several patterns. This will be far from
                // efficient, but I'm feeling lazy for now
                let neg_num_regex = Regex::new(r"^-\d+$").unwrap();
                let num_regex = Regex::new(r"^\d+$").unwrap();
                let real_regex = Regex::new(r"^-?\d+\.\d+$").unwrap();

                // Check from more specific to more general types
                if s == "0" || s == "1" {
                    self.deserialize_bool(visitor)
                } else if neg_num_regex.is_match(s) {
                    self.deserialize_i64(visitor)
                } else if num_regex.is_match(s) {
                    self.deserialize_u64(visitor)
                } else if real_regex.is_match(s) {
                    self.deserialize_f64(visitor)
                } else {
                    self.deserialize_str(visitor)
                }
            }
            Some(Token::ObjEnd) => Err(Error::UnexpectedEndOfObject),
            Some(Token::SeqEnd) => Err(Error::UnexpectedEndOfSequence),
            None => Err(Error::EofWhileParsingAny),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(s) = self.next_key_or_str() {
            if s == "0" {
                visitor.visit_bool(false)
            } else if s == "1" {
                visitor.visit_bool(true)
            } else {
                Err(Error::InvalidBoolean)
            }
        } else {
            Err(Error::EofWhileParsingValue)
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(
            self.next_key_or_str()
                .ok_or(Error::EofWhileParsingValue)?
                .parse()?,
        )
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(
            self.next_key_or_str()
                .ok_or(Error::EofWhileParsingValue)?
                .parse()?,
        )
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(
            self.next_key_or_str()
                .ok_or(Error::EofWhileParsingValue)?
                .parse()?,
        )
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(
            self.next_key_or_str()
                .ok_or(Error::EofWhileParsingValue)?
                .parse()?,
        )
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(
            self.next_key_or_str()
                .ok_or(Error::EofWhileParsingValue)?
                .parse()?,
        )
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(
            self.next_key_or_str()
                .ok_or(Error::EofWhileParsingValue)?
                .parse()?,
        )
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(
            self.next_key_or_str()
                .ok_or(Error::EofWhileParsingValue)?
                .parse()?,
        )
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(
            self.next_key_or_str()
                .ok_or(Error::EofWhileParsingValue)?
                .parse()?,
        )
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(
            self.next_key_or_str()
                .ok_or(Error::EofWhileParsingValue)?
                .parse()?,
        )
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(
            self.next_key_or_str()
                .ok_or(Error::EofWhileParsingValue)?
                .parse()?,
        )
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let s = self.next_key_or_str().ok_or(Error::EofWhileParsingValue)?;
        let mut chars_iter = s.chars();
        if let Some(c) = chars_iter.next() {
            if chars_iter.next().is_none() {
                visitor.visit_char(c)
            } else {
                Err(Error::InvalidChar)
            }
        } else {
            Err(Error::EofWhileParsingValue)
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_str(&self.next_key_or_str().ok_or(Error::EofWhileParsingValue)?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(
            self.next_key_or_str()
                .ok_or(Error::EofWhileParsingValue)?
                .into_owned(),
        )
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // It's unclear how `bytes` would be represented in vdf
        Err(Error::Unsupported("Bytes"))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // It's unclear how `byte buf` would be represented in vdf
        Err(Error::Unsupported("Byte Buf"))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // It looks like vdf will just entirely omit values that aren't used, so if the field
        // appeared then it should be `Some`
        visitor.visit_some(self)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // It's unclear how a unit type would be represented in vdf (Empty string or obj?)
        Err(Error::Unsupported("Unit"))
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // It's unclear how a unit type would be represented in vdf (Empty string or obj?)
        Err(Error::Unsupported("Unit Struct"))
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Considered just a wrapper over the contained value
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqBuilder::new(&mut self).try_build()?)
    }

    fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqBuilder::new(&mut self).length(len).try_build()?)
    }

    fn deserialize_tuple_struct<V>(
        mut self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqBuilder::new(&mut self).length(len).try_build()?)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // A map is just an object containing a list of keyvalues
        visitor.visit_map(ObjEater::try_new(&mut self)?)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
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

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // An identifier is just a str
        self.deserialize_str(visitor)
    }

    // TODO: I think this will get hit if the vdf has extra keys that aren't used. Falling back
    // to deserializing to an `Obj`, `Seq`, or `Str` based on the token should be a good heuristic
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

#[derive(Debug)]
struct ObjEater<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> ObjEater<'a, 'de> {
    fn try_new(de: &'a mut Deserializer<'de>) -> Result<Self> {
        // In the case of wanting to deserialize the top level to a `HashMap`
        // pop off the top level key
        while let Some(Token::Key(_)) = de.peek() {
            de.next();
        }

        // An object starts with an `ObjBegin` and ends with `ObjEnd`
        match de.next() {
            Some(Token::ObjBegin) => Ok(Self { de }),
            Some(_) => Err(Error::ExpectedObjectStart),
            None => Err(Error::EofWhileParsingObject),
        }
    }
}

impl<'de, 'a> MapAccess<'de> for ObjEater<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        match self.de.peek() {
            Some(Token::Key(_)) => seed.deserialize(&mut *self.de).map(Some),
            Some(Token::ObjEnd) => {
                self.de.next();
                Ok(None)
            }
            Some(_) => Err(Error::ExpectedSomeIdent),
            None => Err(Error::EofWhileParsingObject),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if self.de.peek_is_value() {
            seed.deserialize(&mut *self.de)
        } else {
            Err(Error::ExpectedSomeValue)
        }
    }
}

#[derive(Debug)]
struct SeqBuilder<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    maybe_len: Option<usize>,
}

impl<'a, 'de> SeqBuilder<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self {
            de,
            maybe_len: None,
        }
    }

    fn length(mut self, len: usize) -> Self {
        self.maybe_len = Some(len);
        self
    }

    fn try_build(self) -> Result<SeqEater<'a, 'de>> {
        match (self.maybe_len, self.de.peek()) {
            (Some(len), Some(Token::SeqBegin)) if len != 1 => {
                Ok(SeqEater::new_set_length(self.de, len))
            }
            // `len` says single element, but `SeqBegin` indicates otherwise
            (Some(_), Some(Token::SeqBegin)) => Err(Error::TrailingTokens),
            (None, Some(Token::SeqBegin)) => Ok(SeqEater::new_variable_length(self.de)),
            // TODO: these can be condensed once 1.53 lands
            (_, Some(Token::ObjBegin)) => Ok(SeqEater::new_single_value(self.de)),
            (_, Some(Token::Str(_))) => Ok(SeqEater::new_single_value(self.de)),
            _ => Err(Error::ExpectedSomeValue),
        }
    }
}

#[derive(Debug)]
enum SeqEater<'a, 'de: 'a> {
    SingleValue(SingleValueEater<'a, 'de>),
    SetLength(SetLengthEater<'a, 'de>),
    VariableLength(VariableLengthEater<'a, 'de>),
}

impl<'a, 'de> SeqEater<'a, 'de> {
    fn new_single_value(de: &'a mut Deserializer<'de>) -> Self {
        Self::SingleValue(SingleValueEater {
            de,
            finished: false,
        })
    }

    fn new_set_length(de: &'a mut Deserializer<'de>, remaining: usize) -> Self {
        // Pop off the marker
        if let Some(Token::SeqBegin) = de.next() {
            Self::SetLength(SetLengthEater { de, remaining })
        } else {
            unreachable!("SeqBegin was peeked");
        }
    }

    fn new_variable_length(de: &'a mut Deserializer<'de>) -> Self {
        // Pop off the marker
        if let Some(Token::SeqBegin) = de.next() {
            Self::VariableLength(VariableLengthEater { de })
        } else {
            unreachable!("SeqBegin was peeked");
        }
    }
}

impl<'de, 'a> SeqAccess<'de> for SeqEater<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self {
            Self::SingleValue(eater) => eater.next_element_seed(seed),
            Self::SetLength(eater) => eater.next_element_seed(seed),
            Self::VariableLength(eater) => eater.next_element_seed(seed),
        }
    }
}

#[derive(Debug)]
struct SingleValueEater<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    finished: bool,
}

impl<'de, 'a> SingleValueEater<'a, 'de> {
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.finished {
            Ok(None)
        } else {
            self.finished = true;
            match self.de.peek() {
                Some(Token::ObjBegin) | Some(Token::Str(_)) => {
                    seed.deserialize(&mut *self.de).map(Some)
                }
                Some(_) => Err(Error::ExpectedSomeValue),
                None => Err(Error::EofWhileParsingSequence),
            }
        }
    }
}

#[derive(Debug)]
struct SetLengthEater<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    remaining: usize,
}

impl<'de, 'a> SetLengthEater<'a, 'de> {
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.de.peek() {
            Some(Token::ObjBegin) | Some(Token::Str(_)) => {
                self.remaining -= 1;
                let val = seed.deserialize(&mut *self.de).map(Some)?;

                // Eagerly pop off the end marker since this won't get called again
                if self.remaining == 0 {
                    match self.de.next() {
                        Some(Token::SeqEnd) => Ok(()),
                        Some(_) => Err(Error::TrailingTokens),
                        None => Err(Error::EofWhileParsingSequence),
                    }?;
                }

                Ok(val)
            }
            Some(Token::SeqEnd) => Err(Error::UnexpectedEndOfSequence),
            Some(_) => Err(Error::ExpectedSomeValue),
            None => Err(Error::EofWhileParsingSequence),
        }
    }
}

#[derive(Debug)]
struct VariableLengthEater<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'de, 'a> VariableLengthEater<'a, 'de> {
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.de.peek() {
            Some(Token::ObjBegin) | Some(Token::Str(_)) => {
                seed.deserialize(&mut *self.de).map(Some)
            }
            Some(Token::SeqEnd) => {
                // Pop off the marker
                if let Some(Token::SeqEnd) = self.de.next() {
                    Ok(None)
                } else {
                    unreachable!("SeqEnd was peeked");
                }
            }
            Some(_) => Err(Error::ExpectedSomeValue),
            None => Err(Error::EofWhileParsingSequence),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Container<T> {
        inner: T,
    }

    impl<T> Container<T> {
        fn new(inner: T) -> Self {
            Self { inner }
        }
    }

    #[test]
    fn basic_struct() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct TestStruct {
            field1: i32,
            field2: String,
        }

        let s = r#"
"TestStruct"
{
    "field1" "-123"
    "field2" "Sample String"
}
        "#;

        let sample: TestStruct = from_str(s).unwrap();
        assert_eq!(
            sample,
            TestStruct {
                field1: -123,
                field2: String::from("Sample String")
            }
        )
    }

    #[test]
    fn basic_types() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct BasicTypes {
            boolean: bool,
            character: char,
            signed8: i8,
            signed16: i16,
            signed32: i32,
            signed64: i64,
            unsigned8: u8,
            unsigned16: u16,
            unsigned32: u32,
            unsigned64: u64,
            float32: f32,
            float64: f64,
        }

        let s = r#"
"Key"
{
    "boolean" "0"
    "character" "a"
    "signed8" "1"
    "signed16" "2"
    "signed32" "3"
    "signed64" "4"
    "unsigned8" "5"
    "unsigned16" "6"
    "unsigned32" "7"
    "unsigned64" "8"
    "float32" "1.0"
    "float64" "2.0"
}
        "#;

        let sample: BasicTypes = from_str(s).unwrap();
        assert_eq!(
            sample,
            BasicTypes {
                boolean: false,
                character: 'a',
                signed8: 1,
                signed16: 2,
                signed32: 3,
                signed64: 4,
                unsigned8: 5,
                unsigned16: 6,
                unsigned32: 7,
                unsigned64: 8,
                float32: 1.0,
                float64: 2.0
            }
        );
    }

    #[test]
    fn nested_structs() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct OuterStruct {
            field: String,
            inner1: InnerStruct,
            inner2: InnerStruct,
        }

        #[derive(Deserialize, Debug, PartialEq)]
        struct InnerStruct {
            field: String,
        }

        let s = r#"
"OuterStruct"
{
    "field" "Outer Value"
    "inner1"
    {
        "field" "Inner1 Value"
    }
    "inner2"
    {
        "field" "Inner2 Value"
    }
}
        "#;

        let sample: OuterStruct = from_str(s).unwrap();
        assert_eq!(
            sample,
            OuterStruct {
                field: String::from("Outer Value"),
                inner1: InnerStruct {
                    field: String::from("Inner1 Value"),
                },
                inner2: InnerStruct {
                    field: String::from("Inner2 Value"),
                }
            },
        );
    }

    #[test]
    fn newtype_struct() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct I32Wrapper(i32);

        let s = r#"
"Container"
{
    "inner" "123"
}
        "#;

        let sample: Container<I32Wrapper> = from_str(s).unwrap();
        assert_eq!(sample, Container::new(I32Wrapper(123)));
    }

    #[test]
    fn unit_variant_enum() {
        #[derive(Deserialize, Debug, PartialEq)]
        enum SampleEnum {
            Foo,
            Bar,
        }

        let s = r#"
"Key"
{
    "inner" "Foo"
}
        "#;
        let sample: Container<SampleEnum> = from_str(s).unwrap();
        assert_eq!(sample, Container::new(SampleEnum::Foo));
    }

    #[test]
    fn sequence() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Inner {
            field: String,
        }

        let single_str = r#"
"Key"
{
    "inner"
    {
        "field" "Some String"
    }
}
        "#;

        let single: Container<Vec<Inner>> = from_str(single_str).unwrap();
        assert_eq!(
            single,
            Container::new(vec![Inner {
                field: String::from("Some String")
            }])
        );

        let double_str = r#"
"Key"
{
    "inner"
    {
        "field" "Some String"
    }
    "inner"
    {
        "field" "Another String"
    }
}
        "#;

        let double: Container<Vec<Inner>> = from_str(double_str).unwrap();
        assert_eq!(
            double,
            Container::new(vec![
                Inner {
                    field: String::from("Some String")
                },
                Inner {
                    field: String::from("Another String")
                }
            ])
        );
    }

    #[test]
    fn tuple() {
        let s = r#"
"Key"
{
    "inner" "1"
    "inner" "2"
    "inner" "Sample Text"
}
        "#;

        let sample: Container<(bool, i32, String)> = from_str(s).unwrap();
        assert_eq!(
            sample,
            Container::new((true, 2, String::from("Sample Text")))
        );
    }

    #[test]
    fn tuple_struct() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct TupleStruct(bool, i32, String);

        let s = r#"
"Key"
{
    "inner" "1"
    "inner" "2"
    "inner" "Sample Text"
}
        "#;

        let sample: Container<TupleStruct> = from_str(s).unwrap();
        assert_eq!(
            sample,
            Container::new(TupleStruct(true, 2, String::from("Sample Text"),))
        )
    }

    // TODO: it's not clear if the ordering of values is expected to stay the
    // same in vdf. If that is the case then it would be important to track
    // down a map type that preserves insertion order
    #[test]
    fn hashmap() {
        let nested = r#"
"Key"
{
    "inner"
    {
        "0" "Foo"
        "1" "Bar"
        "2" "Baz"
    }
}
        "#;

        let mut ideal = HashMap::new();
        ideal.insert(0, "Foo".to_owned());
        ideal.insert(1, "Bar".to_owned());
        ideal.insert(2, "Baz".to_owned());

        let sample: Container<HashMap<u64, String>> = from_str(nested).unwrap();
        assert_eq!(sample, Container::new(ideal.clone()));

        let top_level = r#"
"Key"
{
    "0" "Foo"
    "1" "Bar"
    "2" "Baz"
}
        "#;

        let sample: HashMap<u64, String> = from_str(top_level).unwrap();
        assert_eq!(sample, ideal);
    }

    #[test]
    fn option() {
        let none_str = r#"
"Key"
{
}
        "#;

        let none: Container<Option<String>> = from_str(none_str).unwrap();
        assert_eq!(none, Container::new(None));

        let some_str = r#"
"Key"
{
    "inner" "Some value"
}
        "#;

        let some: Container<Option<String>> = from_str(some_str).unwrap();
        assert_eq!(some, Container::new(Some(String::from("Some value"))));
    }

    #[test]
    fn token_stream() {
        let s = r#"
"Outer Key" "Outer Value"
"Outer Key"
{
    "Inner Key" "Inner Value"
}
        "#;
        let vdf = Vdf::parse(s).unwrap();
        let token_stream = TokenStream::from(vdf);
        assert_eq!(
            token_stream,
            TokenStream(vec![
                Token::Key(Cow::from("Outer Key")),
                Token::SeqBegin,
                Token::Str(Cow::from("Outer Value")),
                Token::ObjBegin,
                Token::Key(Cow::from("Inner Key")),
                Token::Str(Cow::from("Inner Value")),
                Token::ObjEnd,
                Token::SeqEnd,
            ])
        );
    }
}
