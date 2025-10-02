use std::{borrow::Cow, collections::BTreeMap, fmt};

use crate::{Obj, Value};

use serde_core::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};

fn value_string(s: impl ToString) -> Value<'static> {
    Value::Str(s.to_string().into())
}

struct ValueVisitor;

impl<'a> Visitor<'a> for ValueVisitor {
    type Value = Value<'a>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("any valid VDF value")
    }

    fn visit_borrowed_str<E>(self, s: &'a str) -> Result<Self::Value, E> {
        Ok(Value::Str(Cow::Borrowed(s)))
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> {
        Ok(value_string(s))
    }

    fn visit_string<E>(self, s: String) -> Result<Self::Value, E> {
        Ok(Value::Str(Cow::Owned(s)))
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'a>,
    {
        let mut obj = BTreeMap::new();
        while let Some((key, value)) = visitor.next_entry()? {
            obj.insert(key, value);
        }
        Ok(Value::Obj(crate::Obj(obj)))
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Value<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

struct ObjVisitor;

impl<'a> Visitor<'a> for ObjVisitor {
    type Value = Obj<'a>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("an object")
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'a>,
    {
        let mut obj = BTreeMap::new();
        while let Some((key, value)) = visitor.next_entry()? {
            obj.insert(key, value);
        }
        Ok(Obj(obj))
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Obj<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ObjVisitor)
    }
}
