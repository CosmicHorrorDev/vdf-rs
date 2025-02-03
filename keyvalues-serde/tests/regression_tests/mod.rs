use std::collections::{BTreeMap, HashMap};

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

#[test]
fn flatten() {
    #[derive(Debug, Deserialize, Serialize)]
    struct Foo {
        a: i32,
        // This, otherwise innocuous looking, `flatten` changes this type to be treated as what it's
        // being flattened to. In this case from a struct to a map
        #[serde(flatten)]
        other: HashMap<String, String>,
    }

    let foo = Foo {
        a: 0,
        other: HashMap::new(),
    };
    let serialized = keyvalues_serde::to_string(&foo).unwrap();
    insta::assert_snapshot!(serialized);
}
