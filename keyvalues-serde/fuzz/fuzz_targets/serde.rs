#![no_main]
use arbitrary::Arbitrary;
use keyvalues_serde::{from_str, to_string};
use libfuzzer_sys::fuzz_target;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Arbitrary, Deserialize, Serialize)]
struct KitchenSink {
    boolean: bool,
    character: char,
    float32: f32,
    float64: f64,
    signed08: i8,
    signed16: i16,
    signed32: i32,
    signed64: i64,
    unsigned08: u8,
    unsigned16: u16,
    unsigned32: u32,
    unsigned64: u64,
    // TODO: make a note about this
    #[serde(default)]
    vec: Vec<bool>,
    optional: Option<u32>,
    inner_struct: InnerStruct,
    inner_enum: InnerEnum,
    inner_tuple_struct: InnerTupleStruct,
}

#[derive(Debug, PartialEq, Arbitrary, Deserialize, Serialize)]
struct InnerStruct {
    field: String,
}

#[derive(Debug, PartialEq, Arbitrary, Deserialize, Serialize)]
enum InnerEnum {
    Foo,
    Bar,
    Baz,
}

#[derive(Debug, PartialEq, Arbitrary, Deserialize, Serialize)]
struct InnerTupleStruct(bool, i32, String);

fuzz_target!(|initial: KitchenSink| {
    // TODO: make this error
    // Only normal real numbers are allowed
    if initial.float32.is_normal() && initial.float64.is_normal() {
        let vdf_text = to_string(&initial).unwrap();
        let reparsed = from_str(&vdf_text).unwrap();

        assert_eq!(initial, reparsed,);
    }
});
