# Version 0.2.1

The main headline of this update is significantly slimming down our dependency
tree. We now depend solely on `pest` (and its transitive deps) :tada:

## Fix

- Don't overflow the stack when parsing strings containing many escaped chars [(#94)]

## Deps

- Drop `thiserror` for a manual implementation [(#56)]
- Run `cargo update` and `cargo upgrade` [(#58)]
- Commit generated parser code instead of generating with `pest_derive` [(#95)] [(#96)]

## Docs

- Fix incorrect indentation in README example [(#43)]
- Update installation docs to use `cargo add` [(#58)]
- Spruce up more `Cargo.toml` package fields [(#70)]
- Copy licenses into crates [(#75)]

[(#43)]: https://github.com/CosmicHorrorDev/vdf-rs/pull/43
[(#56)]: https://github.com/CosmicHorrorDev/vdf-rs/pull/56
[(#58)]: https://github.com/CosmicHorrorDev/vdf-rs/pull/58
[(#70)]: https://github.com/CosmicHorrorDev/vdf-rs/pull/70
[(#75)]: https://github.com/CosmicHorrorDev/vdf-rs/pull/75
[(#94)]: https://github.com/CosmicHorrorDev/vdf-rs/pull/94
[(#95)]: https://github.com/CosmicHorrorDev/vdf-rs/pull/95
[(#96)]: https://github.com/CosmicHorrorDev/vdf-rs/pull/96
