use std::collections::BTreeMap;

pub type KeyBuf = String;
pub type KeyValuesBuf = BTreeMap<KeyBuf, Vec<ValueBuf>>;

#[derive(Debug, PartialEq, Default)]
pub struct VdfBuf(pub KeyValuesBuf);

#[derive(Debug, PartialEq)]
pub enum ValueBuf {
    Str(String),
    Obj(VdfBuf),
}
