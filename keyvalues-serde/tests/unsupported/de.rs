use crate::utils::Container;

use keyvalues_serde::{from_str, Error, Result};
use serde::Deserialize;

// It doesn't matter what the input text is, just that we match most of the structure other than
// the invalid type
const DUMMY_TEXT: &str = r#"
"Container"
{
    "inner"    "dummy"
}
"#;

macro_rules! check {
    ($res:expr, $msg:expr) => {
        println!("{:?}", $res);
        assert!($res.is_err());
        assert!(matches!($res, Err(Error::Unsupported($msg))))
    };
}

#[test]
fn bytes() {
    let bytes: Result<Container<&[u8]>> = from_str(DUMMY_TEXT);
    check!(bytes, "Bytes");
}

#[test]
fn unit() {
    let unit_type: Result<Container<()>> = from_str(DUMMY_TEXT);
    check!(unit_type, "Unit");
}

#[test]
fn unit_struct() {
    #[derive(Deserialize, Debug)]
    struct Unit;

    let unit_struct: Result<Container<Unit>> = from_str(DUMMY_TEXT);
    check!(unit_struct, "Unit Struct");
}
