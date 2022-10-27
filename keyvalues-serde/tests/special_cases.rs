use insta::{assert_debug_snapshot, assert_snapshot};
use keyvalues_serde::{
    from_str, from_str_with_key, to_string, to_string_with_key, to_writer, to_writer_with_key,
    Error,
};
use maplit::hashmap;
use pretty_assertions::assert_eq;
use serde::Deserialize;

use std::{borrow::Cow, collections::HashMap};

mod utils;

use utils::{read_asset_file, test_vdf_deserialization, BoxedResult, Container};

// TODO: what happens if you try to serialize a hashmap without providing a key?

#[test]
fn snapshot_writing() -> BoxedResult<()> {
    let name_base = "snapshot_writing";

    let vdf_struct = Container::new(123);
    let mut buf = Vec::new();

    // Write a vdf then verify it's correct
    to_writer(&mut buf, &vdf_struct)?;
    assert_snapshot!(
        format!("{}-to_writer", name_base),
        std::str::from_utf8(&buf)?
    );

    // And the same with a custom key
    buf.clear();
    to_writer_with_key(&mut buf, &vdf_struct, "Custom")?;
    assert_snapshot!(
        format!("{}-to_writer_with_key", name_base),
        std::str::from_utf8(&buf)?
    );

    Ok(())
}

#[test]
fn hashmap_top_level() -> BoxedResult<()> {
    let val = hashmap! {
        0 => "Foo",
        1 => "Bar",
        2 => "Baz",
    };
    let vdf_text = read_asset_file("hashmap_top_level.vdf")?;
    test_vdf_deserialization(&vdf_text, &val)?;

    // Using a hashmap on the top level has no way of indicating what the key should be so it must
    // be passed in separately
    let val_text = to_string_with_key(&val, "Key")?;
    assert_eq!(vdf_text, val_text, "Failed serializing");
    Ok(())
}

// Deserialization throws away the top level key, so `from_str_with_key` is needed to read it
#[test]
fn check_deserialization_key() -> BoxedResult<()> {
    let vdf_text = read_asset_file("hashmap_top_level.vdf")?;
    let (_, key): (HashMap<u64, String>, Cow<str>) = from_str_with_key(&vdf_text)?;

    assert_eq!(key, "Key", "Incorrect deserialization key");
    Ok(())
}

#[test]
fn non_finite_float_serialization_failure() {
    let vdf = Container::new(std::f32::NAN);
    if let Err(Error::NonFiniteFloat(f)) = to_string(&vdf) {
        assert!(f.is_nan());
    } else {
        unreachable!("Serialization should fail with NaN float");
    }
}

#[test]
fn non_finite_float_deserialization_failure() -> BoxedResult<()> {
    let vdf_text = read_asset_file("subnormal_float.vdf")?;
    if let Err(Error::NonFiniteFloat(f)) = from_str::<Container<f32>>(&vdf_text) {
        assert!(f.is_infinite());
    } else {
        unreachable!("Deserialization should fail with inf float");
    }

    Ok(())
}

#[test]
fn non_normal_but_finite_float_serialization() -> BoxedResult<()> {
    let vdf_text = read_asset_file("zero_float.vdf")?;
    let vdf: Container<f32> = from_str(&vdf_text)?;

    assert_eq!(vdf, Container::new(0.0f32));
    Ok(())
}

#[test]
fn extract_only_some_members() -> BoxedResult<()> {
    let vdf_text = read_asset_file("multiple_members.vdf")?;
    let vdf: Container<String> = from_str(&vdf_text)?;

    assert_eq!(vdf, Container::new(String::from("Value")));
    Ok(())
}

// Flatten infers values by the structure so this tests the use of all the `.deserialize_any()`
// variants
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct AnyHolder {
    #[serde(flatten)]
    foo: StructureDefinedType,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct StructureDefinedType {
    inner: String,
    #[serde(rename = "str key")]
    str_key: String,
    #[serde(rename = "obj key")]
    obj_key: HashMap<String, String>,
    #[serde(rename = "seq key")]
    seq_key: Vec<String>,
}

#[test]
fn deserialize_any_values() -> BoxedResult<()> {
    let vdf_text = read_asset_file("multiple_members.vdf")?;
    let any_holder: AnyHolder = from_str(&vdf_text)?;

    assert_debug_snapshot!("deserialize_any_values", any_holder);
    Ok(())
}

#[test]
fn borrowed_escaped_string() -> BoxedResult<()> {
    let vdf_text = read_asset_file("escaped_string.vdf")?;
    let vdf: Container<Cow<str>> = from_str(&vdf_text)?;

    assert_eq!(vdf, Container::new(Cow::from("tab\tseparated")));
    Ok(())
}

#[derive(Deserialize, Debug, PartialEq)]
struct BorrowedString<'a> {
    #[serde(borrow)]
    inner: Cow<'a, str>,
}

#[test]
fn borrowed_string_is_borrowed() -> BoxedResult<()> {
    let vdf_text = read_asset_file("string_container.vdf")?;
    let vdf: BorrowedString = from_str(&vdf_text)?;

    assert!(matches!(vdf.inner, Cow::Borrowed(_)));
    Ok(())
}
