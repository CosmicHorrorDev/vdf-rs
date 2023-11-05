# _keyvalues-parser_

[![codecov](https://codecov.io/gh/CosmicHorrorDev/vdf-rs/branch/main/graph/badge.svg?token=L2FUD0098X)](https://codecov.io/gh/CosmicHorrorDev/vdf-rs)
[![build status](https://img.shields.io/github/actions/workflow/status/CosmicHorrorDev/vdf-rs/basic.yml?branch=main)](https://github.com/CosmicHorrorDev/vdf-rs/actions)
[![Documentation](https://img.shields.io/docsrs/keyvalues-parser/latest)](https://docs.rs/keyvalues-parser/latest/keyvalues_parser/)

`keyvalues-parser` uses [`pest`](https://lib.rs/crates/pest) to parse
[VDF text v1](https://developer.valvesoftware.com/wiki/KeyValues)
files to an untyped Rust structure to ease manipulation and navigation. The
parser provides an untyped `Vdf` representation as well as a linear
`TokenStream`

The library is primarily used in conjunction with
[`keyvalues-serde`](https://github.com/CosmicHorrorDev/vdf-rs/tree/main/keyvalues-serde)
which provides a more ergonomic (yet more limiting) means of dealing with VDF
text.

## Installation

Just add the library to your `Cargo.toml`

```toml
[dependencies]
keyvalues-parser = "0.1.0"
```

## Quickstart

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

use keyvalues_parser::Vdf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vdf = Vdf::parse(LOGIN_USERS_VDF)?;
    assert_eq!(
        "12345678901234567",
	vdf.value.unwrap_obj().keys().next().unwrap(),
    );

    Ok(())
}
```

## Limitations

<!-- TODO: This could use a lot of cleanup -->

VDF text is drastically underspecified. This leads to the following liberties
being taken

- Not respecting the ordering of key-value pairs, where the pairs are stored in a `BTreeMap` that sorts the values based on the key
- Because of limitations in representing sequences, an empty `Vec` of values will be rendered as a missing keyvalue pair

## Benchmarks

A set of basic benchmarks can be found in the 
[benches directory](https://github.com/CosmicHorrorDev/vdf-rs/tree/main/keyvalues-parser/benches)

These just test timing and throughput for both parsing and rendering of a
fairly typical VDF file

## License

Licensed under either of

<!-- TODO: symlink these licenses and include in each crate -->

 - Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
