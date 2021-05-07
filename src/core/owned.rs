use std::collections::BTreeMap;

use crate::core::{Value, Vdf};

pub type KeyBuf = String;
pub type KeyValuesBuf = BTreeMap<KeyBuf, Vec<ValueBuf>>;

#[derive(Debug, PartialEq, Default)]
pub struct VdfBuf(pub KeyValuesBuf);

impl VdfBuf {
    pub fn to_vdf(&self) -> Vdf {
        let inner = self
            .0
            .iter()
            .map(|(key, values)| {
                let key_ref = key.as_str();
                let values_ref = values.iter().map(|val| val.to_value()).collect();
                (key_ref, values_ref)
            })
            .collect();

        Vdf(inner)
    }
}

#[derive(Debug, PartialEq)]
pub enum ValueBuf {
    Str(String),
    Obj(VdfBuf),
}

impl ValueBuf {
    pub fn to_value(&self) -> Value {
        match self {
            ValueBuf::Str(string) => Value::Str(string.as_str()),
            ValueBuf::Obj(obj) => Value::Obj(obj.to_vdf()),
        }
    }
}
