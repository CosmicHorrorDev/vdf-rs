use keyvalues_parser::Value;

const TEXT: &str = r"
circus_of_values {
    text woo
    obj { foo {} }
    seq one
    seq {}
}
";

// TODO: Limitations...
// - Empty `Vec`'s require a `serde(default)` to avoid an error
// - Flattening into an `Obj` has issues unless the field is a sequnce of length >1 (this is not
//   the case for a `Vec<Value>` field). This is probably from it going through `deserialize_any()`
//   which doesn't know when a string is a part of a sequence
#[test]
fn value_variety() {
    #[derive(serde::Deserialize, Debug)]
    #[expect(dead_code)]
    struct LooselyTyped<'text> {
        #[serde(borrow)]
        text: Value<'text>,
        #[serde(borrow)]
        obj: Value<'text>,
        #[serde(borrow, default)]
        seq: Vec<Value<'text>>,
    }

    let loosely_typed: LooselyTyped = keyvalues_serde::from_str(TEXT).unwrap();
    insta::assert_debug_snapshot!(loosely_typed, @r#"
    LooselyTyped {
        text: Str(
            "woo",
        ),
        obj: Obj(
            Obj(
                {
                    "foo": [
                        Obj(
                            Obj(
                                {},
                            ),
                        ),
                    ],
                },
            ),
        ),
        seq: [
            Str(
                "one",
            ),
            Obj(
                Obj(
                    {},
                ),
            ),
        ],
    }
    "#);
}
