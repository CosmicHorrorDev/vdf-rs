use keyvalues_parser::Vdf;

use std::{fs, path::Path};

fn read_asset_file(file_name: &str) -> std::io::Result<String> {
    let asset_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(file_name);
    fs::read_to_string(asset_path)
}

fn get_version<'a>(controller_mappings: &'a Vdf<'a>) -> Option<&'a str> {
    controller_mappings
        .value
        .get_obj()?
        .get("version")?
        .get(0)?
        .get_str()
}

fn update_version(controller_mappings: &mut Vdf, new_version: String) -> Option<()> {
    let version = controller_mappings
        .value
        .get_mut_obj()?
        .get_mut("version")?
        .get_mut(0)?
        .get_mut_str()?
        .to_mut();

    *version = new_version;

    Some(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdf_text = read_asset_file("parse_mutate_render.vdf")?;
    let mut controller_mappings = Vdf::parse(&vdf_text)?;

    // Reading information from VDF:
    // This involves a lot of `Option`s so it's moved inside a function
    let version = get_version(&controller_mappings).expect("controller_mappings must have version");
    println!("Old Controller Mappings Version: {}", version);

    // Mutating information from VDF:
    // Updating the version
    update_version(&mut controller_mappings, "3".to_string())
        .expect("controller_mappings must have version");

    // Render the VDF:
    // `Vdf` implements `Display` which also provides `.to_string()`
    println!("Updated Controller Mappings:\n{}", controller_mappings);
    assert_eq!(get_version(&controller_mappings), Some("3"));

    Ok(())
}
