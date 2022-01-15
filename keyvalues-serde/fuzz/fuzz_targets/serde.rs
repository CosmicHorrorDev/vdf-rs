#![no_main]
use arbitrary::Arbitrary;
use keyvalues_serde::{from_str, to_string};
use libfuzzer_sys::fuzz_target;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Arbitrary, Deserialize, Serialize)]
struct KitchenSink {
    boolean: bool,
    character: char,
    // TODO: If f64 really isn't supported then it would make sense to remove it instead of having
    // people be confused about why f64 values are subtly changing
    // f64 isn't included since it just gets represented as an f32
    float32: f32,
    signed08: i8,
    signed16: i16,
    signed32: i32,
    signed64: i64,
    signed128: i128,
    unsigned08: u8,
    unsigned16: u16,
    unsigned32: u32,
    unsigned64: u64,
    unsigned128: i128,
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
    // TODO: might want to manually implement arbitrary, but non_finite floats aren't allowed and
    // will cause the conversion to fail
    if let Ok(vdf_text) = to_string(&initial) {
        let reparsed = from_str(&vdf_text).unwrap();

        assert_eq!(initial, reparsed);
    }
});
