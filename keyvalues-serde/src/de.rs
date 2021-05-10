// FIXME: replace the unwraps here with actual error handling

use keyvalues_parser::core::{Key, Value, Vdf};
use serde::{
    de::{self, DeserializeSeed, MapAccess, Visitor},
    Deserialize,
};

use std::borrow::Cow;

use crate::error::{Error, Result};

// I've been struggling to get serde to play nice with using a more complex internal structure in a
// `Deserializer`. I think the easiest solution I can come up with is to flatten out the `Vdf` into
// a stream of tokens that serde can consume. In this way the Deserializer can just work on
// munching through all the tokens instead of trying to mutate a more complex nested structure
// containing different types
/// A stream of tokens representing vdf. I think an example is the easiest way to understand the
/// structure so something like
/// ```
/// "Outer Key" "Outer Value"
/// "Outer Key"
/// {
///     "Inner Key" "Inner Value"
/// }
/// ```
/// will be transformed into
/// ```
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
/// ```
/// TokenStream([
///     Key("Outer Key"),
///     Str("Outer Value"),
///     Key("Outer Key"),
///     ObjBegin,
///     Key("Inner Key"),
///     Str("Inner Value"),
///     ObjEnd,
/// )]
/// ```
/// So in this way it's a linear sequence of keys and values where the value is either a str or
/// an object.
#[derive(Clone, Debug, PartialEq)]
struct TokenStream<'a>(Vec<Token<'a>>);

impl<'a> From<Vdf<'a>> for TokenStream<'a> {
    fn from(vdf: Vdf<'a>) -> Self {
        let mut inner = Vec::new();

        for (key, values) in vdf.0.into_iter() {
            for value in values {
                inner.push(Token::Key(key.clone()));
                match value {
                    Value::Str(s) => {
                        inner.push(Token::Str(s));
                    }
                    Value::Obj(obj) => {
                        inner.push(Token::ObjBegin);
                        inner.extend(TokenStream::from(obj).0);
                        inner.push(Token::ObjEnd);
                    }
                }
            }
        }

        Self(inner)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Token<'a> {
    Key(Cow<'a, str>),
    Str(Cow<'a, str>),
    ObjBegin,
    ObjEnd,
}

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

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_str(s);
    let t = T::deserialize(&mut deserializer)?;
    // TODO: potentially do some validation here
    Ok(t)
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
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
        todo!()
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
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

struct ObjEater<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> ObjEater<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self { de }
    }
}

impl<'de, 'a> MapAccess<'de> for ObjEater<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        todo!()
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestStruct {
        field1: i32,
        field2: String,
    }

    #[test]
    fn dev_helper() {
        let s = r#"
"TestStruct"
{
    "field1" "-123"
    "field2" "Sample String"
}
        "#;

        let sample: Result<TestStruct> = from_str(s);
        println!("{:#?}", sample);
        todo!();
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
                Token::Str(Cow::from("Outer Value")),
                Token::Key(Cow::from("Outer Key")),
                Token::ObjBegin,
                Token::Key(Cow::from("Inner Key")),
                Token::Str(Cow::from("Inner Value")),
                Token::ObjEnd,
            ])
        );
    }
}
