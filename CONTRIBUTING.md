Thanks for looking into contributing to the project! There's not much out of
the ordinary for this project. The essentials are

- Test with `cargo test`
- Format code with `cargo fmt`
- Fix lints found with `cargo clippy`
- Benchmarks can be performed with `cargo bench` (optionally `cargo criterion`)

and the third-party cargo extensions used are

- [`cargo-insta`](https://lib.rs/crates/cargo-insta) for snapshot testing
    - Interacted with `cargo insta review` when needed
- [`cargo-fuzz`](https://lib.rs/crates/cargo-fuzz) for fuzz testing
    - Fuzzing is run in CI and shouldn't need to be run locally

# Cutting a Release

Releases are published for `keyvalues-parser` and `keyvalues-serde` by pushing
a `parser-v<VERSION>` and `serde-v<VERSION>` respectively. This will run the
test suite, all fuzzers, and then publish the library to crates.io. Before
publishing at the very least the following checklist should be covered

1. Before Publishing
    - [ ] Bump the version in `Cargo.toml`
    - [ ] Bump the version in the crate's `README.md`
    - [ ] Run the benchmarks and update the results in `BENCHMARKS.md`
    - [ ] Update the MSRV `$ cargo msrv --min 1.60.0 -- cargo check`
    - [ ] Consult with `cargo-semver-checks`
    - [ ] Consult with `cargo-deny`
    - [ ] Write relevant release notes in corresponding `CHANGELOG.md`
2. Push the appropriate tag as noted above
3. Write up release notes from the `CHANGELOG.md` and publish them to the GH
   release
