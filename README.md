# numancy workspace

[![crates.io](https://img.shields.io/crates/v/numancy.svg)](https://crates.io/crates/numancy)
[![docs.rs](https://img.shields.io/docsrs/numancy)](https://docs.rs/numancy)
[![CI](https://github.com/0set0set/numancy/actions/workflows/ci.yml/badge.svg)](https://github.com/0set0set/numancy/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Numerology calculations (Pythagorean and Brazilian cabalistic), split into two
crates:

- [`crates/numancy`](crates/numancy) — the library. Pure, dependency-free Rust;
  this is what is published to [crates.io](https://crates.io/crates/numancy).
- [`crates/numancy-cli`](crates/numancy-cli) — a small command-line tool built on
  top of the library, handy for trying things out. Not published (yet).

## Try the CLI

The CLI prints its result as JSON. Pick a `--system`; `--number` is optional and
selects a single number, while omitting it returns every number of the system.

```bash
# a single number
cargo run -p numancy-cli -- "Grace Hopper" 1906-12-09 \
  --system cabalistic --number motivation
```

```json
{
  "master": false,
  "name": "Grace Hopper",
  "number": "motivation",
  "system": "cabalistic",
  "value": 9
}
```

```bash
# every number of the system (default when --number is omitted)
cargo run -p numancy-cli -- "Grace Hopper" 1906-12-09 --system cabalistic
```

```json
{
  "name": "Grace Hopper",
  "numbers": {
    "birthday": { "master": false, "value": 9 },
    "destiny": { "master": false, "value": 1 },
    "expression": { "master": false, "value": 4 },
    "motivation": { "master": false, "value": 9 }
  },
  "system": "cabalistic"
}
```

The example above is abbreviated: the full object also includes the remaining
numbers of the system, such as `personal-year`, `personal-month`,
`personal-day` and (Pythagorean) `universal-day`, whose values depend on
`--reference`.

`--reference` (YYYY-MM-DD) is only used by `personal-year`, `personal-month`,
`personal-day` and `universal-day`, and defaults to today (UTC). Run
`cargo run -p numancy-cli -- --help` for the full list of systems and numbers.

## Use the library

Add the crate and compute a chart:

```bash
cargo add numancy
```

```rust
use numancy::{chart::CabalisticMap, BirthDate, ReferenceDate};

let birth = BirthDate::new(1939, 11, 7)?;
let reference = ReferenceDate::new(2020, 3, 15)?;
let map = CabalisticMap::new("Barbara Liskov", birth, reference)?;
assert_eq!(map.motivation.value, 11);
# Ok::<(), numancy::NumerologyError>(())
```

See [`crates/numancy/README.md`](crates/numancy/README.md) for the full library
overview

## Development

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Formatting uses only stable `rustfmt` options (see `rustfmt.toml`), so no nightly
toolchain is required. The library builds with zero external dependencies; the
CLI adds `clap` and `time`.

## Disclaimer

Numerology is a symbolic, reflective practice, not a scientific or predictive
system. These crates only perform the arithmetic of the tradition; they make no
claims about the meaning or truth of the results. Examples use only public
figures (pioneering women in computing) and their publicly known birth dates.

## License

Licensed under the [Apache License, Version 2.0](LICENSE). Unless you explicitly
state otherwise, any contribution intentionally submitted for inclusion shall be
licensed as above, without any additional terms or conditions.
