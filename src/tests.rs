use once_cell::sync::Lazy;

use std::{fs, path::Path};

use crate::common::{Pair, Value, Vdf};

static CONTROLLER_MAPPINGS_VDF: Lazy<Vdf> = Lazy::new(|| {
    Vdf(vec![Pair(
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
    )])
});

static NICHE_VDF: Lazy<Vdf> = Lazy::new(|| {
    Vdf(vec![
        Pair("#base", Value::Str("file")),
        Pair(
            "Outer Key",
            Value::Obj(vec![Pair("Inner Key", Value::Str("Value"))]),
        ),
    ])
});

#[test]
fn basic_deserialization() {
    let sample_file = Path::new("tests")
        .join("samples")
        .join("controller_mappings.vdf");
    let unparsed = fs::read_to_string(&sample_file).unwrap();
    let vdf = Vdf::parse(&unparsed).unwrap();

    println!("Vdf: {:#?}", vdf);
    println!("Ideal: {:#?}", CONTROLLER_MAPPINGS_VDF);
    assert_eq!(vdf, *CONTROLLER_MAPPINGS_VDF);
}

#[test]
fn basic_serialization() {
    let sample_file = Path::new("tests")
        .join("samples")
        .join("controller_mappings.vdf");
    let unparsed = fs::read_to_string(&sample_file).unwrap();
    let serialized = CONTROLLER_MAPPINGS_VDF.to_string();

    assert_eq!(unparsed, serialized);
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

#[test]
fn multiple_outer_pairs_serialization() {
    let sample_file = Path::new("tests").join("samples").join("niche.vdf");
    let unparsed = fs::read_to_string(&sample_file).unwrap();
    let vdf = Vdf::parse(&unparsed).unwrap();

    assert_eq!(vdf, *NICHE_VDF);
}
