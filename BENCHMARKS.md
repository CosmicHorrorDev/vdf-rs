# Benchmarks

The benchmarks cover parsing, rendering, serializing, and deserializing the
`keyvalues-parser/tests/app_info.vdf` and `keyvalues-serde/tests/app_info.vdf`
files. These files are identical and represent a slightly modified app info
output from `steamcmd`. The only modifications are some structural changes that
make (de)serializing sane (In practice you would do this modification to the
`Vdf` representation before deserializing).

All of the following results were from running on a Ryzen 7 3700 running Arch
Linux. You can run all the bencmarks by running either `cargo bench` or
`cargo criterion` from the project directory.

## `keyvalues-parser`

| Name | Result | Description |
| :---: | :---: | :--- |
| `parse timing` | 24 μs | Times parsing the `app_info.vdf` file |
| `parse throughput` | 103 MiB/s | Throughput of ^^ |
| `render timing` | 14 μs | Times rendering the `Vdf` representation of `app_info.vdf` to a `String` |
| `render throughput` | 177 MiB//s | Throughput of ^^ |

## `keyvalues-serde`

| Name | Result | Description |
| :---: | :---: | :--- |
| `de all timing` | 47 μs | Times deserializing the entirety of `app_info.vdf` to a struct |
| `de all throughput` | 50 MiB/s | Throughput of ^^ |
| `de extract timing` | 11 ms | Times extracting a single deeply nested value from `app_info.vdf` |
| `ser all timing` | 49 μs | Times serializing the entirety of `app_info.vdf` from a struct |
| `ser all throughput` | 50 MiB/s | Throughput of ^^ |
