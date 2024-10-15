use keyvalues_serde::{from_str, to_string};
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};

use std::{fmt, fs, path::Path};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Container<T> {
    inner: T,
}

impl<T> Container<T> {
    #[allow(dead_code)]
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

pub type BoxedResult<T> = Result<T, Box<dyn std::error::Error>>;

#[allow(dead_code)]
pub fn read_asset_file(file_name: &str) -> BoxedResult<String> {
    let val = fs::read_to_string(Path::new("tests").join("assets").join(file_name))?;
    Ok(val)
}

#[allow(dead_code)]
pub fn test_vdf_deserialization<'a, T>(vdf_text: &'a str, ideal_val: &T) -> BoxedResult<()>
where
    T: fmt::Debug + PartialEq + Deserialize<'a>,
{
    let deserialized_val: T = from_str(vdf_text)?;
    assert_eq!(&deserialized_val, ideal_val, "Failed deserializing");
    Ok(())
}

// I'm too tired to be able to wrap my head around why just this one function is causing trouble
#[allow(dead_code)]
pub fn test_vdf_serialization<T>(ideal_text: &str, val: &T) -> BoxedResult<()>
where
    T: fmt::Debug + PartialEq + Serialize,
{
    let val_text = to_string(val)?;
    assert_eq!(ideal_text, val_text, "Failed serializing");
    Ok(())
}
