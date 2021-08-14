use std::collections::BTreeMap;

use crate::{error::Result, Value, Vdf};

pub type OwnedKey = String;

pub type OwnedObj = BTreeMap<OwnedKey, Vec<OwnedValue>>;

#[cfg_attr(test, derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OwnedVdf {
    pub key: OwnedKey,
    pub value: OwnedValue,
}

impl<'a> From<&Vdf<'a>> for OwnedVdf {
    fn from(vdf: &Vdf<'a>) -> Self {
        Self {
            key: vdf.key.to_string(),
            value: OwnedValue::from(&vdf.value),
        }
    }
}

impl OwnedVdf {
    pub fn new(key: OwnedKey, value: OwnedValue) -> Self {
        Self { key, value }
    }

    pub fn parse(s: &str) -> Result<Self> {
        let vdf = Vdf::parse(s)?;
        Ok(Self::from(&vdf))
    }
}

#[cfg_attr(test, derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OwnedValue {
    Str(String),
    Obj(OwnedObj),
}

impl<'a> From<&Value<'a>> for OwnedValue {
    fn from(value: &Value<'a>) -> Self {
        match value {
            Value::Str(s) => Self::Str(s.to_string()),
            Value::Obj(obj) => {
                let owned_obj = obj
                    .iter()
                    .map(|(key, vals)| {
                        let owned_key = key.to_string();
                        let owned_vals = vals.iter().map(OwnedValue::from).collect();
                        (owned_key, owned_vals)
                    })
                    .collect();

                Self::Obj(owned_obj)
            }
        }
    }
}

impl OwnedValue {
    pub fn is_str(&self) -> bool {
        self.get_str().is_some()
    }

    pub fn is_obj(&self) -> bool {
        self.get_obj().is_some()
    }

    pub fn get_str(&self) -> Option<&str> {
        if let Self::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn get_obj(&self) -> Option<&OwnedObj> {
        if let Self::Obj(obj) = self {
            Some(obj)
        } else {
            None
        }
    }

    pub fn get_mut_str(&mut self) -> Option<&mut String> {
        if let Self::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn get_mut_obj(&mut self) -> Option<&mut OwnedObj> {
        if let Self::Obj(obj) = self {
            Some(obj)
        } else {
            None
        }
    }

    pub fn unwrap_str(self) -> String {
        self.expect_str("Called `unwrap_str` on a `Value::Obj` variant")
    }

    pub fn unwrap_obj(self) -> OwnedObj {
        self.expect_obj("Called `unwrap_obj` on a `Value::Str` variant")
    }

    pub fn expect_str(self, msg: &str) -> String {
        if let Self::Str(s) = self {
            s
        } else {
            panic!("{}", msg)
        }
    }

    pub fn expect_obj(self, msg: &str) -> OwnedObj {
        if let Self::Obj(obj) = self {
            obj
        } else {
            panic!("{}", msg)
        }
    }
}
