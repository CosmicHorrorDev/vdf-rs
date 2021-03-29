#![no_main]
use libfuzzer_sys::fuzz_target;
use vdf_pest::common::Vdf;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = vdf_pest::common::Vdf::parse(s).map(|initial_vdf| {
            // Now try round tripping to a string and back to see if it's the same
            let unparsed = initial_vdf.to_string();
            let reparsed_vdf = Vdf::parse(&unparsed).unwrap();
            assert_eq!(initial_vdf, reparsed_vdf);
        });
    }
});
