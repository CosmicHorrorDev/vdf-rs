# _vdf-rs_

[![codecov](https://codecov.io/gh/CosmicHorrorDev/vdf-rs/branch/main/graph/badge.svg?token=L2FUD0098X)](https://codecov.io/gh/CosmicHorrorDev/vdf-rs)
[![build status](https://img.shields.io/github/actions/workflow/status/CosmicHorrorDev/vdf-rs/basic.yml?branch=main)](https://github.com/CosmicHorrorDev/vdf-rs/actions)

The `vdf-rs` project is focused on providing sane methods of dealing with
Valve's Data Format v1 (VDF) also known as
[KeyValues](https://developer.valvesoftware.com/wiki/KeyValues).

Currently the project is composed of

## [`keyvalues-parser`](keyvalues-parser)

[![codecov](https://codecov.io/gh/CosmicHorrorDev/vdf-rs/branch/main/graph/badge.svg?token=L2FUD0098X)](https://codecov.io/gh/CosmicHorrorDev/vdf-rs)
[![build status](https://img.shields.io/github/actions/workflow/status/CosmicHorrorDev/vdf-rs/basic.yml?branch=main)](https://github.com/CosmicHorrorDev/vdf-rs/actions)
[![Documentation](https://img.shields.io/docsrs/keyvalues-parser/latest)](https://docs.rs/keyvalues-parser/latest/keyvalues_parser/)

A lower level parser/render for VDF text files

```rust
const LOGIN_USERS_VDF: &str = r#"
"users"
{
    "12345678901234567"
    {
        "AccountName"        "ACCOUNT_NAME"
        "PersonaName"        "PERSONA_NAME"
        "RememberPassword"    "1"
        "MostRecent"        "1"
        "Timestamp"        "1234567890"
    }
}
"#;

let vdf = keyvalues_parser::Vdf::parse(LOGIN_USERS_VDF)?;
assert_eq!(
    "12345678901234567",
    vdf.value.unwrap_obj().keys().next().unwrap(),
);
```

## [`keyvalues-serde`](keyvalues-serde) 

[![codecov](https://codecov.io/gh/CosmicHorrorDev/vdf-rs/branch/main/graph/badge.svg?token=L2FUD0098X)](https://codecov.io/gh/CosmicHorrorDev/vdf-rs)
[![build status](https://img.shields.io/github/actions/workflow/status/CosmicHorrorDev/vdf-rs/basic.yml?branch=main)](https://github.com/CosmicHorrorDev/vdf-rs/actions)
[![Documentation](https://img.shields.io/docsrs/keyvalues-serde/latest)](https://docs.rs/keyvalues-serde/latest/keyvalues_serde/)

VDF text (De)serialization built on the [`serde`](https://serde.rs) framework

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

## License

Licensed under either of

 - Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 - MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
