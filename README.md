# _vdf-rs_

The `vdf-rs` project is focused on providing sane methods of dealing with
Valve's Data Format v1 and v2 (VDF) also known as
[KeyValues](https://developer.valvesoftware.com/wiki/KeyValues).

Currently the project is composed of

 - [`keyvalues-parser`](keyvalues-parse) - A lower(ish) level parser/renderer for VDF text files
 - [`keyvalues-serde`](keyvalues-serde) - (De)serialization built on the [`serde`](https://lib.rs/crates/serde) framework

## License

Licensed under either of

 - Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 - MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
