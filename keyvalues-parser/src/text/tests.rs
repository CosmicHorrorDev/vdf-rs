use std::{error::Error, fs, path::Path};

use crate::Vdf;

type BoxedResult<T> = Result<T, Box<dyn Error>>;

fn read_asset_file(file_name: &str) -> BoxedResult<String> {
    let val = fs::read_to_string(Path::new("tests").join("assets").join(file_name))?;
    Ok(val)
}

fn snapshot_test_parse_and_render(file_name: &str) -> BoxedResult<()> {
    let vdf_text = read_asset_file(file_name)?;
    let vdf = Vdf::parse(&vdf_text)?;
    insta::assert_ron_snapshot!(vdf);

    let rendered = vdf.to_string();
    insta::assert_snapshot!(rendered);

    Ok(())
}

macro_rules! parse_render_test {
    ($func_name:ident, $file_name: literal) => {
        #[test]
        fn $func_name() -> BoxedResult<()> {
            snapshot_test_parse_and_render($file_name)
        }
    };
}

parse_render_test!(basic, "basic.vdf");
parse_render_test!(app_manifest, "app_manifest.vdf");
parse_render_test!(comments, "comments.vdf");
parse_render_test!(unquoted_strings, "unquoted_strings.vdf");
parse_render_test!(special_charaters, "special_characters.vdf");
parse_render_test!(app_info, "app_info.vdf");
