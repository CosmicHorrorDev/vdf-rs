use keyvalues_parser::Vdf;

mod fuzzer;

#[test]
fn raw_no_newline() {
    let vdf_text = "no newline";
    Vdf::parse(vdf_text).unwrap();
}
