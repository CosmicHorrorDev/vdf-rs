#![no_main]

use std::borrow::Cow;

use keyvalues_parser::{Obj, Value, Vdf};
use keyvalues_serde::from_str;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|text: &str| {
    if let Ok(obj) = from_str::<Obj<'_>>(text) {
        let vdf = Vdf {
            key: Cow::Borrowed(""),
            value: Value::Obj(obj.clone()),
        };
        let text_again = vdf.to_string();
        let obj2 = from_str::<Obj<'_>>(&text_again).unwrap();
        assert_eq!(obj, obj2);
    }
});
