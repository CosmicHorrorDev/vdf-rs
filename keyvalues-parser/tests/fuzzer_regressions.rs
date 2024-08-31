use keyvalues_parser::Vdf;
use pretty_assertions::assert_eq;

type BoxedResult<T> = Result<T, Box<dyn std::error::Error>>;

// Mimics the behavior of the parse fuzzer test for regressions testing
fn parse_valid(contents: &str) -> BoxedResult<()> {
    let parsed = Vdf::parse(&contents).expect("Input has to be valid here");
    let vdf_text = parsed.to_string();
    let reparsed = Vdf::parse(&vdf_text)?;
    assert_eq!(parsed, reparsed);

    Ok(())
}

// Checks that we return an error instead of panicking or hanging
fn parse_invalid(contents: &str) -> BoxedResult<()> {
    Vdf::parse(&contents).unwrap_err();
    Ok(())
}

macro_rules! gen_fuzzer_tests {
    ( $test_fn:ident, $( $name:ident ),* ) => {
        $(
            #[test]
            fn $name() -> BoxedResult<()> {
                let contents = include_str!(
                    concat!("fuzzer_regression_assets/", stringify!($name))
                );
                $test_fn(contents)
            }
        )*
    };
}

gen_fuzzer_tests!(parse_valid, valid_1, valid_2, valid_3);
gen_fuzzer_tests!(
    parse_invalid,
    invalid_1,
    invalid_2,
    invalid_3,
    invalid_4,
    invalid_5
);
