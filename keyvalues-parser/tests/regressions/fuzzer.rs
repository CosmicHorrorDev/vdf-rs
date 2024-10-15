use keyvalues_parser::Vdf;
use pretty_assertions::assert_eq;

type BoxedResult<T> = Result<T, Box<dyn std::error::Error>>;

// Mimics the behavior of the parse fuzzer test for regressions testing
fn parse_valid(contents: &str) -> BoxedResult<()> {
    let parsed = Vdf::parse(contents).expect("Input has to be valid here");
    let vdf_text = parsed.to_string();
    let reparsed = Vdf::parse(&vdf_text)?;
    assert_eq!(parsed, reparsed);

    Ok(())
}

// Checks that we return an error instead of panicking or hanging
fn parse_invalid(contents: &str) -> BoxedResult<()> {
    Vdf::parse(contents).unwrap_err();
    Ok(())
}

macro_rules! gen_fuzzer_tests {
    ( $test_fn:ident, $( ( $name:ident, $input:expr ) ),* $(,)? ) => {
        $(
            #[test]
            fn $name() -> BoxedResult<()> {
                let contents = $input;
                $test_fn(contents)
            }
        )*
    };
}

mod valid {
    use super::*;

    gen_fuzzer_tests!(
        parse_valid,
        (unqoted_backslash_key, r#"\ """#),
        (escaped_chars, r#""" "\r\\\n\t\"""#),
    );
}

mod invalid {
    use super::*;

    gen_fuzzer_tests!(
        parse_invalid,
        (empty, ""),
        (partial_map, "a{\n\"\""),
        (macrolike_key_then_map, "#basefoo{}"),
        (macrolike_key_then_str, "#base no_vdf"),
        (trailing_bytes, "foo {}\n\ntrailing bytes"),
    );
}
