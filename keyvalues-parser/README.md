# _keyvalues-parser_

`keyvalues-parser` uses [`pest`](https://lib.rs/crates/pest) to parse
[VDF text v1 and v2](https://developer.valvesoftware.com/wiki/KeyValues)
files to an untyped Rust structure to ease manipulation and navigation. This
representation is composed of the following (simplified) values.

```rust
type Key = Cow<str>;
type Obj = BTreeMap<Key, Vec<Value>>;

struct Vdf {
    key: Key,
    value: Value,
}

enum Value {
    Str(Cow<str>),
    Obj(Obj),
}
```

Where the following basic VDF text has the corresponding `Vdf` structure

```text
"Outer Key"
{
    "Inner Key"  "Inner Val"
    "Seq Key"    "Str Val"
    "Seq Key"
    {
    }
}
```

```ron
Vdf(
  key: "Outer Key",
  value: Obj({
    "Inner Key": ["Inner Val"],
    "Seq Key": [
      Str("Str Val"),
      Obj({}),
    ],
  }),
)
```

<!--
TODO: list different behavior that may be unexpected like ordering of values
when reserialized and how keys with an empty vec as the value don't get
rendered.

TODO: mention how to actually parse and render the text

TODO: mention limitations of VDF Text

TODO: provide a quickstart on parsing, mutation, then rendering

TODO: mention keyvalues-serde
-->

## License

Licensed under either of

 - Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 - MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
