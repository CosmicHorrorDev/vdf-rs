#![no_main]

use keyvalues_parser::Vdf;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|text: &str| {
    if let Err(err) = Vdf::parse(text) {
        // Lots of fiddly logic in displaying that can panic
        err.to_string();

        // The error snippet should match the original text sliced using the error span
        let from_orig = err.index_span().slice(text);
        let from_snippet = err.error_snippet();
        assert_eq!(from_orig, from_snippet);
    }
});
