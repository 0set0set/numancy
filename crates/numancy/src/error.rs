//! Typed errors for invalid numerology inputs.

use core::fmt;

/// Errors returned when inputs cannot produce a numerology result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumerologyError {
  /// The name contained no usable letters after normalization.
  EmptyName,
  /// The address contained no usable digits or letters.
  EmptyAddress,
  /// The calendar date is out of the accepted range.
  InvalidDate {
    /// Provided year.
    year: u32,
    /// Provided month.
    month: u32,
    /// Provided day.
    day: u32,
  },
}

impl fmt::Display for NumerologyError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::EmptyName => write!(f, "name has no usable letters"),
      Self::EmptyAddress => write!(f, "address has no usable digits or letters"),
      Self::InvalidDate { year, month, day } => {
        write!(f, "invalid date {year}-{month:02}-{day:02}")
      }
    }
  }
}

impl std::error::Error for NumerologyError {}
