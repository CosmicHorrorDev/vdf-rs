use insta::{assert_ron_snapshot, assert_snapshot};

use std::{error::Error, fs, path::Path};

use crate::{PartialVdf, Vdf};

type BoxedResult<T> = Result<T, Box<dyn Error>>;

fn read_asset_file(file_name: &str) -> BoxedResult<String> {
    let val = fs::read_to_string(Path::new("tests").join("assets").join(file_name))?;
    Ok(val)
}

// Snapshots both parsing and re-rendering the text from a file
fn snapshot_test_parse_and_render(file_name: &str) -> BoxedResult<()> {
    let vdf_text = read_asset_file(file_name)?;
    let vdf = Vdf::parse(&vdf_text)?;
    assert_ron_snapshot!(vdf);

    let rendered = vdf.to_string();
    assert_snapshot!(rendered);

    Ok(())
}

fn snapshot_test_raw_parse_render(file_name: &str) -> BoxedResult<()> {
    let vdf_text = read_asset_file(file_name)?;
    let vdf = Vdf::parse_raw(&vdf_text)?;
    assert_ron_snapshot!(vdf);

    let mut buf = String::new();
    vdf.render_raw(&mut buf)?;
    assert_snapshot!(buf);

    Ok(())
}

// Snapshots both parsing and re-rendering the text from a file
fn snapshot_test_partial_parse_and_render(file_name: &str) -> BoxedResult<()> {
    let vdf_text = read_asset_file(file_name)?;
    let vdf = PartialVdf::parse(&vdf_text)?;
    assert_ron_snapshot!(vdf);

    let rendered = vdf.to_string();
    assert_snapshot!(rendered);

    Ok(())
}

fn snapshot_test_partial_raw_parse_render(file_name: &str) -> BoxedResult<()> {
    let vdf_text = read_asset_file(file_name)?;
    let vdf = PartialVdf::parse_raw(&vdf_text)?;
    assert_ron_snapshot!(vdf);

    let mut buf = String::new();
    vdf.render_raw(&mut buf)?;
    assert_snapshot!(buf);

    Ok(())
}

// Generates tests where the `name`s indicate the unit test name and the file without an extension
macro_rules! parse_test_generator {
    ( $test_type:ident, $( $name:ident ),* ) => {
        $(
            #[test]
            fn $name() -> BoxedResult<()> {
                ($test_type)(&format!("{}.vdf", stringify!($name)))
            }
        )*
    }
}

parse_test_generator!(
    snapshot_test_parse_and_render,
    basic,
    app_manifest,
    comments,
    unquoted_strings,
    special_characters,
    app_info,
    null_byte
);

parse_test_generator!(snapshot_test_raw_parse_render, raw_strings);

parse_test_generator!(
    snapshot_test_partial_parse_and_render,
    base_multiple,
    base_quoted,
    base_unquoted
);

parse_test_generator!(
    snapshot_test_partial_raw_parse_render,
    base_multiple_raw_strings
);
