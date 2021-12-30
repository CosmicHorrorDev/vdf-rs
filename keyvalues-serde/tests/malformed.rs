use keyvalues_serde::{error::Result, from_str};
use serde::Deserialize;

use std::collections::HashMap;

mod utils;

use crate::utils::{read_asset_file, BoxedResult, Container};

#[test]
fn str_when_wanting_obj() -> BoxedResult<()> {
    let vdf_text = read_asset_file("string_container.vdf")?;
    let result: Result<Container<HashMap<String, String>>> = from_str(&vdf_text);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn str_when_wanting_top_level_obj() -> BoxedResult<()> {
    let vdf_text = read_asset_file("top_level_string.vdf")?;
    let result: Result<Container<String>> = from_str(&vdf_text);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn obj_when_wanting_str() -> BoxedResult<()> {
    let vdf_text = read_asset_file("obj_container.vdf")?;
    let result: Result<Container<String>> = from_str(&vdf_text);
    assert!(result.is_err());
    Ok(())
}

#[test]
fn incorrect_seq_length() -> BoxedResult<()> {
    let vdf_len_one = read_asset_file("string_container.vdf")?;
    let len_two: Result<Container<(String, String)>> = from_str(&vdf_len_one);
    assert!(len_two.is_err());

    let vdf_len_two = read_asset_file("sequence_string_double.vdf")?;
    let len_one: Result<Container<(String,)>> = from_str(&vdf_len_two);
    assert!(len_one.is_err());
    let len_three: Result<Container<(String, String, String)>> = from_str(&vdf_len_two);
    assert!(len_three.is_err());
    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct Pair {
    pub first: String,
    pub second: String,
}

#[test]
fn wants_too_many_members() -> BoxedResult<()> {
    let vdf_text = read_asset_file("string_container.vdf")?;
    let result: Result<Pair> = from_str(&vdf_text);
    assert!(result.is_err());
    Ok(())
}
