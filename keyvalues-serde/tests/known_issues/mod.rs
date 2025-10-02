use std::collections::BTreeMap;

use insta::assert_debug_snapshot;
use keyvalues_parser::{Obj, Value};
use keyvalues_serde::from_str;
use serde::Deserialize;

const FLATTEN_SINGLE: &str = "
top {
    common woo
    text str
}
";

const FLATTEN_SEQ: &str = "
top {
    common woo
    seq one
    seq {}
}
";

#[expect(dead_code)]
#[derive(Debug, Deserialize)]
struct FlattenObj<'text> {
    common: &'text str,
    #[serde(borrow, flatten)]
    obj: Obj<'text>,
}

#[expect(dead_code)]
#[derive(Debug, Deserialize)]
struct FlattenMap<'text> {
    common: &'text str,
    #[serde(borrow, flatten)]
    map: BTreeMap<String, Value<'text>>,
}

mod good {
    use super::*;

    #[test]
    fn flatten_obj() {
        let FlattenObj { common: _, obj } = from_str(FLATTEN_SEQ).unwrap();
        assert_debug_snapshot!(obj, @r#"
        Obj(
            {
                "seq": [
                    Str(
                        "one",
                    ),
                    Obj(
                        Obj(
                            {},
                        ),
                    ),
                ],
            },
        )
        "#);
    }

    /// an im-perfect workaround. you can explicitly use a map with a single value to flatten
    /// single values into, but this fails when there's multiple :(
    #[test]
    fn flatten_map() {
        let FlattenMap { common: _, map } = from_str(FLATTEN_SINGLE).unwrap();
        assert_debug_snapshot!(map, @r#"
        {
            "text": Str(
                "str",
            ),
        }
        "#);
    }
}

mod bad {
    use super::*;

    /// due to the ambiguous nature of anything being a sequence and `Obj` working through
    /// `.deserialize_any()`, we fail to `serde(flatten)` a single value into a `Vec<Value>>` (but
    /// it works fine if there are multiple values)
    #[test]
    fn flatten_obj() {
        from_str::<FlattenObj>(FLATTEN_SINGLE).expect_err("It works now!");
    }
}
