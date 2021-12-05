//! `keyvalues-serde` is a (de)serialization library for
//! [VDF text v1 and v2](https://developer.valvesoftware.com/wiki/KeyValues)
//! built on the [`serde`](https://lib.rs/crates/serde) framework. This library
//! leverages `keyvalues-parser` for parsing and rendering the keyvalues text. This
//! makes it easy to deal with VDF text files using strongly typed Rust structures.
//!
//! ## Installation
//!
//! _Note: this requires at least Rust `1.42.0`_
//!
//! Just add the following to your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! keyvalues-serde = "0.1.0"
//! ```
//!
//! ## Usage
//!
//! There is documentation available
//! [here](https://docs.rs/keyvalues-serde/0.2.0/keyvalues_serde/) and there are
//! examples available in the
//! [examples directory](https://github.com/LovecraftianHorror/vdf-rs/tree/main/keyvalues-serde/examples)
//!
//! ### Quickstart
//!
//! ```toml
//! [dependencies]
//! keyvalues-serde = "0.1.0"
//! serde = { version = "1.0.0", features = ["derive"] }
//! ```
//!
//! ```rust
//! use keyvalues_serde::{from_str, Result as VdfResult};
//! use serde::Deserialize;
//!
//! // Contents take from my ~/.data/Steam/steam/games/PlatformMenu.vdf
//! const VDF_TEXT: &str = r##"
//! // this file defines the contents of the platform menu
//! "Platform"
//! {
//!     "Menu"
//!     {
//!         "Games"
//!         {
//!             "dll"       "steamui"
//!             "interface" "SteamUIGames001"
//!             "MenuName"  "#Steam_Games"
//!             "SteamApp"  "1"
//!         }
//!         "Friends"
//!         {
//!             "dll"       "bin/friendsui"
//!             "interface" "VGuiModuleTracker001"
//!             "MenuName"  "#App_Friends"
//!         }
//!         "Servers"
//!         {
//!             "dll"       "bin/serverbrowser"
//!             "interface" "VGuiModuleServerBrowser001"
//!             "MenuName"  "#App_Servers"
//!         }
//!         "Settings"
//!         {
//!             "dll"       "steamui"
//!             "interface" "VGuiModuleSettings001"
//!             "MenuName"  "#App_Settings"
//!             "SteamApp"  "1"
//!         }
//!     }
//! }
//! "##;
//!
//! #[derive(Deserialize, Debug)]
//! struct Platform {
//!     #[serde(rename = "Menu")]
//!     menu: Menu,
//! }
//!
//! #[derive(Deserialize, Debug)]
//! #[serde(rename_all = "PascalCase")]
//! struct Menu {
//!     games: MenuModule,
//!     friends: MenuModule,
//!     servers: MenuModule,
//!     settings: MenuModule,
//! }
//!
//! #[derive(Deserialize, Debug)]
//! struct MenuModule {
//!     dll: String,
//!     interface: String,
//!     #[serde(rename = "MenuName")]
//!     menu_name: String,
//!     #[serde(rename = "SteamApp")]
//!     steam_app: Option<bool>,
//! }
//!
//! fn main() -> VdfResult<()> {
//!     let platform: Platform = from_str(VDF_TEXT)?;
//!     println!("{:#?}", platform);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Datatypes
//!
//! ### Supported
//!
//! - Primitive Types
//!     - `bool`
//!     - `i8`, `i16`, `i32`, `i64`, `i128`
//!     - `u8`, `u16`, `u32`, `u64`, `u128`
//!     - `f32`, `f64`
//!     - `char`
//! - `String`
//! - `Option`
//!     - VDF doesn't have the concept of a `null` type, so an optional value is considered `Some`
//!       if present and `None` if missing
//! - Unit Variant Enum
//!     - Represented as text matching the variant name
//! - Newtype Struct
//!     - Considered just a wrapper over the contained data type
//! - Homogenous Sequences (`Vec`-like types)
//!     - Represented as several pairs with the same key
//! - Heterogeneous Sequences (`tuple`-like types)
//!     - Represented as several pairs with the same key
//! - TupleStruct
//!     - Considered a wrapper over the contained tuple
//! - Map (`HashMap`-like types)
//!     - Represented by a list of pairs contained within curly-braces `{}`
//! - Struct
//!     - The same as Map. The name of the struct is ignored unless it's the used for the top-level
//!       key
//!
//! ### Unsupported
//!
//! | Type | Reasoning |
//! | :---: | :--- |
//! | Byte Array | No clear VDF representation |
//! | Unit | No clear VDF representation |
//! | Unit Struct | No clear VDF representation |
//! | Enum-containers (newtype, tuple, and struct variants) | The only clear usage would be the untagged representation in which case the ambiguity of types (everything is essentially just strings or objects) allows for too many footguns for me to be comfortable supporting |
//!
//! ## Potential Pitfalls
//!
//! - Any sequence types containing `Option`s may lead to unexpected ordering issues since a `None`
//!   is just ommitted
//!     - For instance a tuple containing an `Option` in the middle will be very problematic
//! - Empty `Vec`s and `Option`s with `None` are both ommitted when serializing.
//! - Nested sequences are impossible to represent due to the limited nature of sequences in VDF
//!   (AFAIK)

pub mod de;
pub mod error;
pub mod ser;
mod tokens;

#[doc(inline)]
pub use de::{from_str, from_str_with_key, Deserializer};
#[doc(inline)]
pub use error::{Error, Result};
#[doc(inline)]
pub use ser::{to_string, to_string_with_key, to_writer, to_writer_with_key, Serializer};
