# Benchmarks

_Disclaimer: These benchmarks exist solely to give a general idea of various
performance numbers. These are not rigorous and should not be used for
comparing similar projects_

The benchmarks cover parsing, rendering, serializing, and deserializing a
`<STEAM>/debian-installation/controller_base/templates/controller_generic_wasd.vdf`
file from my steam install. This is a fairly large (20 KiB) VDF file that
contains varying structures, types, nesting, etc.

All of the following results were from running on a Linux machine with a Ryzen
7 3700. You can run all the benchmarks by running either `cargo bench` from the
project directory

## `keyvalues-parser`

| Name | ? |
| :---: | :--- |
| `parse` | Parsing the file |
| `render` | Rendering the parsed file back to a `String` |

```text
parser     fastest       │ slowest       │ median        │ mean          │ samples
├─ parse   110.8 µs      │ 202.3 µs      │ 112.3 µs      │ 113.5 µs      │ 1759
│          180.2 MB/s    │ 98.74 MB/s    │ 177.8 MB/s    │ 175.9 MB/s    │
╰─ render  113.8 µs      │ 161.7 µs      │ 116.9 µs      │ 117.3 µs      │ 1702
           167.1 MB/s    │ 117.6 MB/s    │ 162.6 MB/s    │ 162.2 MB/s    │
```

## `keyvalues-serde`

Deserializes and serializes the file to various flavors of Rust types

| Type | ? |
| :---: | :--- |
| `FullStructOwned` | A `struct` using owned `String` types internally that represents _all_ of the information present in the file
| `FullStructBorrowed` | The same as `FullStructOwned`, but the `struct` uses `&str` types and `#[serde(borrow)]` to support "zero-copy" (de)serialization where possible |
| `SingleField` | A `struct` consisting of only a single simple field ignoring all of the other data in the file |

_Spoilers: Both using borrowed data and ignoring nearly all of the data present
while have a pretty minimal impact compared to the naïve approach_

```text
ser_de                    fastest       │ slowest       │ median        │ mean          │ samples
├─ deserialize                          │               │               │               │
│  ├─ FullStructBorrowed  210.6 µs      │ 400.5 µs      │ 216.5 µs      │ 218.1 µs      │ 910
│  │                      94.85 MB/s    │ 49.88 MB/s    │ 92.28 MB/s    │ 91.57 MB/s    │
│  ├─ FullStructOwned     214.4 µs      │ 255.7 µs      │ 218.7 µs      │ 220 µs        │ 898
│  │                      93.16 MB/s    │ 78.11 MB/s    │ 91.33 MB/s    │ 90.81 MB/s    │
│  ╰─ SingleField         200.9 µs      │ 240.5 µs      │ 206.7 µs      │ 207.5 µs      │ 963
│                         99.44 MB/s    │ 83.07 MB/s    │ 96.64 MB/s    │ 96.28 MB/s    │
╰─ serialize                            │               │               │               │
   ├─ FullStructBorrowed  260.9 µs      │ 497.1 µs      │ 277.7 µs      │ 276.4 µs      │ 714
   │                      72.85 MB/s    │ 38.24 MB/s    │ 68.45 MB/s    │ 68.76 MB/s    │
   ╰─ FullStructOwned     263.4 µs      │ 298.7 µs      │ 279.3 µs      │ 277.7 µs      │ 711
                          72.15 MB/s    │ 63.64 MB/s    │ 68.04 MB/s    │ 68.43 MB/s    │
```
