use std::ops::Deref;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct VdfPair<'a>(pub &'a str, pub VdfValue<'a>);

impl<'a> VdfPair<'a> {
    pub fn key(&self) -> &str {
        &self.0
    }

    pub fn value(&self) -> &VdfValue {
        &self.1
    }
}

#[derive(Debug)]
pub enum VdfValue<'a> {
    Str(&'a str),
    Obj(Vec<VdfPair<'a>>),
}

impl<'a> VdfValue<'a> {
    pub fn is_str(&self) -> bool {
        if let Self::Str(_) = &self {
            true
        } else {
            false
        }
    }

    pub fn is_obj(&self) -> bool {
        if let Self::Obj(_) = &self {
            true
        } else {
            false
        }
    }
}
