use std::{borrow::Cow, collections::BTreeMap};

pub type Key<'a> = Cow<'a, str>;
pub type Obj<'a> = BTreeMap<Key<'a>, Vec<Value<'a>>>;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vdf<'a> {
    pub key: Key<'a>,
    pub value: Value<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value<'a> {
    Str(Cow<'a, str>),
    Obj(Obj<'a>),
}

impl<'a> Value<'a> {
    pub fn is_str(&self) -> bool {
        self.get_str().is_some()
    }

    pub fn is_obj(&self) -> bool {
        self.get_obj().is_some()
    }

    pub fn get_str(&self) -> Option<&Cow<'a, str>> {
        if let Value::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn get_obj(&self) -> Option<&Obj> {
        if let Value::Obj(obj) = self {
            Some(&obj)
        } else {
            None
        }
    }
}
