#![no_main]
use keyvalues_parser::{error::Result, Vdf};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: (bool, &str)| {
    let (raw, text) = input;
    if raw {
        let _ = Vdf::parse_raw(text).map(|parsed| {
            let mut rendered = String::new();
            parsed.render_raw(&mut rendered).unwrap();
            let reparsed = Vdf::parse_raw(&rendered).unwrap();
            assert_eq!(parsed, reparsed);
        });
    } else {
        let _ = Vdf::parse(text).map(|parsed| {
            // Now try round tripping to a string and back to see if it's the same
            let mut rendered = String::new();
            parsed.render(&mut rendered).unwrap();
            let reparsed = Vdf::parse(&rendered).unwrap();
            assert_eq!(parsed, reparsed);
        });
    }
});
