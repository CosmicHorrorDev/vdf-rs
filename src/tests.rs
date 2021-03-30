use once_cell::sync::Lazy;

use std::{error::Error, fs, path::Path};

use crate::common::{Pair, Value, Vdf};

type TestResult<T> = Result<T, Box<dyn Error>>;

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

fn read_corpus_file(file_name: &str) -> TestResult<String> {
    let corpus_file_path = Path::new("tests").join("corpus").join(file_name);
    let contents = fs::read_to_string(&corpus_file_path)?;
    Ok(contents)
}

#[test]
fn basic_deserialization() -> TestResult<()> {
    let unparsed = read_corpus_file("controller_mappings.vdf")?;
    let vdf = Vdf::parse(&unparsed)?;
    assert_eq!(vdf, *CONTROLLER_MAPPINGS_VDF);

    Ok(())
}

#[test]
fn basic_serialization() -> TestResult<()> {
    let unparsed = read_corpus_file("controller_mappings.vdf")?;
    let serialized = CONTROLLER_MAPPINGS_VDF.to_string();
    assert_eq!(unparsed, serialized);

    Ok(())
}

// Serialization isn't guaranteed to be exactly the same as the origin text, but it should have
// equivalent meaning
#[test]
fn round_trip() -> TestResult<()> {
    let unparsed = read_corpus_file("app_manifest.vdf")?;
    let vdf = Vdf::parse(&unparsed)?;
    assert_eq!(unparsed, vdf.to_string());

    Ok(())
}

#[test]
fn multiple_outer_pairs_serialization() -> TestResult<()> {
    let unparsed = read_corpus_file("niche.vdf")?;
    let vdf = Vdf::parse(&unparsed)?;
    assert_eq!(vdf, *NICHE_VDF);

    Ok(())
}
