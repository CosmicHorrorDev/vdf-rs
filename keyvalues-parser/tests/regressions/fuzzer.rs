use keyvalues_parser::Vdf;

macro_rules! gen_fuzzer_tests {
    ( $test_fn:ident, $( ( $name:ident, $input:expr ) ),* $(,)? ) => {
        $(
            #[test]
            fn $name() {
                let vdf_text = $input;
                $test_fn(vdf_text)
            }
        )*
    };
}

mod valid {
    use super::*;

    // Mimics the behavior of the parse fuzzer test for regressions testing
    fn parse_valid(vdf_text: &str) {
        let parsed = Vdf::parse(vdf_text).expect("Input has to be valid here");
        let vdf_text = parsed.to_string();
        let reparsed = Vdf::parse(&vdf_text).expect("Roundtrip should still parse");
        pretty_assertions::assert_eq!(parsed, reparsed);
    }

    gen_fuzzer_tests!(
        parse_valid,
        (unqoted_backslash_key, r#"\ """#),
        (escaped_chars, r#""" "\r\\\n\t\"""#),
    );
}

mod invalid {
    use super::*;

    // Checks that we return an error instead of panicking or hanging
    fn parse_invalid(vdf_text: &str) {
        Vdf::parse(vdf_text).unwrap_err();
    }

    gen_fuzzer_tests!(
        parse_invalid,
        (empty, ""),
        (partial_map, "a{\n\"\""),
        (macrolike_key_then_map, "#basefoo{}"),
        (macrolike_key_then_str, "#base no_vdf"),
        (trailing_bytes, "foo {}\n\ntrailing bytes"),
    );

    fn error_invariants(vdf_text: &str) {
        let err = Vdf::parse(vdf_text).unwrap_err();

        // Lots of fiddly logic in displaying that can panic
        err.to_string();

        // The error snippet should match the original text sliced using the error span
        let from_orig = err.index_span().slice(vdf_text);
        let from_snippet = err.error_snippet();
        assert_eq!(from_orig, from_snippet);
    }

    gen_fuzzer_tests!(
        error_invariants,
        // FIXME: I think this needs to allow for the error line ending to be optional even when
        // the error ending isn't. The end of the line span should run up to EOI and trying to use
        // `RangeInclusive` to represent that runs into issues with multi-byte chars
        (newline_in_error_span_before_eoi, "\"\\\n"),
        (newline_as_invalid_escaped_char, "\"\\\n\""),
    );
}
