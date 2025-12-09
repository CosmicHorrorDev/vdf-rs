# Version 0.2.2 | 2025-12-08

## Feat

- Add limited `Deserialize` impls for `Obj` and `Value` [(#101)]
  - Limited because VDF doesn't act as a fully self-describing format, so
    things may fail to deserialize for seemingly benign reasons. Use at your own
    risk

## Docs

- Add a crates.io badge to the README [(#103)]

## Internal

- Cleanup generated parser code [(#99)] [(#107)]
- Prune some test/benchmark assets [(#105)]

[(#99)]: https://github.com/CosmicHorrorDev/vdf-rs/pull/99
[(#101)]: https://github.com/CosmicHorrorDev/vdf-rs/pull/101
[(#103)]: https://github.com/CosmicHorrorDev/vdf-rs/pull/103
[(#105)]: https://github.com/CosmicHorrorDev/vdf-rs/pull/105
[(#107)]: https://github.com/CosmicHorrorDev/vdf-rs/pull/107

# Version 0.2.1 | 2025-09-22

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
