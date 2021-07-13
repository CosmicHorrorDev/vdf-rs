use insta::assert_snapshot;
use keyvalues_serde::{
    from_str, from_str_with_key, to_string, to_string_with_key, to_writer, to_writer_with_key,
    Error,
};
use pretty_assertions::assert_eq;
use tempdir::TempDir;

use std::{
    borrow::Cow,
    collections::HashMap,
    fs::{self, File},
};

mod utils;

use utils::{read_asset_file, test_vdf_deserialization, BoxedResult, Container};

// TODO: what happens if you try to serialize a hashmap without providing a key?

#[test]
fn snapshot_writing_to_file() -> BoxedResult<()> {
    let vdf_struct = Container::new(123);
    let dir = TempDir::new("keyvalues-serde")?;
    let file_path = dir.path().join("sample.vdf");

    // Write a vdf to a file then verify it's correct
    let mut file = File::create(&file_path)?;
    to_writer(&mut file, &vdf_struct)?;
    let vdf_text = fs::read_to_string(&file_path)?;
    assert_snapshot!(vdf_text);

    // And the same with a custom key
    let mut file = File::create(&file_path)?;
    to_writer_with_key(&mut file, &vdf_struct, "Custom")?;
    let vdf_text = fs::read_to_string(&file_path)?;
    assert_snapshot!(vdf_text);

    Ok(())
}

#[test]
fn hashmap_top_level() -> BoxedResult<()> {
    let mut val = HashMap::new();
    val.insert(0, "Foo".to_owned());
    val.insert(1, "Bar".to_owned());
    val.insert(2, "Baz".to_owned());
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
        panic!("Serialization should fail with NaN float");
    }
}

#[test]
fn non_finite_float_deserialization_failure() -> BoxedResult<()> {
    let vdf_text = read_asset_file("subnormal_float.vdf")?;
    if let Err(Error::NonFiniteFloat(f)) = from_str::<Container<f32>>(&vdf_text) {
        assert!(f.is_infinite());
    } else {
        panic!("Deserialization should fail with inf float");
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