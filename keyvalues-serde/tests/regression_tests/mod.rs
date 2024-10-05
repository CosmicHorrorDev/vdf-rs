use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

const ENUM_KEY: &str = r#"
Outer
{
    enum_keys
    {
        Foo {}
        Bar {}
    }
}
"#;

#[test]
fn enum_key() {
    #[derive(Debug, Deserialize, Serialize)]
    struct Outer {
        enum_keys: BTreeMap<Enum, Empty>,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
    enum Enum {
        Foo,
        Bar,
    }

    #[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
    struct Empty {}

    let outer: Outer = keyvalues_serde::from_str(ENUM_KEY).unwrap();
    let empty = Empty {};
    let expected = BTreeMap::from([(Enum::Foo, empty), (Enum::Bar, empty)]);
    assert_eq!(outer.enum_keys, expected);
}
