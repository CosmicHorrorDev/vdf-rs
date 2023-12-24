use std::collections::BTreeMap;

use keyvalues_serde::{from_str, to_string, Error, Result};
use serde::Deserialize;

mod utils;

use utils::Container;

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

// It doesn't matter what the input text is, just that we match most of the structure other than
// the invalid type
#[test]
fn invalid_types() {
    let bytes: Result<Container<&[u8]>> = from_str(DUMMY_TEXT);
    check!(bytes, "Bytes");

    // TODO: how do we get serde to call `.deserialize_byte_buf()`

    let unit_type: Result<Container<()>> = from_str(DUMMY_TEXT);
    check!(unit_type, "Unit");

    #[derive(Deserialize, Debug)]
    struct Unit;

    let unit_struct: Result<Container<Unit>> = from_str(DUMMY_TEXT);
    check!(unit_struct, "Unit Struct");
}

#[test]
fn missing_top_level_key() {
    // TODO: clean up error type, so we can compare
    let _err = to_string(&BTreeMap::<(), ()>::new()).unwrap_err();
}
