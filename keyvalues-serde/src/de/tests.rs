use serde::Deserialize;

use std::collections::HashMap;

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

#[test]
fn basic_struct() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct TestStruct {
        field1: i32,
        field2: String,
    }

    let s = r#"
"TestStruct"
{
    "field1" "-123"
    "field2" "Sample String"
}
        "#;

    let sample: TestStruct = from_str(s).unwrap();
    assert_eq!(
        sample,
        TestStruct {
            field1: -123,
            field2: String::from("Sample String")
        }
    )
}

#[test]
fn basic_types() {
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

    let s = r#"
"Key"
{
    "boolean" "0"
    "character" "a"
    "signed8" "1"
    "signed16" "2"
    "signed32" "3"
    "signed64" "4"
    "unsigned8" "5"
    "unsigned16" "6"
    "unsigned32" "7"
    "unsigned64" "8"
    "float32" "1.0"
    "float64" "2.0"
}
        "#;

    let sample: BasicTypes = from_str(s).unwrap();
    assert_eq!(
        sample,
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
            float64: 2.0
        }
    );
}

#[test]
fn nested_structs() {
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

    let s = r#"
"OuterStruct"
{
    "field" "Outer Value"
    "inner1"
    {
        "field" "Inner1 Value"
    }
    "inner2"
    {
        "field" "Inner2 Value"
    }
}
        "#;

    let sample: OuterStruct = from_str(s).unwrap();
    assert_eq!(
        sample,
        OuterStruct {
            field: String::from("Outer Value"),
            inner1: InnerStruct {
                field: String::from("Inner1 Value"),
            },
            inner2: InnerStruct {
                field: String::from("Inner2 Value"),
            }
        },
    );
}

#[test]
fn newtype_struct() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct I32Wrapper(i32);

    let s = r#"
"Container"
{
    "inner" "123"
}
        "#;

    let sample: Container<I32Wrapper> = from_str(s).unwrap();
    assert_eq!(sample, Container::new(I32Wrapper(123)));
}

#[test]
fn unit_variant_enum() {
    #[derive(Deserialize, Debug, PartialEq)]
    enum SampleEnum {
        Foo,
        Bar,
    }

    let s = r#"
"Key"
{
    "inner" "Foo"
}
        "#;
    let sample: Container<SampleEnum> = from_str(s).unwrap();
    assert_eq!(sample, Container::new(SampleEnum::Foo));
}

#[test]
fn sequence() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Inner {
        field: String,
    }

    let single_str = r#"
"Key"
{
    "inner"
    {
        "field" "Some String"
    }
}
        "#;

    let single: Container<Vec<Inner>> = from_str(single_str).unwrap();
    assert_eq!(
        single,
        Container::new(vec![Inner {
            field: String::from("Some String")
        }])
    );

    let double_str = r#"
"Key"
{
    "inner"
    {
        "field" "Some String"
    }
    "inner"
    {
        "field" "Another String"
    }
}
        "#;

    let double: Container<Vec<Inner>> = from_str(double_str).unwrap();
    assert_eq!(
        double,
        Container::new(vec![
            Inner {
                field: String::from("Some String")
            },
            Inner {
                field: String::from("Another String")
            }
        ])
    );
}

#[test]
fn tuple() {
    let s = r#"
"Key"
{
    "inner" "1"
    "inner" "2"
    "inner" "Sample Text"
}
        "#;

    let sample: Container<(bool, i32, String)> = from_str(s).unwrap();
    assert_eq!(
        sample,
        Container::new((true, 2, String::from("Sample Text")))
    );
}

#[test]
fn tuple_struct() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct TupleStruct(bool, i32, String);

    let s = r#"
"Key"
{
    "inner" "1"
    "inner" "2"
    "inner" "Sample Text"
}
        "#;

    let sample: Container<TupleStruct> = from_str(s).unwrap();
    assert_eq!(
        sample,
        Container::new(TupleStruct(true, 2, String::from("Sample Text"),))
    )
}

// TODO: it's not clear if the ordering of values is expected to stay the
// same in vdf. If that is the case then it would be important to track
// down a map type that preserves insertion order
#[test]
fn hashmap() {
    let nested = r#"
"Key"
{
    "inner"
    {
        "0" "Foo"
        "1" "Bar"
        "2" "Baz"
    }
}
        "#;

    let mut ideal = HashMap::new();
    ideal.insert(0, "Foo".to_owned());
    ideal.insert(1, "Bar".to_owned());
    ideal.insert(2, "Baz".to_owned());

    let sample: Container<HashMap<u64, String>> = from_str(nested).unwrap();
    assert_eq!(sample, Container::new(ideal.clone()));

    let top_level = r#"
"Key"
{
    "0" "Foo"
    "1" "Bar"
    "2" "Baz"
}
        "#;

    let sample: HashMap<u64, String> = from_str(top_level).unwrap();
    assert_eq!(sample, ideal);
}

#[test]
fn option() {
    let none_str = r#"
"Key"
{
}
        "#;

    let none: Container<Option<String>> = from_str(none_str).unwrap();
    assert_eq!(none, Container::new(None));

    let some_str = r#"
"Key"
{
    "inner" "Some value"
}
        "#;

    let some: Container<Option<String>> = from_str(some_str).unwrap();
    assert_eq!(some, Container::new(Some(String::from("Some value"))));
}
