use std::{error::Error, fs, path::Path};

use crate::core::Vdf;

type BoxedResult<T> = Result<T, Box<dyn Error>>;

fn read_asset_file(file_name: &str) -> BoxedResult<String> {
    let val = fs::read_to_string(Path::new("tests").join("assets").join(file_name))?;
    Ok(val)
}

#[test]
fn basic() -> BoxedResult<()> {
    let vdf_text = read_asset_file("basic.vdf")?;
    let vdf = Vdf::parse(&vdf_text)?;
    insta::assert_ron_snapshot!(vdf);

    let rendered = vdf.to_string();
    insta::assert_snapshot!(rendered);

    Ok(())
}

#[test]
fn app_manifest() -> BoxedResult<()> {
    let vdf_text = read_asset_file("app_manifest.vdf")?;
    let vdf = Vdf::parse(&vdf_text)?;
    insta::assert_ron_snapshot!(vdf);

    let rendered = vdf.to_string();
    insta::assert_snapshot!(rendered);

    Ok(())
}

#[test]
fn comments() -> BoxedResult<()> {
    let vdf_text = read_asset_file("comments.vdf")?;
    let vdf = Vdf::parse(&vdf_text)?;
    insta::assert_ron_snapshot!(vdf);

    let rendered = vdf.to_string();
    insta::assert_snapshot!(rendered);

    Ok(())
}

#[test]
fn unquoted_strings() -> BoxedResult<()> {
    let vdf_text = read_asset_file("unquoted_strings.vdf")?;
    let vdf = Vdf::parse(&vdf_text)?;
    insta::assert_ron_snapshot!(vdf);

    let rendered = vdf.to_string();
    insta::assert_snapshot!(rendered);

    Ok(())
}
