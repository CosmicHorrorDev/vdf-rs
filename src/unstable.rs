use std::collections::BTreeMap;

use crate::common::{Value, Vdf};

// TODO: implement Borrow and ToOwned so that we can switch around easier

pub type Key = String;
pub type KeyValues = BTreeMap<Key, Vec<OwnedValue>>;

#[derive(Debug, PartialEq, Default)]
pub struct OwnedVdf(pub KeyValues);

#[derive(Debug, PartialEq)]
pub enum OwnedValue {
    Str(String),
    Obj(OwnedVdf),
}

impl<'a> From<Vdf<'a>> for OwnedVdf {
    fn from(vdf: Vdf<'a>) -> Self {
        Self::from(&vdf)
    }
}

impl<'a> From<&Vdf<'a>> for OwnedVdf {
    fn from(vdf: &Vdf<'a>) -> Self {
        let mut container = KeyValues::new();
        for (key, values) in vdf.inner().iter() {
            let owned_key = key.to_string();
            container.insert(owned_key, values.iter().map(OwnedValue::from).collect());
        }

        Self(container)
    }
}

impl<'a> From<Value<'a>> for OwnedValue {
    fn from(value: Value<'a>) -> Self {
        Self::from(&value)
    }
}

impl<'a> From<&Value<'a>> for OwnedValue {
    fn from(value: &Value<'a>) -> Self {
        match value {
            Value::Str(s) => Self::Str(s.to_string()),
            Value::Obj(obj) => Self::Obj(obj.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error::Error, fs, path::Path};

    type TestResult<T> = Result<T, Box<dyn Error>>;

    #[test]
    fn extracting() -> TestResult<()> {
        let sample_file = Path::new("tests").join("corpus").join("app_manifest.vdf");
        let owned = OwnedVdf::from(Vdf::parse(&fs::read_to_string(&sample_file)?)?);

        println!("thing: {:#?}", owned);
        panic!();

        Ok(())
    }
}
