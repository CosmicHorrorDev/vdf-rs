# _keyvalues-serde_

`keyvalues-serde` is a (de)serialization library for
[VDF text v1 and v2](https://developer.valvesoftware.com/wiki/KeyValues)
built on the [`serde`](https://lib.rs/crates/serde) framework. This library
leverages `keyvalues-parser` for parsing and rendering the keyvalues text. This
makes it easy to deal with VDF text files using strongly typed Rust structures.

## Installation

_Note: this requires at least Rust `1.42.0`_

Just add the following to your `Cargo.toml`

```toml
[dependencies]
keyvalues-serde = "0.1.0"
```

## Usage

There is documentation available on TODO and there are examples available in
the
[examples directory](https://github.com/LovecraftianHorror/vdf-rs/tree/main/keyvalues-serde/examples)

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
- Homogenous Sequences (`Vec`-like types)
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

- Any sequence types containing `Option`s may lead to unexpected ordering issues since a `None` is just ommitted
    - For instance a tuple containing an `Option` in the middle will be very problematic
- Empty `Vec`s and `Option`s with `None` are both ommitted when serializing.
- Nested sequences are impossible to represent due to the limited nature of sequences in VDF (AFAIK)

## License

Licensed under either of

 - Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 - MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
