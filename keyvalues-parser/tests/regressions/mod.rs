use std::iter;

use keyvalues_parser::Vdf;

mod fuzzer;

// The parser could hit a stack overflow when dealing with a string containing a large amount of
// escape sequences. This happened in the wild from a VDF file that contained a large JSON blob in
// a string that required a lot of escapes
#[test]
fn issue_54() {
    // Generates strings e.g. `lots_of_escaped(2)` gives `"" "\"\""`
    fn lots_of_escapes(num_escaped: usize) -> String {
        iter::once("\"\" \"")
            .chain(iter::repeat("\\\"").take(num_escaped))
            .chain(iter::once("\""))
            .collect()
    }

    let vdf_text = lots_of_escapes(20_000);
    Vdf::parse(&vdf_text).unwrap();
}

#[test]
fn raw_no_newline() {
    let vdf_text = "no newline";
    Vdf::parse(vdf_text).unwrap();
}
