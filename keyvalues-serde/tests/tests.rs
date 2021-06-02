use insta::assert_snapshot;
use keyvalues_serde::{
    from_str, from_str_with_key, to_string, to_string_with_key, to_writer, to_writer_with_key,
};
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use tempdir::TempDir;

use std::{
    borrow::Cow,
    collections::HashMap,
    error::Error,
    fmt,
    fs::{self, File},
    path::Path,
};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Container<T> {
    inner: T,
}

impl<T> Container<T> {
    fn new(inner: T) -> Self {
        Self { inner }
    }
}

type BoxedResult<T> = Result<T, Box<dyn Error>>;

fn read_asset_file(file_name: &str) -> BoxedResult<String> {
    let val = fs::read_to_string(Path::new("tests").join("assets").join(file_name))?;
    Ok(val)
}

fn test_vdf_deserialization<'a, T>(vdf_text: &'a str, ideal_val: &T) -> BoxedResult<()>
where
    T: fmt::Debug + PartialEq + Deserialize<'a>,
{
    let deserialized_val: T = from_str(&vdf_text)?;
    assert_eq!(&deserialized_val, ideal_val, "Failed deserializing");
    Ok(())
}

fn test_vdf_serialization<T>(ideal_text: &str, val: &T) -> BoxedResult<()>
where
    T: fmt::Debug + PartialEq + Serialize,
{
    let val_text = to_string(val)?;
    assert_eq!(ideal_text, val_text, "Failed serializing");
    Ok(())
}

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

macro_rules! test_ser_de {
    ($func_name:ident, $test_val:expr, $file_name:literal) => {
        #[test]
        fn $func_name() -> BoxedResult<()> {
            let vdf_text = read_asset_file($file_name)?;
            test_vdf_deserialization(&vdf_text, &$test_val)?;
            test_vdf_serialization(&vdf_text, &$test_val)
        }
    };
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct TestStruct {
    field1: i32,
    field2: String,
}

test_ser_de!(
    basic_struct,
    TestStruct {
        field1: -123,
        field2: String::from("Sample String")
    },
    "basic_struct.vdf"
);

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct BasicTypes {
    boolean: bool,
    character: char,
    signed8: i8,
    signed16: i16,
    signed32: i32,
    signed64: i64,
    unsigned8: u8,
    unsigned16: u16,
    unsigned32: u32,
    unsigned64: u64,
    float32: f32,
    float64: f64,
}

test_ser_de!(
    basic_types,
    BasicTypes {
        boolean: false,
        character: 'a',
        signed8: 1,
        signed16: 2,
        signed32: 3,
        signed64: 4,
        unsigned8: 5,
        unsigned16: 6,
        unsigned32: 7,
        unsigned64: 8,
        float32: 1.0,
        float64: 2.0,
    },
    "basic_types.vdf"
);

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct OuterStruct {
    field: String,
    inner1: Container<String>,
    inner2: Container<String>,
}

test_ser_de!(
    nested_structs,
    OuterStruct {
        field: String::from("Outer Value"),
        inner1: Container::new(String::from("Inner1 Value")),
        inner2: Container::new(String::from("Inner2 Value")),
    },
    "nested_structs.vdf"
);

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct I32Wrapper(i32);

test_ser_de!(
    newtype_struct,
    Container::new(I32Wrapper(123)),
    "newtype_struct.vdf"
);

#[derive(Deserialize, Serialize, Debug, PartialEq)]
enum SampleEnum {
    Foo,
    Bar,
}

test_ser_de!(
    unit_variant_enum,
    Container::new(SampleEnum::Foo),
    "unit_variant_enum.vdf"
);

test_ser_de!(
    sequence_single,
    Container::new(vec![Container::new(String::from("Some String"))]),
    "sequence_single.vdf"
);

test_ser_de!(
    sequence_double,
    Container::new(vec![
        Container::new(String::from("Some String")),
        Container::new(String::from("Another String")),
    ]),
    "sequence_double.vdf"
);

test_ser_de!(
    tuple,
    Container::new((true, 2, String::from("Sample Text"))),
    "tuple.vdf"
);

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct TupleStruct(bool, i32, String);

test_ser_de!(
    tuple_struct,
    Container::new(TupleStruct(true, 2, String::from("Sample Text"))),
    "tuple.vdf"
);

// TODO: it's not clear if the ordering of values is expected to stay the same in vdf. If that is
// the case then it would be important to track down a map type that preserves insertion order. It
// looks like something like hashlink should work out
test_ser_de!(
    hashmap_nested,
    {
        let mut inner = HashMap::new();
        inner.insert(0, "Foo".to_owned());
        inner.insert(1, "Bar".to_owned());
        inner.insert(2, "Baz".to_owned());
        Container::new(inner)
    },
    "hashmap_nested.vdf"
);

test_ser_de!(
    option_none,
    Container::<Option<String>>::new(None),
    "option_none.vdf"
);

test_ser_de!(
    option_some,
    Container::new(Some(String::from("Some value"))),
    "option_some.vdf"
);
