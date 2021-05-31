# Benchmarks

Currently the benchmark just covers parsing the `tests/app_info.vdf` file which
is a reasonably large VDF file that should be a good representation of a
typical VDF file. All of the following results were from running on a Ryzen 7
3700 on Arch Linux. You can run the benchmarks yourself by running either
`cargo bench` or `cargo criterion` from the `keyvalues-parser` directory.
yielded 24.7 μs for parsing the VDF file at a rate of 103 MiB/s. You can run
the bencmarks for the parser yourself by running either `cargo bench` or
`cargo criterion` from the project directory

| Name | Result | Description |
| :---: | :---: | :--- |
| `parse timing` | 24.7 μs | Times parsing the `tests/assets/app_info.vdf` file |
| `parse throughput` | 103 MiB/s | Tests throughput from parsing the `tests/assets/app_info.vdf` file |
