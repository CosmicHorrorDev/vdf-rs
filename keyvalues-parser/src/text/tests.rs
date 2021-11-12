use insta::{assert_ron_snapshot, assert_snapshot};

use std::{error::Error, fs, path::Path};

use crate::{text::parse::Opts, Vdf};

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

// Generates tests where the `name`s indicate the unit test name and the file without an extension
macro_rules! parse_render_tests_from_files {
    ( $( $name:ident ),* ) => {
        $(
            #[test]
            fn $name() -> BoxedResult<()> {
                snapshot_test_parse_and_render(&format!("{}.vdf", stringify!($name)))
            }
        )*
    }
}

parse_render_tests_from_files!(
    basic,
    app_manifest,
    comments,
    unquoted_strings,
    special_characters,
    app_info,
    null_byte
);

#[test]
fn read_raw_strings() -> BoxedResult<()> {
    let vdf_text = read_asset_file("raw_strings.vdf")?;
    let vdf = Vdf::parse_with_opts(
        &vdf_text,
        Opts {
            parse_escaped_characters: false,
        },
    )?;
    assert_ron_snapshot!(vdf);

    Ok(())
}
