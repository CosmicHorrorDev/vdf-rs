use insta::assert_snapshot;
use keyvalues_serde::{error::Result, from_str};
use serde::Deserialize;

use std::{collections::HashMap, fmt};

mod utils;

use crate::utils::{read_asset_file, BoxedResult, Container};

// Helper macro that generates the boilerplate for snapshotting a deserialization error backed by a
// file
macro_rules! test_snapshot_de {
    ($func_name:ident, $de_ty:ty, $file_name:expr $(,)?) => {
        #[test]
        fn $func_name() -> BoxedResult<()> {
            let vdf_text = read_asset_file($file_name)?;
            snapshot_de_err::<$de_ty>(stringify!($func_name), &vdf_text);

            Ok(())
        }
    };
}

fn snapshot_de_err<'a, T: Deserialize<'a> + fmt::Debug>(snapshot_name: &str, vdf_text: &'a str) {
    let result: Result<T> = from_str(vdf_text);
    let err = result.unwrap_err();
    assert_snapshot!(snapshot_name, err.to_string());
}

test_snapshot_de!(
    str_when_wanting_obj,
    Container<HashMap<String, String>>,
    "string_container.vdf",
);

test_snapshot_de!(
    str_when_wanting_top_level_obj,
    Container<String>,
    "top_level_string.vdf",
);

test_snapshot_de!(obj_when_wanting_str, Container<String>, "obj_container.vdf");

#[test]
fn incorrect_seq_length() -> BoxedResult<()> {
    let name_base = "incorrect_seq_length";
    let vdf_len_one = read_asset_file("string_container.vdf")?;
    let name = format!("{}-one_expecting_two", name_base);
    snapshot_de_err::<Container<(String, String)>>(&name, &vdf_len_one);

    let vdf_len_two = read_asset_file("sequence_string_double.vdf")?;
    let name = format!("{}-two_expecting_one", name_base);
    snapshot_de_err::<Container<(String,)>>(&name, &vdf_len_two);
    let name = format!("{}-two_expecting_three", name_base);
    snapshot_de_err::<Container<(String, String, String)>>(&name, &vdf_len_two);

    Ok(())
}

#[test]
fn parsing_obj_as_sequence() {
    let text = r#""Blah" {}"#;
    assert!(from_str::<Vec<String>>(text).is_err());
}

#[derive(Deserialize, Debug)]
pub struct Pair {
    pub first: String,
    pub second: String,
}

test_snapshot_de!(wants_too_many_members, Pair, "string_container.vdf",);

const INVALID_BOOL_TEXT: &str = r#"
"Container"
{
    "inner" "2"
}
"#;

#[test]
fn invalid_bool() {
    snapshot_de_err::<Container<bool>>("invalid_bool", INVALID_BOOL_TEXT);
}

const ZERO_LEN_CHAR_TEXT: &str = r#"
"Container"
{
    "inner" ""
}
"#;

const TWO_LEN_CHAR_TEXT: &str = r#"
"Container"
{
    "inner" "ab"
}
"#;

#[test]
fn invalid_chars() {
    let name_base = "invalid_chars";
    snapshot_de_err::<Container<char>>(&format!("{}-zero_len", name_base), ZERO_LEN_CHAR_TEXT);
    snapshot_de_err::<Container<char>>(&format!("{}-two_len", name_base), TWO_LEN_CHAR_TEXT);
}
