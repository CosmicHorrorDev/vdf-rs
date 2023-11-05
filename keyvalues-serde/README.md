# _keyvalues-serde_

[![codecov](https://codecov.io/gh/CosmicHorrorDev/vdf-rs/branch/main/graph/badge.svg?token=L2FUD0098X)](https://codecov.io/gh/CosmicHorrorDev/vdf-rs)
[![build status](https://img.shields.io/github/actions/workflow/status/CosmicHorrorDev/vdf-rs/basic.yml?branch=main)](https://github.com/CosmicHorrorDev/vdf-rs/actions)
[![Documentation](https://img.shields.io/docsrs/keyvalues-serde/latest)](https://docs.rs/keyvalues-serde/latest/keyvalues_serde/)

`keyvalues-serde` is a (de)serialization library for
[VDF text v1](https://developer.valvesoftware.com/wiki/KeyValues)
built on the [`serde`](https://lib.rs/crates/serde) framework. This library
leverages `keyvalues-parser` for parsing and rendering the keyvalues text. This
makes it easy to deal with VDF text files using strongly typed Rust structures.

## Installation

Just add the following to your `Cargo.toml`

```toml
[dependencies]
keyvalues-serde = "0.1.0"
serde = { version = "1.0.0", features = ["derive"] }
```

## Quickstart

```rust
use serde::Deserialize;

// Contents take from my ~/.data/Steam/steam/games/PlatformMenu.vdf
const VDF_TEXT: &str = r##"
// this file defines the contents of the platform menu
"Platform"
{
    "Menu"
    {
        "Games"
        {
            "dll"       "steamui"
            "interface" "SteamUIGames001"
            "MenuName"  "#Steam_Games"
            "SteamApp"  "1"
        }
        "Friends"
        {
            "dll"       "bin/friendsui"
            "interface" "VGuiModuleTracker001"
            "MenuName"  "#App_Friends"
        }
        "Servers"
        {
            "dll"       "bin/serverbrowser"
            "interface" "VGuiModuleServerBrowser001"
            "MenuName"  "#App_Servers"
        }
        "Settings"
        {
            "dll"       "steamui"
            "interface" "VGuiModuleSettings001"
            "MenuName"  "#App_Settings"
            "SteamApp"  "1"
        }
    }
}
"##;

#[derive(Deserialize, Debug)]
struct Platform {
    #[serde(rename = "Menu")]
    menu: Menu,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Menu {
    games: MenuModule,
    friends: MenuModule,
    servers: MenuModule,
    settings: MenuModule,
}

#[derive(Deserialize, Debug)]
struct MenuModule {
    dll: String,
    interface: String,
    #[serde(rename = "MenuName")]
    menu_name: String,
    #[serde(rename = "SteamApp")]
    steam_app: Option<bool>,
}

fn main() -> keyvalues_serde::Result<()> {
    let platform: Platform = keyvalues_serde::from_str(VDF_TEXT)?;
    println!("{:#?}", platform);

    Ok(())
}
```

## Datatypes

### Supported

- Primitive Types
    - `bool`
    - `i8`, `i16`, `i32`, `i64`, `i128`
    - `u8`, `u16`, `u32`, `u64`, `u128`
    - `f32`, `f64`
    - `char`
- `String`
- `Option`
    - VDF doesn't have the concept of a `null` type, so an optional value is considered `Some` if present and `None` if missing
- Unit Variant Enum
    - Represented as text matching the variant name
- Newtype Struct
    - Considered just a wrapper over the contained data type
- Homogeneous Sequences (`Vec`-like types)
    - Represented as several pairs with the same key
- Heterogeneous Sequences (`tuple`-like types)
    - Represented as several pairs with the same key
- TupleStruct
    - Considered a wrapper over the contained tuple
- Map (`HashMap`-like types)
    - Represented by a list of pairs contained within curly-braces `{}`
- Struct
    - The same as Map. The name of the struct is ignored unless it's the used for the top-level key

### Unsupported

| Type | Reasoning |
| :---: | :--- |
| Byte Array | No clear VDF representation |
| Unit | No clear VDF representation |
| Unit Struct | No clear VDF representation |
| Enum-containers (newtype, tuple, and struct variants) | The only clear usage would be the untagged representation in which case the ambiguity of types (everything is essentially just strings or objects) allows for too many footguns for me to be comfortable supporting |

## Potential Pitfalls

- Any sequence types containing `Option`s may lead to unexpected ordering issues since a `None` is just omitted
    - For instance a tuple containing an `Option` in the middle will be very problematic
- Empty `Vec`s and `Option`s with `None` are both omitted when serializing.
- Nested sequences are impossible to represent due to the limited nature of sequences in VDF (AFAIK)

## License

Licensed under either of

<!-- TODO: symlink the license files and publish them in the crate -->
 - Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
