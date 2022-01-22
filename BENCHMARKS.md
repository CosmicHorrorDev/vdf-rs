# Benchmarks

The benchmarks cover parsing, rendering, serializing, and deserializing the
`keyvalues-parser/tests/app_info.vdf` and `keyvalues-serde/tests/app_info.vdf`
files. These files are identical and represent a slightly modified app info
output from `steamcmd`. The only modifications are some structural changes that
make (de)serializing sane (In practice you would do this modification to the
`Vdf` representation before deserializing).

All of the following results were from running on a Ryzen 7 3700 running Arch
Linux. You can run all the benchmarks by running either `cargo bench` or
`cargo criterion` from the project directory.

## `keyvalues-parser`

| Name | Result | Description |
| :---: | :---: | :--- |
| `parse timing` | 20.3 μs | Times parsing the `app_info.vdf` file |
| `parse throughput` | 123.4 MiB/s | Throughput of ^^ |
| `render timing` | 12.9 μs | Times rendering the `Vdf` representation of `app_info.vdf` to a `String` |
| `render throughput` | 194.3 MiB/s | Throughput of ^^ |

## `keyvalues-serde`

| Name | Result | Description |
| :---: | :---: | :--- |
| `de all owned timing` | 34.6 μs | Times deserializing the entirety of `app_info.vdf` to a struct |
| `de all owned throughput` | 72.5 MiB/s | Throughput of ^^ |
| `de all borrowed timing` | 35.9 μs | Times deserializing the entirety of `app_info.vdf` to a struct |
| `de all borrowed throughput` | 70.0 MiB/s | Throughput of ^^ |
| `de extract single timing` | 35.4 μs | Times extracting a single deeply nested value from `app_info.vdf` |
| `de extract single throughput` | 70.9 MiB/s | Throughput of ^^ |
| `ser all timing` | 43.6 μs | Times serializing the entirety of `app_info.vdf` from a struct |
| `ser all throughput` | 56.2 MiB/s | Throughput of ^^ |
