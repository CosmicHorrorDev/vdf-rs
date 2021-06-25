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

macro_rules! parse_render_test_infer_file {
    ($func_name:ident) => {
        #[test]
        fn $func_name() -> BoxedResult<()> {
            snapshot_test_parse_and_render(&format!("{}.vdf", stringify!($func_name)))
        }
    };
}

parse_render_test_infer_file!(basic);
parse_render_test_infer_file!(app_manifest);
parse_render_test_infer_file!(comments);
parse_render_test_infer_file!(unquoted_strings);
parse_render_test_infer_file!(special_characters);
parse_render_test_infer_file!(app_info);
