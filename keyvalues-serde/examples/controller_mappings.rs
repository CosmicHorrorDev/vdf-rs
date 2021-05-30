use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const VDF_TEXT: &str = r#"
"controller_mappings"
{
    "version"  "2"
    "game"     "Generic Gamepad"

    "group"
    {
        "id"    "0"
        "mode"  "four_buttons"
        "bindings"
        {
            "button_a"  "xinput_button A"
            "button_b"  "xinput_button B"
            "button_x"  "xinput_button X"
            "button_y"  "xinput_button Y"
        }
    }

    "group"
    {
        "id"    "1"
        "mode"  "dpad"
        "bindings"
        {
            "dpad_north"  "xinput_button dpad_up"
            "dpad_south"  "xinput_button dpad_down"
            "dpad_east"   "xinput_button dpad_right"
            "dpad_west"   "xinput_button dpad_left"
        }
    }

    "group_source_bindings"
    {
        "0"    "button_diamond"
        "1"    "left_trackpad"
    }
}
"#;

#[derive(Deserialize, Serialize, Debug)]
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
    // Deserialize the VDF file
    let mut mappings: ControllerMappings = keyvalues_serde::from_str(VDF_TEXT)?;
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
