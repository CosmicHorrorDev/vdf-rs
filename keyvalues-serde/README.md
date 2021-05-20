# _keyvalues-serde_

`keyvalues-serde` is a (de)serialization library for
[VDF text v1 and v2](https://developer.valvesoftware.com/wiki/KeyValues)
built on the [`serde`](https://lib.rs/crates/serde) framework. This library
leverages `keyvalues-parser` for parsing and rendering the keyvalues text. This
makes it easy to deal with VDF text files using strongly typed Rust structures.
For instance the following (simplified) VDF text can be manipulated with ease

<!--

TODO: discuss which data types are supported and which aren't

TODO: discuss potential pitfalls (tuples (with options), empty vecs, empty
options, etc.)

TODO: actually run and test the code examples

TODO: maybe slim down the quickstart example and move more complicated ones
into separate examples in the examples directory

-->

`simplified_gamepad.vdf`

```text
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
```

```rust
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Deserialize, Serialize, Debug)]
struct ControllerMappings {
    version: u64,
    game: String,
    #[serde(rename = "group")]
    groups: Vec<Group>,
    group_source_bindings: HashMap<u64, ControllerComponent>,
    settings: HashMap<String, String>,
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
    let vdf_text = fs::read_to_string("simplified_gamepad.vdf")?;
    let mut mappings: ControllerMappings = keyvalues_serde::from_str(&vdf_text)?;

    // Modify and serialize back to VDF
    mappings.game = String::from("Custom layout");
    let modified_text = mappings.to_string();

    Ok(())
}
```

## License

Licensed under either of

 - Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 - MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
