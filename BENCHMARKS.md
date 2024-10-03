# Benchmarks

The benchmarks cover parsing, rendering, serializing, and deserializing the
`keyvalues-parser/tests/app_info.vdf` and `keyvalues-serde/tests/app_info.vdf`
files. These files are identical and represent a slightly modified app info
output from `steamcmd`. The only modifications are some structural changes that
make (de)serializing sane (In practice you would do this modification to the
`Vdf` representation before deserializing).

All of the following results were from running on a Linux machine with a Ryzen
7 3700. You can run all the benchmarks by running either `cargo bench` from the
project directory.

## `keyvalues-parser`

| Name | Description |
| :---: | :--- |
| `parse` | Parses an `app_info.vdf` file |
| `render` | Renders a parsed `app_info.vdf` to a `String` |

```text
Timer precision: 20 ns
parser     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ parse   23.82 µs      │ 38.58 µs      │ 24.77 µs      │ 27.87 µs      │ 100     │ 100
│          110.3 MB/s    │ 68.16 MB/s    │ 106.1 MB/s    │ 94.35 MB/s    │         │
╰─ render  16.7 µs       │ 31.31 µs      │ 16.86 µs      │ 17.55 µs      │ 100     │ 100
           157.4 MB/s    │ 83.97 MB/s    │ 155.9 MB/s    │ 149.8 MB/s    │         │
```

## `keyvalues-serde`

| Name |  Description |
| :---: | :--- |
| `deserialize_all_borrowed` | Deserializes an `app_info` file to a struct with borrowed data types |
| `deserialize_all_owned` | Deserializes an `app_info` file to a struct with owned data types |
| `deserialize_extract_single` | Extracts a single nested value from an `app_info` file |
| `serialize` | Serializes a struct representing an `app_info` file |

```text
Timer precision: 20 ns
ser_de                         fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ deserialize_all_borrowed    38.59 µs      │ 76.47 µs      │ 41.22 µs      │ 47.25 µs      │ 100     │ 100
│                              68.14 MB/s    │ 34.38 MB/s    │ 63.79 MB/s    │ 55.65 MB/s    │         │
├─ deserialize_all_owned       57.13 µs      │ 150.2 µs      │ 59.77 µs      │ 63.14 µs      │ 100     │ 100
│                              46.02 MB/s    │ 17.5 MB/s     │ 43.99 MB/s    │ 41.64 MB/s    │         │
├─ deserialize_extract_single  51.67 µs      │ 105 µs        │ 76.06 µs      │ 67.85 µs      │ 100     │ 100
│                              50.89 MB/s    │ 25.02 MB/s    │ 34.57 MB/s    │ 38.76 MB/s    │         │
╰─ serialize                   51.21 µs      │ 122.1 µs      │ 63.96 µs      │ 73.15 µs      │ 100     │ 100
                               50.19 MB/s    │ 21.05 MB/s    │ 40.19 MB/s    │ 35.14 MB/s    │         │
```
