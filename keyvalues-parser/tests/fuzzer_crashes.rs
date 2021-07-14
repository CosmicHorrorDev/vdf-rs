use keyvalues_parser::Vdf;
use pretty_assertions::assert_eq;

use std::{fs, path::Path};

type BoxedResult<T> = Result<T, Box<dyn std::error::Error>>;

// Mimics the behavior of the parse fuzzer test for regressions testing
fn parse_fuzz_test(file_name: &str) -> BoxedResult<()> {
    let crash_file = Path::new("tests").join("crash_outputs").join(file_name);
    let contents = fs::read_to_string(crash_file)?;

    // This should be infallible unless the grammar changes in which case the test is no longer
    // valid
    let parsed = Vdf::parse(&contents).expect("Input has to be valid here");
    let vdf_text = parsed.to_string();
    let reparsed = Vdf::parse(&vdf_text)?;
    assert_eq!(parsed, reparsed);

    Ok(())
}

// Generates a tests for each `name` that indicates both the test name and file name
macro_rules! parse_fuzzer_crash_infer_files {
    ( $( $name:ident ),* ) => {
        $(
            #[test]
            fn $name() -> BoxedResult<()> {
                parse_fuzz_test(stringify!($name))
            }
        )*
    };
}

parse_fuzzer_crash_infer_files!(crash_1, crash_2, crash_3);
