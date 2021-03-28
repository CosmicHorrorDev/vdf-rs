use once_cell::sync::Lazy;

use std::{fs, path::Path};

use crate::common::{Pair, Value, Vdf};

static SAMPLE_VDF: Lazy<Vdf> = Lazy::new(|| {
    Vdf(Pair(
        "controller_mappings",
        Value::Obj(vec![
            Pair("version", Value::Str("2")),
            Pair(
                "group",
                Value::Obj(vec![Pair("mode", Value::Str("four_buttons"))]),
            ),
            Pair(
                "group",
                Value::Obj(vec![Pair(
                    "settings",
                    Value::Obj(vec![Pair("requires_click", Value::Str("0"))]),
                )]),
            ),
        ]),
    ))
});

#[test]
fn deserialization() {
    let sample_file = Path::new("tests")
        .join("samples")
        .join("controller_mappings.vdf");
    let unparsed = fs::read_to_string(&sample_file).unwrap();
    let vdf = Vdf::parse(&unparsed).unwrap();

    println!("Vdf: {:#?}", vdf);
    println!("Ideal: {:#?}", SAMPLE_VDF);
    assert_eq!(vdf, *SAMPLE_VDF);
}

#[test]
fn serialization() {
    let sample_file = Path::new("tests")
        .join("samples")
        .join("controller_mappings.vdf");
    let unparsed = fs::read_to_string(&sample_file).unwrap();

    assert_eq!(unparsed, *SAMPLE_VDF.to_string());
}

// Serialization isn't guaranteed to be exactly the same as the origin text, but it should have
// equivalent meaning
#[test]
fn round_trip() {
    let sample_file = Path::new("tests").join("samples").join("app_manifest.vdf");
    let unparsed = fs::read_to_string(&sample_file).unwrap();
    let vdf = Vdf::parse(&unparsed).unwrap();
    assert_eq!(unparsed, vdf.to_string());
}
