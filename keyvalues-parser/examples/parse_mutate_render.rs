use keyvalues_parser::Vdf;

use std::borrow::Cow;

const VDF_TEXT: &str = r#"
"controller_mappings"
{
	"version"		"2"
	"group"
	{
		"mode"		"four_buttons"
	}
	"group"
	{
		"settings"
		{
			"requires_click"		"0"
		}
	}
}
"#;

fn get_version<'a>(controller_mappings: &'a Vdf<'a>) -> Option<&'a Cow<'a, str>> {
    controller_mappings
        .value
        .get_obj()?
        .get("version")?
        .get(0)?
        .get_str()
}

fn update_version<'text, 'func>(
    controller_mappings: &'func mut Vdf<'text>,
    new_version: &str,
) -> Option<()> {
    let version = controller_mappings
        .value
        .get_mut_obj()?
        .get_mut("version")?
        .get_mut(0)?
        .get_mut_str()?
        .to_mut();

    *version = String::from(new_version);

    Some(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut controller_mappings = Vdf::parse(VDF_TEXT)?;

    // Reading information from VDF:
    // This involves a lot of `Option`s so it's moved inside a function
    let version = get_version(&controller_mappings).expect("controller_mappings must have version");
    println!("Controller Mappings Version: {}", version);

    // Mutating information from VDF:
    // Updating the version
    update_version(&mut controller_mappings, "3").expect("controller_mappings must have version");

    // Render the VDF:
    // `Vdf` implements `Display` which also provides `.to_string()`
    println!("{}", controller_mappings);
    assert_eq!(get_version(&controller_mappings), Some(&Cow::from("3")));

    Ok(())
}
