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
├─ parse   487.7 µs      │ 991.3 µs      │ 565.6 µs      │ 570.2 µs      │ 5261
│          40.96 MB/s    │ 20.15 MB/s    │ 35.32 MB/s    │ 35.04 MB/s    │
╰─ render  79.09 µs      │ 215.8 µs      │ 84.52 µs      │ 85.43 µs      │ 35053
           240.5 MB/s    │ 88.15 MB/s    │ 225.1 MB/s    │ 222.7 MB/s    │
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
│  ├─ FullStructBorrowed  558.4 µs      │ 993.2 µs      │ 623.6 µs      │ 625.1 µs      │ 4790
│  │                      35.78 MB/s    │ 20.11 MB/s    │ 32.03 MB/s    │ 31.96 MB/s    │
│  ├─ FullStructOwned     610.7 µs      │ 991.9 µs      │ 629.9 µs      │ 632.3 µs      │ 4726
│  │                      32.71 MB/s    │ 20.14 MB/s    │ 31.72 MB/s    │ 31.59 MB/s    │
│  ╰─ SingleField         598.4 µs      │ 1.144 ms      │ 621.2 µs      │ 622.8 µs      │ 4816
│                         33.38 MB/s    │ 17.45 MB/s    │ 32.16 MB/s    │ 32.07 MB/s    │
╰─ serialize                            │               │               │               │
   ├─ FullStructBorrowed  183.9 µs      │ 1.128 ms      │ 194.7 µs      │ 202.3 µs      │ 14721
   │                      103.3 MB/s    │ 16.84 MB/s    │ 97.63 MB/s    │ 93.96 MB/s    │
   ╰─ FullStructOwned     176.7 µs      │ 505.2 µs      │ 193.3 µs      │ 194.5 µs      │ 15317
                          107.5 MB/s    │ 37.62 MB/s    │ 98.32 MB/s    │ 97.73 MB/s    │
```
