use std::ops::Deref;

#[derive(Debug, PartialEq)]
pub struct Vdf<'a>(pub VdfPair<'a>);

impl<'a> Vdf<'a> {
    pub fn inner(&self) -> &VdfPair {
        &self
    }
}

impl<'a> Deref for Vdf<'a> {
    type Target = VdfPair<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
pub struct VdfPair<'a>(pub &'a str, pub VdfValue<'a>);

impl<'a> VdfPair<'a> {
    pub fn key(&self) -> &str {
        &self.0
    }

    pub fn value(&self) -> &'a VdfValue {
        &self.1
    }
}

#[derive(Debug, PartialEq)]
pub enum VdfValue<'a> {
    Str(&'a str),
    Obj(Vec<VdfPair<'a>>),
}

impl<'a> VdfValue<'a> {
    pub fn is_str(&self) -> bool {
        self.get_str().is_some()
    }

    pub fn is_obj(&self) -> bool {
        self.get_obj().is_some()
    }

    pub fn get_str(&self) -> Option<&str> {
        if let VdfValue::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn get_obj(&self) -> Option<&[VdfPair]> {
        if let VdfValue::Obj(obj) = self {
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
    pub fn try_get_obj(&self) -> Result<&[VdfPair], ()> {
        self.get_obj().ok_or(())
    }
}
