use std::{collections::BTreeMap, io::Read};

use crate::core::{Value, Vdf};

pub type KeyBuf = String;
pub type KeyValuesBuf = BTreeMap<KeyBuf, Vec<ValueBuf>>;

// TODO: can Vdf and VdfBuf be combined by just using `Cow<'a, str>`?
#[derive(Debug, PartialEq, Default)]
pub struct VdfBuf(pub KeyValuesBuf);

impl VdfBuf {
    // No fancy streaming or anything like that yet. This just reads the full value in and then
    // parses it
    // TODO: fix error junk here
    pub fn from_reader(mut read: impl Read) -> Result<Self, Box<dyn std::error::Error>> {
        let mut buffer = String::new();
        read.read_to_string(&mut buffer)?;

        let vdf = Vdf::parse(&buffer)?;
        Ok(vdf.into())
    }

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

impl<'a> From<Vdf<'a>> for VdfBuf {
    fn from(vdf: Vdf) -> Self {
        vdf.into()
    }
}

impl<'a> From<&'a Vdf<'a>> for VdfBuf {
    fn from(vdf: &Vdf) -> Self {
        vdf.to_vdf_buf()
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
