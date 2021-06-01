use keyvalues_serde::{from_str, from_str_with_key, to_string, to_string_with_key};
use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};

use std::{borrow::Cow, collections::HashMap, error::Error, fmt, fs, path::Path};

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

macro_rules! test_ser_de {
    ($test_val:ident, $file_name:literal) => {{
        let vdf_text = read_asset_file($file_name)?;
        test_vdf_deserialization(&vdf_text, &$test_val)?;
        test_vdf_serialization(&vdf_text, &$test_val)
    }};
}

#[test]
fn basic_struct() -> BoxedResult<()> {
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct TestStruct {
        field1: i32,
        field2: String,
    }

    let val = TestStruct {
        field1: -123,
        field2: String::from("Sample String"),
    };
    test_ser_de!(val, "basic_struct.vdf")
}

#[test]
fn basic_types() -> BoxedResult<()> {
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

    let val = BasicTypes {
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
    };
    // TODO: fields get serializaed in alphabetical order so make sure that is fine
    test_ser_de!(val, "basic_types.vdf")
}

#[test]
fn nested_structs() -> BoxedResult<()> {
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct OuterStruct {
        field: String,
        inner1: Container<String>,
        inner2: Container<String>,
    }

    let val = OuterStruct {
        field: String::from("Outer Value"),
        inner1: Container::new(String::from("Inner1 Value")),
        inner2: Container::new(String::from("Inner2 Value")),
    };
    test_ser_de!(val, "nested_structs.vdf")
}

#[test]
fn newtype_struct() -> BoxedResult<()> {
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct I32Wrapper(i32);

    let val = Container::new(I32Wrapper(123));
    test_ser_de!(val, "newtype_struct.vdf")
}

#[test]
fn unit_variant_enum() -> BoxedResult<()> {
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    enum SampleEnum {
        Foo,
        Bar,
    }

    let val = Container::new(SampleEnum::Foo);
    test_ser_de!(val, "unit_variant_enum.vdf")
}

#[test]
fn sequence_single() -> BoxedResult<()> {
    let val = Container::new(vec![Container::new(String::from("Some String"))]);
    test_ser_de!(val, "sequence_single.vdf")
}

#[test]
fn sequence_double() -> BoxedResult<()> {
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct Inner {
        field: String,
    }

    let val = Container::new(vec![
        Container::new(String::from("Some String")),
        Container::new(String::from("Another String")),
    ]);
    test_ser_de!(val, "sequence_double.vdf")
}

#[test]
fn tuple() -> BoxedResult<()> {
    let val = Container::new((true, 2, String::from("Sample Text")));
    test_ser_de!(val, "tuple.vdf")
}

#[test]
fn tuple_struct() -> BoxedResult<()> {
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct TupleStruct(bool, i32, String);

    let val = Container::new(TupleStruct(true, 2, String::from("Sample Text")));
    test_ser_de!(val, "tuple.vdf")
}

// TODO: it's not clear if the ordering of values is expected to stay the same in vdf. If that is
// the case then it would be important to track down a map type that preserves insertion order. It
// looks like something like hashlink should work out
#[test]
fn hashmap_nested() -> BoxedResult<()> {
    let mut inner = HashMap::new();
    inner.insert(0, "Foo".to_owned());
    inner.insert(1, "Bar".to_owned());
    inner.insert(2, "Baz".to_owned());
    let val = Container::new(inner);
    test_ser_de!(val, "hashmap_nested.vdf")
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

#[test]
fn check_deserialization_key() -> BoxedResult<()> {
    let vdf_text = read_asset_file("hashmap_top_level.vdf")?;
    let (_, key): (HashMap<u64, String>, Cow<str>) = from_str_with_key(&vdf_text)?;

    assert_eq!(key, "Key", "Incorrect deserialization key");
    Ok(())
}

#[test]
fn option_none() -> BoxedResult<()> {
    let val: Container<Option<String>> = Container::new(None);
    test_ser_de!(val, "option_none.vdf")
}

#[test]
fn option_some() -> BoxedResult<()> {
    let val = Container::new(Some(String::from("Some value")));
    test_ser_de!(val, "option_some.vdf")
}
