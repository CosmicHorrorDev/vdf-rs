// FIXME: replace the unwraps here with actual error handling

use keyvalues_parser::core::{Value, Vdf};
use regex::Regex;
use serde::{
    de::{
        self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
        Visitor,
    },
    Deserialize,
};

use std::{
    borrow::Cow,
    fmt,
    ops::{AddAssign, Deref, DerefMut, MulAssign},
    str::FromStr,
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

    fn next_str(&mut self) -> Result<Cow<'a, str>> {
        if let Some(Token::Str(s)) = self.next() {
            Ok(s)
        } else {
            todo!()
        }
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
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    // TODO: potentially do some validation here
    Ok(t)
}

#[derive(Debug)]
pub struct Deserializer<'de> {
    tokens: TokenStream<'de>,
}

impl<'de> Deserializer<'de> {
    // TODO: this can really return an error from parsing here
    pub fn from_str(input: &'de str) -> Self {
        let vdf = Vdf::parse(input).unwrap();
        let tokens = TokenStream::from(vdf);
        Self { tokens }
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
            Some(Token::Str(s)) => {
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
            _ => todo!(),
        }
    }

    // TODO: find examples of bools being used in vdf to know their format
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Ok(s) = self.next_str() {
            if s == "0" {
                visitor.visit_bool(false)
            } else if s == "1" {
                visitor.visit_bool(true)
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }

    // TODO: can this be for anything other than `Str`?
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.next_str().unwrap().parse().unwrap())
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.next_str().unwrap().parse().unwrap())
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.next_str().unwrap().parse().unwrap())
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.next_str().unwrap().parse().unwrap())
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.next_str().unwrap().parse().unwrap())
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.next_str().unwrap().parse().unwrap())
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.next_str().unwrap().parse().unwrap())
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.next_str().unwrap().parse().unwrap())
    }

    // TODO: try to find usages of real numbers in vdf
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.next_str().unwrap().parse().unwrap())
    }

    // TODO: try to find usages of real numbers in vdf
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.next_str().unwrap().parse().unwrap())
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let s = self.next_str()?;
        let mut chars_iter = s.chars();
        if let Some(c) = chars_iter.next() {
            assert!(chars_iter.next().is_none());
            visitor.visit_char(c)
        } else {
            todo!()
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.next() {
            Some(Token::Key(s)) | Some(Token::Str(s)) => visitor.visit_str(&s),
            _ => todo!(),
        }
    }

    // TODO: can this be for anything other than `Str`
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let s = self.next_str()?;
        visitor.visit_string(s.into_owned())
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // It's unclear how `bytes` would be represented in vdf
        todo!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // It's unclear how `byte buf` would be represented in vdf
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // TODO: it looks like this is the empty string or at least in some cases?
        // It's unclear how a null type would be represented in vdf (Empty string or obj?)
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // It's unclear how a unit type would be represented in vdf (Empty string or obj?)
        todo!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // It's unclear how a unit type would be represented in vdf (Empty string or obj?)
        todo!()
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
        visitor.visit_seq(SeqEater::try_new(&mut self)?)
    }

    // TODO: the length here can help make `SeqEater` logic simpler
    fn deserialize_tuple<V>(mut self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqEater::try_new(&mut self)?)
    }

    // TODO: the length here can help make `SeqEater` logic simpler
    fn deserialize_tuple_struct<V>(
        mut self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SeqEater::try_new(&mut self)?)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // A map is just an object containing a list of keyvalues
        visitor.visit_map(ObjEater::try_new(&mut self).unwrap())
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
        // A struct is either a key followed by an obj, or just an obj in the
        // case of a nested struct where the key is already popped off
        match self.peek() {
            Some(Token::Key(_)) => {
                self.next();
            }
            Some(Token::ObjBegin) => {}
            _ => todo!(),
        }

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
        // TODO: should this be a next or just a peek?
        match self.next() {
            Some(Token::Str(s)) => visitor.visit_enum(s.into_deserializer()),
            Some(Token::ObjBegin) => {
                todo!()
            }
            Some(Token::SeqBegin) => {
                todo!()
            }
            _ => todo!(),
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
        todo!()
    }
}

#[derive(Debug)]
struct ObjEater<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> ObjEater<'a, 'de> {
    fn try_new(de: &'a mut Deserializer<'de>) -> Result<Self> {
        // An object starts with an `ObjBegin` and ends with `ObjEnd`
        match de.next() {
            Some(Token::ObjBegin) => {}
            _ => todo!(),
        }

        Ok(Self { de })
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
            _ => todo!(),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        if self.de.peek_is_value() {
            seed.deserialize(&mut *self.de)
        } else {
            todo!()
        }
    }
}

// TODO: test how this works with nested sequences
#[derive(Debug)]
struct SeqEater<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    single_value: bool,
    finished: bool,
}

impl<'a, 'de> SeqEater<'a, 'de> {
    fn try_new(de: &'a mut Deserializer<'de>) -> Result<Self> {
        match de.peek() {
            Some(Token::SeqBegin) => {
                // Pop off the marker
                de.next();

                Ok(Self {
                    de,
                    single_value: false,
                    finished: false,
                })
            }
            // A sequence with just a single value isn't surrounded by `Seq*`
            // tags where it can either be an `Obj` or a `Str`
            Some(Token::ObjBegin) | Some(Token::Str(_)) => Ok(Self {
                de,
                single_value: true,
                finished: false,
            }),
            _ => todo!(),
        }
    }
}

impl<'de, 'a> SeqAccess<'de> for SeqEater<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.finished {
            return Ok(None);
        }

        if self.single_value {
            match self.de.peek() {
                Some(Token::Str(_)) | Some(Token::ObjBegin) => {
                    self.finished = true;
                    seed.deserialize(&mut *self.de).map(Some)
                }
                _ => todo!(),
            }
        } else {
            if self.de.peek_is_value() {
                let res = seed.deserialize(&mut *self.de).map(Some);

                // Eagerly check for the `SeqEnd`. This is done because some datatypes like `Vec`
                // will continue iterating till `SeqEnd` is hit while types with a known size like
                // tuples will end iterating when they've consumed all desired elements which
                // leaves a lingering `SeqEnd` on without doing this
                if let Some(Token::SeqEnd) = self.de.peek() {
                    self.de.next();
                    self.finished = true;
                }

                res
            } else {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn newtype_variant_enum() {
        #[derive(Deserialize, Debug, PartialEq)]
        #[serde(untagged)]
        enum NewTypeEnum {
            Variant(i32),
        }

        let s = r#"
"Key"
{
    "inner" "123"
}
        "#;

        let sample: Container<NewTypeEnum> = from_str(s).unwrap();
        assert_eq!(sample, Container::new(NewTypeEnum::Variant(123)));
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

        let single: Container<Vec<Inner>> = from_str(single_str).unwrap();
        assert_eq!(
            single,
            Container::new(vec![Inner {
                field: String::from("Some String")
            }])
        );

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
            Container::new(TupleStruct(true, 2, String::from("Sample Text")))
        )
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
