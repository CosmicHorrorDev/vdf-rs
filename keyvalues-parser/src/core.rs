use std::{
    borrow::Cow,
    collections::{
        btree_map::{Iter, IterMut, Keys, Range, RangeMut, Values, ValuesMut},
        BTreeMap,
    },
    ops::Index,
    ops::RangeBounds,
};

pub type Key<'a> = Cow<'a, str>;
pub type KeyValues<'a> = BTreeMap<Key<'a>, Vec<Value<'a>>>;

// TODO: handle `Extend`, `FromIterator` and `IntoIterator` as well
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Vdf<'a>(pub KeyValues<'a>);

// TODO: is there anything for generating delegate methods? All of these so far are just delegate
// methods
// TODO: implement some of the traits that `BTreeMap` has too
impl<'a> Vdf<'a> {
    pub fn consume_value(self) -> Vec<Value<'a>> {
        self.0.into_iter().next().unwrap().1
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&Vec<Value>> {
        self.0.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Vec<Value<'a>>> {
        self.0.get_mut(key)
    }

    pub fn get_key_value(&self, key: &str) -> Option<(&Key, &Vec<Value>)> {
        self.0.get_key_value(key)
    }

    pub fn iter(&self) -> Iter<'_, Key, Vec<Value>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Key<'a>, Vec<Value<'a>>> {
        self.0.iter_mut()
    }

    pub fn keys(&self) -> Keys<'_, Key, Vec<Value>> {
        self.0.keys()
    }

    pub fn values(&self) -> Values<'_, Key, Vec<Value>> {
        self.0.values()
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, Key<'a>, Vec<Value<'a>>> {
        self.0.values_mut()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn range<R>(&self, bounds: R) -> Range<'_, Key, Vec<Value>>
    where
        R: RangeBounds<Key<'a>>,
    {
        self.0.range(bounds)
    }

    pub fn range_mut<R>(&mut self, bounds: R) -> RangeMut<'_, Key<'a>, Vec<Value<'a>>>
    where
        R: RangeBounds<Key<'a>>,
    {
        self.0.range_mut(bounds)
    }

    pub fn remove(&mut self, key: &str) -> Option<Vec<Value>> {
        self.0.remove(key)
    }

    pub fn remove_entry(&mut self, key: &str) -> Option<(Key<'a>, Vec<Value<'a>>)> {
        self.0.remove_entry(key)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Value<'a> {
    Str(Cow<'a, str>),
    Obj(Vdf<'a>),
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

    pub fn get_obj(&self) -> Option<&Vdf> {
        if let Value::Obj(obj) = self {
            Some(&obj)
        } else {
            None
        }
    }

    // TODO: work out error situation
    pub fn try_get_str(&self) -> Result<&Cow<'a, str>, ()> {
        self.get_str().ok_or(())
    }

    // TODO: work out error situation
    pub fn try_get_obj(&self) -> Result<&Vdf, ()> {
        self.get_obj().ok_or(())
    }
}

impl<'a> Index<&str> for Vdf<'a> {
    type Output = Vec<Value<'a>>;

    fn index(&self, index: &str) -> &Self::Output {
        &self.0[index]
    }
}
