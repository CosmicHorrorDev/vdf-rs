pub mod owned;

use std::{
    collections::{
        btree_map::{Iter, Keys, Values},
        BTreeMap,
    },
    ops::Index,
};

pub type Key<'a> = &'a str;
pub type KeyValues<'a> = BTreeMap<Key<'a>, Vec<Value<'a>>>;

#[derive(Debug, PartialEq, Default)]
pub struct Vdf<'a>(pub KeyValues<'a>);

impl<'a> Vdf<'a> {
    pub fn inner(&self) -> &KeyValues {
        todo!()
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&Vec<Value>> {
        self.0.get(key)
    }

    pub fn get_key_value(&self, key: &str) -> Option<(&Key, &Vec<Value>)> {
        self.0.get_key_value(key)
    }

    pub fn iter(&self) -> Iter<'_, Key, Vec<Value>> {
        self.0.iter()
    }

    pub fn keys(&self) -> Keys<'_, Key, Vec<Value>> {
        self.0.keys()
    }

    pub fn values(&self) -> Values<'_, Key, Vec<Value>> {
        self.0.values()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Str(&'a str),
    Obj(Vdf<'a>),
}

impl<'a> Value<'a> {
    pub fn is_str(&self) -> bool {
        self.get_str().is_some()
    }

    pub fn is_obj(&self) -> bool {
        self.get_obj().is_some()
    }

    pub fn get_str(&self) -> Option<&str> {
        if let Value::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn get_obj(&self) -> Option<&Vdf> {
        if let Value::Obj(obj) = self {
            Some(&obj)
        } else {
            None
        }
    }

    // TODO: work out error situation
    pub fn try_get_str(&self) -> Result<&str, ()> {
        self.get_str().ok_or(())
    }

    // TODO: work out error situation
    pub fn try_get_obj(&self) -> Result<&Vdf, ()> {
        self.get_obj().ok_or(())
    }
}

impl<'a> Index<&str> for Vdf<'a> {
    type Output = Vec<Value<'a>>;

    fn index(&self, needle: &str) -> &Self::Output {
        &self.0[needle]
    }
}
