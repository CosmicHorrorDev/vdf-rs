// TODO: a lot of these should be moved to integration tests

use insta::{assert_debug_snapshot, assert_snapshot};

use std::{borrow::Cow, collections::BTreeMap, error::Error, fs, path::Path};

use keyvalues_parser::{Obj, PartialVdf, Value, Vdf};

type BoxedResult<T> = Result<T, Box<dyn Error>>;

fn read_asset_file(file_name: &str) -> BoxedResult<String> {
    let val = fs::read_to_string(Path::new("tests").join("assets").join(file_name))?;
    Ok(val)
}

// Just mirror the internal types to allow for deriving `Serialize`
#[derive(Debug)]
#[allow(dead_code)]
struct PartialVdfDef<'a> {
    key: Cow<'a, str>,
    value: ValueDef<'a>,
    bases: Vec<Cow<'a, str>>,
}

impl<'a> From<PartialVdf<'a>> for PartialVdfDef<'a> {
    fn from(partial_vdf: PartialVdf<'a>) -> Self {
        let PartialVdf { key, value, bases } = partial_vdf;
        Self {
            key,
            value: ValueDef::from(value),
            bases,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct VdfDef<'a> {
    key: Cow<'a, str>,
    value: ValueDef<'a>,
}

impl<'a> From<Vdf<'a>> for VdfDef<'a> {
    fn from(vdf: Vdf<'a>) -> Self {
        let Vdf { key, value } = vdf;
        Self {
            key,
            value: ValueDef::from(value),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum ValueDef<'a> {
    Str(Cow<'a, str>),
    Obj(ObjDef<'a>),
}

impl<'a> From<Value<'a>> for ValueDef<'a> {
    fn from(value: Value<'a>) -> Self {
        match value {
            Value::Str(s) => Self::Str(s),
            Value::Obj(obj) => Self::Obj(ObjDef::from(obj)),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct ObjDef<'a>(BTreeMap<Cow<'a, str>, Vec<ValueDef<'a>>>);

impl<'a> From<Obj<'a>> for ObjDef<'a> {
    fn from(obj: Obj<'a>) -> Self {
        let inner = obj
            .into_inner()
            .into_iter()
            .map(|(key, values)| {
                let value_defs = values.into_iter().map(ValueDef::from).collect();
                (key, value_defs)
            })
            .collect();
        Self(inner)
    }
}

// Snapshots both parsing and re-rendering the text from a file
fn snapshot_test_parse_and_render(snapshot_name_base: &str, file_name: &str) -> BoxedResult<()> {
    let vdf_text = read_asset_file(file_name)?;
    let vdf = Vdf::parse(&vdf_text)?;
    assert_debug_snapshot!(
        format!("parsed-{}", snapshot_name_base),
        VdfDef::from(vdf.clone())
    );

    let rendered = vdf.to_string();
    assert_snapshot!(format!("rendered-{}", snapshot_name_base), rendered);

    Ok(())
}

fn snapshot_test_raw_parse_render(snapshot_name_base: &str, file_name: &str) -> BoxedResult<()> {
    let vdf_text = read_asset_file(file_name)?;
    let vdf = Vdf::parse_raw(&vdf_text)?;
    assert_debug_snapshot!(
        format!("parsed-{}", snapshot_name_base),
        VdfDef::from(vdf.clone())
    );

    let mut rendered = String::new();
    vdf.render_raw(&mut rendered)?;
    assert_snapshot!(format!("rendered-{}", snapshot_name_base), rendered);

    Ok(())
}

// Snapshots both parsing and re-rendering the text from a file
fn snapshot_test_partial_parse_and_render(
    snapshot_name_base: &str,
    file_name: &str,
) -> BoxedResult<()> {
    let vdf_text = read_asset_file(file_name)?;
    let vdf = PartialVdf::parse(&vdf_text)?;
    assert_debug_snapshot!(
        format!("parsed-{}", snapshot_name_base),
        PartialVdfDef::from(vdf.clone())
    );

    let rendered = vdf.to_string();
    assert_snapshot!(format!("rendered-{}", snapshot_name_base), rendered);

    Ok(())
}

fn snapshot_test_partial_raw_parse_render(
    snapshot_name_base: &str,
    file_name: &str,
) -> BoxedResult<()> {
    let vdf_text = read_asset_file(file_name)?;
    let vdf = PartialVdf::parse_raw(&vdf_text)?;
    assert_debug_snapshot!(
        format!("parsed-{}", snapshot_name_base),
        PartialVdfDef::from(vdf.clone())
    );

    let mut rendered = String::new();
    vdf.render_raw(&mut rendered)?;
    assert_snapshot!(format!("rendered-{}", snapshot_name_base), rendered);

    Ok(())
}

// Generates tests where the `name`s indicate the unit test name and the file without an extension
macro_rules! parse_test_generator {
    ( $test_type:ident, $( $name:ident ),* ) => {
        $(
            #[test]
            fn $name() -> BoxedResult<()> {
                let name_str = stringify!($name);
                ($test_type)(name_str, &format!("{}.vdf", name_str))
            }
        )*
    }
}

parse_test_generator!(
    snapshot_test_parse_and_render,
    basic,
    comments,
    unquoted_strings,
    special_characters,
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
