use serde::de::{DeserializeSeed, MapAccess};

use crate::{
    de::Deserializer,
    error::{Error, Result},
    tokens::Token,
};

#[derive(Debug)]
pub struct ObjEater<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> ObjEater<'a, 'de> {
    pub fn try_new(de: &'a mut Deserializer<'de>) -> Result<Self> {
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
