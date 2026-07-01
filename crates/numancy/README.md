# numancy

[![crates.io](https://img.shields.io/crates/v/numancy.svg)](https://crates.io/crates/numancy)
[![docs.rs](https://img.shields.io/docsrs/numancy)](https://docs.rs/numancy)
[![CI](https://github.com/0set0set/numancy/actions/workflows/ci.yml/badge.svg)](https://github.com/0set0set/numancy/actions/workflows/ci.yml)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

A small, dependency-free Rust library for numerology calculations.

It implements two independent systems:

- **Pythagorean** (`pythagorean` module): the widely used Western system where
  letters map `A=1 .. I=9`, then repeat. Master numbers `11`, `22`, `33` are
  preserved in the core numbers.
- **Cabalistic** (`cabalistic` module): the Brazilian cabalistic system, where
  letters map to `1..8` and graphic accents add their own value. Master numbers
  `11` and `22` are preserved.

The two systems use different letter tables and accent rules, so they are kept
strictly separate rather than mixed.

## What it calculates

Requested core numbers (both systems where applicable):

- Motivation (Soul Urge / Heart's Desire)
- Impression (Personality)
- Personal Year, Personal Month, Personal Day
- House / address number

Additional widely analyzed numbers:

- Expression / Destiny, Life Path, Birthday, Maturity, Universal Day
- Hidden Passion / Hidden Tendencies, Karmic Lessons, Karmic Debts
- Subconscious Self / Subconscious Response

Cabalistic-specific:

- Hidden Talent, Psychic Number, Mission, Destiny
- Life Cycles, Challenges, Decision Moments
- Inverted Life Triangle, dominant arcana and regent arcane
- Signature analysis
- Relationship harmony, harmonic numbers and favorable days (lookup tables;
  extensible)

## Example

```rust
use numancy::{BirthDate, chart::CabalisticMap};

let birth = BirthDate::new(1939, 11, 7).unwrap();
let reference = BirthDate::new(2020, 3, 15).unwrap();
let map = CabalisticMap::new("Barbara Liskov", birth, reference).unwrap();

assert_eq!(map.motivation.value, 11); // a master number
assert_eq!(map.impression.value, 4);
assert_eq!(map.expression.value, 6);
assert_eq!(map.life_triangle.regent, 7);
```

The tests and documentation use only public figures (pioneering women in
computing) and their publicly known birth dates as examples.

## Disclaimer

Numerology is a symbolic, reflective practice, not a scientific or predictive
system. This library only performs the arithmetic of the tradition; it makes no
claims about the meaning or truth of the results.

## Development

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## License

Licensed under the [Apache License, Version 2.0](LICENSE). Unless you explicitly
state otherwise, any contribution intentionally submitted for inclusion in this
crate shall be licensed as above, without any additional terms or conditions.
