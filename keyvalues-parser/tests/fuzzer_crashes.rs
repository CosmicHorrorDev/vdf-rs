use keyvalues_parser::Vdf;
use pretty_assertions::assert_eq;

use std::{fs, path::Path};

type BoxedResult<T> = Result<T, Box<dyn std::error::Error>>;

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

macro_rules! parse_fuzz_regression_test {
    ($func_name:ident, $file_name:literal) => {
        #[test]
        fn $func_name() -> BoxedResult<()> {
            parse_fuzz_test($file_name)
        }
    };
}

parse_fuzz_regression_test!(fuzzer_crash_1, "crash-1");
parse_fuzz_regression_test!(fuzzer_crash_2, "crash-2");
parse_fuzz_regression_test!(fuzzer_crash_3, "crash-3");
