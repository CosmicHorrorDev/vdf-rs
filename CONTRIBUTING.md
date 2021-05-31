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
