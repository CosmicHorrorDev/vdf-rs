#![no_main]
use keyvalues::serde::tokens::{NaiveTokenStream, TokenStream};
use keyvalues_parser::Vdf;
use libfuzzer_sys::fuzz_target;

use std::convert::TryFrom;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = Vdf::parse(s).map(|initial_vdf| {
            // Now try roundtripping to a through tokenstreams and back to see if it's the same
            let tokenstream = TokenStream::from(initial_vdf.clone());
            let naive_tokenstream = NaiveTokenStream::from(tokenstream);

            // Since this came from a valid VDF it should be valid no matter what
            let end_vdf = Vdf::try_from(&naive_tokenstream).expect("VDF should be valid");

            assert_eq!(initial_vdf, end_vdf, "VDFs should be equal");
        });
    }
});
