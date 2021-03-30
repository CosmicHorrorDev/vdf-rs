use std::ops::Index;

use crate::common::{Pair, Value, Vdf};

// TODO: implement Borrow and ToOwned so that we can switch around easier

#[derive(Debug, PartialEq)]
pub struct OwnedVdf(pub Vec<OwnedPair>);

#[derive(Debug, PartialEq)]
pub struct OwnedPair(pub String, pub OwnedValue);

#[derive(Debug, PartialEq)]
pub enum OwnedValue {
    Str(String),
    Obj(Vec<OwnedPair>),
}

impl<'a> From<Vdf<'a>> for OwnedVdf {
    fn from(vdf: Vdf<'a>) -> Self {
        Self(vdf.inner().into_iter().map(OwnedPair::from).collect())
    }
}

impl<'a> From<&'a Pair<'a>> for OwnedPair {
    fn from(pair: &'a Pair) -> Self {
        OwnedPair(pair.0.to_string(), OwnedValue::from(&pair.1))
    }
}

impl<'a> From<&'a Value<'a>> for OwnedValue {
    fn from(value: &'a Value) -> Self {
        match value {
            Value::Str(s) => Self::Str(s.to_string()),
            Value::Obj(obj) => Self::Obj(obj.iter().map(OwnedPair::from).collect()),
        }
    }
}

impl Index<&str> for OwnedVdf {
    type Output = OwnedValue;

    fn index(&self, needle: &str) -> &Self::Output {
        &self
            .0
            .iter()
            .find(|OwnedPair(key, _)| needle == key)
            .unwrap()
            .1
    }
}

impl Index<&str> for OwnedPair {
    type Output = OwnedValue;

    fn index(&self, needle: &str) -> &Self::Output {
        if self.0 == needle {
            &self.1
        } else {
            unreachable!()
        }
    }
}

impl Index<&str> for OwnedValue {
    type Output = OwnedValue;

    fn index(&self, needle: &str) -> &Self::Output {
        if let Self::Obj(haystack) = self {
            &haystack
                .iter()
                .find(|OwnedPair(key, _)| needle == key)
                .unwrap()
                .1
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{fs, path::Path};

    #[test]
    fn extracting() {
        let sample_file = Path::new("tests").join("corpus").join("app_manifest.vdf");
        let unparsed = fs::read_to_string(&sample_file).unwrap();
        let vdf = Vdf::parse(&unparsed).unwrap();
        let owned = OwnedVdf::from(vdf);

        println!("thing: {:#?}", owned["AppState"]["buildid"]);
    }
}
