use keyvalues_parser::core::Vdf;
use pretty_assertions::assert_eq;

use std::{fs, path::Path};

fn read_crash_file(file_name: &str) -> String {
    let crash_file = Path::new("tests").join("crash_outputs").join(file_name);
    fs::read_to_string(crash_file).unwrap()
}

type BoxedResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn fuzz_failure_1() -> BoxedResult<()> {
    let contents = read_crash_file("crash-1");

    let parsed = Vdf::parse(&contents).unwrap();
    let vdf_text = parsed.to_string();
    let reparsed = Vdf::parse(&vdf_text)?;
    assert_eq!(parsed, reparsed);

    Ok(())
}
