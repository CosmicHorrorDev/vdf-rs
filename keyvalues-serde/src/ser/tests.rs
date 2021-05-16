use serde::Serialize;

use std::collections::HashMap;

use crate::ser::to_string;

#[derive(Serialize, Debug, PartialEq)]
struct Container<T> {
    inner: T,
}

#[test]
fn basic_structure() {
    #[derive(Serialize, Debug, PartialEq)]
    struct TestStruct {
        field1: i32,
        field2: String,
    }

    let sample_struct = TestStruct {
        field1: -123,
        field2: String::from("Sample String"),
    };

    let expected_vdf = r#""TestStruct"
{
	"field1"	"-123"
	"field2"	"Sample String"
}
"#;

    assert_eq!(to_string(&sample_struct), Ok(expected_vdf.to_owned()));
}
