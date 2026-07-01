//! `numancy` — a dependency-free numerology calculation library.
//!
//! Two independent systems are provided and kept strictly separate because they
//! use different letter tables and accent rules:
//!
//! - [`pythagorean`]: the Western `A=1..I=9` system, preserving master numbers
//!   `11`, `22`, `33`.
//! - [`cabalistic`]: the Brazilian cabalistic `1..8` system, preserving master
//!   numbers `11`, `22`, and valuing graphic accents.
//!
//! High-level profiles live in [`chart`] ([`chart::PythagoreanChart`] and
//! [`chart::CabalisticMap`]). Lower-level building blocks are exposed per topic:
//! [`reduction`], [`alphabet`], [`name`], [`date`], [`address`], [`cycles`],
//! [`pyramid`], [`signature`] and [`compatibility`].
//!
//! Numerology is a symbolic, reflective practice, not a scientific or predictive
//! system; this crate only performs the arithmetic of the tradition.
//!
//! # Example
//!
//! ```
//! use numancy::{chart::CabalisticMap, BirthDate, ReferenceDate};
//!
//! let birth = BirthDate::new(1939, 11, 7)?;
//! let reference = ReferenceDate::new(2020, 3, 15)?;
//! let map = CabalisticMap::new("Barbara Liskov", birth, reference)?;
//!
//! assert_eq!(map.motivation.value, 11); // a master number
//! assert_eq!(map.expression.value, 6);
//! # Ok::<(), numancy::NumerologyError>(())
//! ```

#![warn(missing_docs)]

pub mod address;
pub mod alphabet;
pub mod cabalistic;
pub mod chart;
pub mod compatibility;
pub mod cycles;
pub mod date;
pub mod error;
pub mod name;
pub mod pyramid;
pub mod pythagorean;
pub mod reduction;
pub mod signature;

use alphabet::DecodedChar;
pub use date::{BirthDate, ReferenceDate};
pub use error::NumerologyError;
pub use reduction::CalculatedNumber;

/// The numerology system used to value letters and preserve master numbers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum System {
  /// Western Pythagorean system (`A=1..I=9`, masters 11/22/33).
  Pythagorean,
  /// Brazilian cabalistic system (`1..8`, masters 11/22, accents count).
  Cabalistic,
}

impl System {
  /// The numeric value of a decoded letter under this system.
  #[must_use]
  pub fn letter_value(self, decoded: &DecodedChar) -> u8 {
    match self {
      System::Pythagorean => alphabet::pythagorean_value(decoded),
      System::Cabalistic => alphabet::cabalistic_value(decoded),
    }
  }

  /// The master numbers preserved by this system.
  #[must_use]
  pub fn masters(self) -> &'static [u32] {
    match self {
      System::Pythagorean => reduction::PYTHAGOREAN_MASTERS,
      System::Cabalistic => reduction::CABALISTIC_MASTERS,
    }
  }
}
