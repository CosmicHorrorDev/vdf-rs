# Benchmarks

_Disclaimer: These benchmarks exist solely to give a general idea of various
performance numbers. These are not rigorous and should not be used for
comparing similar projects_

The benchmarks cover parsing, rendering, serializing, and deserializing a
`<STEAM>/debian-installation/controller_base/templates/controller_generic_wasd.vdf`
file from my steam install. This is a fairly large (20 KiB) VDF file that
contains varying structures, types, nesting, etc.

All of the following results were from running on a Linux machine with a
`i5-1135G7 @ 4.20 GHz` CPU. You can run all the benchmarks by running
`cargo bench` from the project directory.

## `keyvalues-parser`

| Name | Description |
| :---: | :--- |
| `parse` | Parses a `controller_generic_wasd.vdf` file |
| `render` | Renders a parsed `controller_generic_wasd.vdf` to a `String` |

```text
parser     fastest       │ slowest       │ median        │ mean          │ samples
├─ parse   489.3 µs      │ 816.2 µs      │ 495.2 µs      │ 500.3 µs      │ 5995
│          40.83 MB/s    │ 24.48 MB/s    │ 40.34 MB/s    │ 39.93 MB/s    │
╰─ render  76.04 µs      │ 151.7 µs      │ 78.04 µs      │ 78.79 µs      │ 38015
           250.2 MB/s    │ 125.4 MB/s    │ 243.7 MB/s    │ 241.4 MB/s    │
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
│  ├─ FullStructBorrowed  553.3 µs      │ 781.7 µs      │ 563 µs        │ 569.7 µs      │ 5256
│  │                      36.1 MB/s     │ 25.56 MB/s    │ 35.49 MB/s    │ 35.07 MB/s    │
│  ├─ FullStructOwned     559.2 µs      │ 702 µs        │ 568.8 µs      │ 576.5 µs      │ 5184
│  │                      35.73 MB/s    │ 28.46 MB/s    │ 35.13 MB/s    │ 34.65 MB/s    │
│  ╰─ SingleField         545.8 µs      │ 907.1 µs      │ 558.6 µs      │ 563.5 µs      │ 5323
│                         36.6 MB/s     │ 22.02 MB/s    │ 35.77 MB/s    │ 35.46 MB/s    │
╰─ serialize                            │               │               │               │
   ├─ FullStructBorrowed  155.5 µs      │ 244.9 µs      │ 162.2 µs      │ 168.2 µs      │ 16952
   │                      122.2 MB/s    │ 77.61 MB/s    │ 117.2 MB/s    │ 113 MB/s      │
   ╰─ FullStructOwned     156.2 µs      │ 234.1 µs      │ 173.2 µs      │ 176.8 µs      │ 16897
                          121.6 MB/s    │ 81.2 MB/s     │ 109.7 MB/s    │ 107.4 MB/s    │
```
