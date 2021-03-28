use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct Vdf<'a>(pub Pair<'a>);

impl<'a> Vdf<'a> {
    pub fn inner(&self) -> &Pair {
        &self
    }
}

impl<'a> Deref for Vdf<'a> {
    type Target = Pair<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct Pair<'a>(pub &'a str, pub Value<'a>);

impl<'a> Pair<'a> {
    pub fn key(&self) -> &str {
        &self.0
    }

    pub fn value(&self) -> &'a Value {
        &self.1
    }
}

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Str(&'a str),
    Obj(Vec<Pair<'a>>),
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

    pub fn get_obj(&self) -> Option<&[Pair]> {
        if let Value::Obj(obj) = self {
            Some(&obj)
        } else {
            None
        }
    }

    // TODO: get the error situation worked out here
    pub fn try_get_str(&self) -> Result<&str, ()> {
        self.get_str().ok_or(())
    }

    // TODO: get the error situation worked out here
    pub fn try_get_obj(&self) -> Result<&[Pair], ()> {
        self.get_obj().ok_or(())
    }
}
