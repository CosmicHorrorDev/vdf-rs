use once_cell::sync::Lazy;

use std::{convert::TryFrom, fs, path::Path};

use crate::common::{Vdf, VdfPair, VdfValue};

static SAMPLE_VDF: Lazy<Vdf> = Lazy::new(|| {
    Vdf(VdfPair(
        "controller_mappings",
        VdfValue::Obj(vec![
            VdfPair("version", VdfValue::Str("2")),
            VdfPair(
                "group",
                VdfValue::Obj(vec![VdfPair("mode", VdfValue::Str("four_buttons"))]),
            ),
            VdfPair(
                "group",
                VdfValue::Obj(vec![VdfPair(
                    "settings",
                    VdfValue::Obj(vec![VdfPair("requires_click", VdfValue::Str("0"))]),
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
    let vdf = Vdf::try_from(unparsed.as_str()).unwrap();

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
    let vdf = Vdf::try_from(unparsed.as_str()).unwrap();
    assert_eq!(unparsed, vdf.to_string());
}
