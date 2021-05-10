// FIXME: replace the unwraps here with actual error handling

use keyvalues_parser::core::{Value, Vdf};
use serde::{
    de::{self, DeserializeSeed, MapAccess, Visitor},
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

impl<'a> TokenStream<'a> {
    fn peek(&self) -> Option<&Token<'a>> {
        self.get(0)
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

impl<'a> Token<'a> {
    fn is_key(&self) -> bool {
        matches!(self, Token::Key(_))
    }

    fn is_str(&self) -> bool {
        matches!(self, Token::Str(_))
    }

    fn is_obj_begin(&self) -> bool {
        matches!(self, Token::ObjBegin)
    }

    fn is_obj_end(&self) -> bool {
        matches!(self, Token::ObjEnd)
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

impl<'de> Deserializer<'de> {
    fn peek(&self) -> Option<&Token<'de>> {
        self.tokens.peek()
    }

    fn next(&mut self) -> Option<Token<'de>> {
        self.tokens.next()
    }
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

    // TODO: can this be for anything other than `Str`?
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(Token::Str(s)) = self.next() {
            visitor.visit_i32(s.parse().unwrap())
        } else {
            todo!()
        }
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
        if let Some(token) = self.next() {
            match token {
                Token::Key(s) | Token::Str(s) => visitor.visit_str(&s),
                _ => todo!(),
            }
        } else {
            todo!()
        }
    }

    // TODO: can this be for anything other than `Str`
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(Token::Str(s)) = self.next() {
            visitor.visit_string(s.into_owned())
        } else {
            todo!()
        }
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
        // Considered just a wrapper over the contained value
        visitor.visit_newtype_struct(self)
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
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // An identifier is just a str
        self.deserialize_str(visitor)
    }

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
        if let Some(token) = de.next() {
            assert!(token.is_obj_begin());
        } else {
            todo!()
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
        match self.de.peek() {
            Some(Token::Str(_)) | Some(Token::ObjBegin) => seed.deserialize(&mut *self.de),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn tuple_struct() {
        #[derive(Deserialize, Debug, PartialEq)]
        struct Container {
            inner: I32Wrapper,
        }

        #[derive(Deserialize, Debug, PartialEq)]
        struct I32Wrapper(i32);

        let s = r#"
"Container"
{
    "inner" "123"
}
        "#;

        let sample: Container = from_str(s).unwrap();
        assert_eq!(
            sample,
            Container {
                inner: I32Wrapper(123)
            }
        );
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
