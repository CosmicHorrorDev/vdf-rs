# _vdf-rs_

[![codecov](https://codecov.io/gh/LovecraftianHorror/vdf-rs/branch/main/graph/badge.svg?token=L2FUD0098X)](https://codecov.io/gh/LovecraftianHorror/vdf-rs)
[![build status](https://img.shields.io/github/workflow/status/lovecraftianhorror/vdf-rs/Build,%20Test,%20Format,%20and%20Lint)](https://github.com/LovecraftianHorror/vdf-rs/actions)
[![MSRV 1.42](https://img.shields.io/badge/MSRV-1.42-blue.svg)](https://blog.rust-lang.org/2020/03/12/Rust-1.42.html)

The `vdf-rs` project is focused on providing sane methods of dealing with
Valve's Data Format v1 and v2 (VDF) also known as
[KeyValues](https://developer.valvesoftware.com/wiki/KeyValues).

Currently the project is composed of

 - [`keyvalues-parser`](keyvalues-parser) - A lower(ish) level parser/renderer for VDF text files
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
