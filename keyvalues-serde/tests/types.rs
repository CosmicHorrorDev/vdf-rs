use serde::{Deserialize, Serialize};

use std::collections::HashMap;

mod utils;

use utils::{
    read_asset_file, test_vdf_deserialization, test_vdf_serialization, BoxedResult, Container,
};

// Helper that generates a test to ensure that the contents within `file_name` deserialize to
// `test_val` and vice-versa with serialization
macro_rules! test_ser_de {
    ($func_name:ident, $test_val:expr, $file_name:expr) => {
        #[test]
        fn $func_name() -> BoxedResult<()> {
            let vdf_text = read_asset_file($file_name)?;
            test_vdf_deserialization(&vdf_text, &$test_val)?;
            test_vdf_serialization(&vdf_text, &$test_val)
        }
    };
}

// Calls `test_ser_de` but generates the filename from the func_name for the common case
macro_rules! test_ser_de_infer_file {
    ($func_name:ident, $test_val:expr) => {
        test_ser_de!(
            $func_name,
            $test_val,
            &format!("{}.vdf", stringify!($func_name))
        );
    };
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct TestStruct {
    field1: i32,
    field2: String,
}

test_ser_de_infer_file!(
    basic_struct,
    TestStruct {
        field1: -123,
        field2: String::from("Sample String")
    }
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

test_ser_de_infer_file!(
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
    }
);

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct OuterStruct {
    field: String,
    inner1: Container<String>,
    inner2: Container<String>,
}

test_ser_de_infer_file!(
    nested_structs,
    OuterStruct {
        field: String::from("Outer Value"),
        inner1: Container::new(String::from("Inner1 Value")),
        inner2: Container::new(String::from("Inner2 Value")),
    }
);

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct I32Wrapper(i32);

test_ser_de_infer_file!(newtype_struct, Container::new(I32Wrapper(123)));

#[derive(Deserialize, Serialize, Debug, PartialEq)]
enum SampleEnum {
    Foo,
    Bar,
}

test_ser_de_infer_file!(unit_variant_enum, Container::new(SampleEnum::Foo));

test_ser_de_infer_file!(
    sequence_single,
    Container::new(vec![Container::new(String::from("Some String"))])
);

test_ser_de_infer_file!(
    sequence_double,
    Container::new(vec![
        Container::new(String::from("Some String")),
        Container::new(String::from("Another String")),
    ])
);

test_ser_de_infer_file!(
    tuple,
    Container::new((true, 2, String::from("Sample Text")))
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
test_ser_de_infer_file!(hashmap_nested, {
    let mut inner = HashMap::new();
    inner.insert(0, "Foo".to_owned());
    inner.insert(1, "Bar".to_owned());
    inner.insert(2, "Baz".to_owned());
    Container::new(inner)
});

test_ser_de_infer_file!(option_none, Container::<Option<String>>::new(None));

test_ser_de_infer_file!(
    option_some,
    Container::new(Some(String::from("Some value")))
);