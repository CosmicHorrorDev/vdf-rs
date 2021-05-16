use serde::Deserialize;

use std::{collections::HashMap, error::Error, fmt, fs, path::Path};

use crate::de::from_str;

#[derive(Deserialize, Debug, PartialEq)]
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

fn test_vdf_deserialization<'a, T>(vdf_text: &'a str, ideal_val: T) -> BoxedResult<()>
where
    T: fmt::Debug + PartialEq + Deserialize<'a>,
{
    let deserialized_val: T = from_str(&vdf_text)?;
    assert_eq!(deserialized_val, ideal_val);
    Ok(())
}

#[test]
fn basic_struct() -> BoxedResult<()> {
    #[derive(Deserialize, Debug, PartialEq)]
    struct TestStruct {
        field1: i32,
        field2: String,
    }

    let ideal = TestStruct {
        field1: -123,
        field2: String::from("Sample String"),
    };
    test_vdf_deserialization(&read_asset_file("basic_struct.vdf")?, ideal)
}

#[test]
fn basic_types() -> BoxedResult<()> {
    #[derive(Deserialize, Debug, PartialEq)]
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

    let ideal = BasicTypes {
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
    test_vdf_deserialization(&read_asset_file("basic_types.vdf")?, ideal)
}

#[test]
fn nested_structs() -> BoxedResult<()> {
    #[derive(Deserialize, Debug, PartialEq)]
    struct OuterStruct {
        field: String,
        inner1: InnerStruct,
        inner2: InnerStruct,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct InnerStruct {
        field: String,
    }

    let ideal = OuterStruct {
        field: String::from("Outer Value"),
        inner1: InnerStruct {
            field: String::from("Inner1 Value"),
        },
        inner2: InnerStruct {
            field: String::from("Inner2 Value"),
        },
    };
    test_vdf_deserialization(&read_asset_file("nested_structs.vdf")?, ideal)
}

#[test]
fn newtype_struct() -> BoxedResult<()> {
    #[derive(Deserialize, Debug, PartialEq)]
    struct I32Wrapper(i32);

    let ideal = Container::new(I32Wrapper(123));
    test_vdf_deserialization(&read_asset_file("newtype_struct.vdf")?, ideal)
}

#[test]
fn unit_variant_enum() -> BoxedResult<()> {
    #[derive(Deserialize, Debug, PartialEq)]
    enum SampleEnum {
        Foo,
        Bar,
    }

    let ideal = Container::new(SampleEnum::Foo);
    test_vdf_deserialization(&read_asset_file("unit_variant_enum.vdf")?, ideal)
}

#[test]
fn sequence_single() -> BoxedResult<()> {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Inner {
        field: String,
    }

    let ideal = Container::new(vec![Inner {
        field: String::from("Some String"),
    }]);
    test_vdf_deserialization(&read_asset_file("sequence_single.vdf")?, ideal)
}

#[test]
fn sequence_double() -> BoxedResult<()> {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Inner {
        field: String,
    }

    let ideal = Container::new(vec![
        Inner {
            field: String::from("Some String"),
        },
        Inner {
            field: String::from("Another String"),
        },
    ]);
    test_vdf_deserialization(&read_asset_file("sequence_double.vdf")?, ideal)
}

#[test]
fn tuple() -> BoxedResult<()> {
    let ideal = Container::new((true, 2, String::from("Sample Text")));
    test_vdf_deserialization(&read_asset_file("tuple.vdf")?, ideal)
}

#[test]
fn tuple_struct() -> BoxedResult<()> {
    #[derive(Deserialize, Debug, PartialEq)]
    struct TupleStruct(bool, i32, String);

    let ideal = Container::new(TupleStruct(true, 2, String::from("Sample Text")));
    test_vdf_deserialization(&read_asset_file("tuple.vdf")?, ideal)
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
    let ideal = Container::new(inner);
    test_vdf_deserialization(&read_asset_file("hashmap_nested.vdf")?, ideal)
}

#[test]
fn hashmap_top_level() -> BoxedResult<()> {
    let mut ideal = HashMap::new();
    ideal.insert(0, "Foo".to_owned());
    ideal.insert(1, "Bar".to_owned());
    ideal.insert(2, "Baz".to_owned());
    test_vdf_deserialization(&read_asset_file("hashmap_top_level.vdf")?, ideal)
}

#[test]
fn option_none() -> BoxedResult<()> {
    let ideal: Container<Option<String>> = Container::new(None);
    test_vdf_deserialization(&read_asset_file("option_none.vdf")?, ideal)
}

#[test]
fn option_some() -> BoxedResult<()> {
    let ideal = Container::new(Some(String::from("Some value")));
    test_vdf_deserialization(&read_asset_file("option_some.vdf")?, ideal)
}
