use serde::{Deserialize, Serialize};

use std::{collections::HashMap, fs, path::Path};

fn read_asset_file(file_name: &str) -> std::io::Result<String> {
    let asset_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(file_name);
    fs::read_to_string(asset_path)
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename = "controller_mappings")]
struct ControllerMappings {
    version: u64,
    game: String,
    #[serde(rename = "group")]
    groups: Vec<Group>,
    group_source_bindings: HashMap<u64, ControllerComponent>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Group {
    id: u64,
    mode: String,
    bindings: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
enum ControllerComponent {
    ButtonDiamond,
    LeftTrackpad,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdf_text = read_asset_file("controller_mappings.vdf")?;

    // Deserialize the VDF file
    let mut mappings: ControllerMappings = keyvalues_serde::from_str(&vdf_text)?;
    println!("Deserialized representation:");
    println!("{:#?}", mappings);

    // Modify the VDF to your heart's content
    mappings.game = String::from("Custom layout");

    // Serialize it back to VDF text
    let modified_text = keyvalues_serde::to_string(&mappings)?;
    println!("Reserialized representation:");
    println!("{}", modified_text);

    Ok(())
}
