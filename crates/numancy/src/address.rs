//! House / address numerology.

use crate::alphabet::decode;
use crate::error::NumerologyError;
use crate::reduction::{reduce, CalculatedNumber, PYTHAGOREAN_MASTERS};
use crate::System;

/// Sum of an address: numeric digits plus letter values under `system`.
#[must_use]
pub fn address_sum(address: &str, system: System) -> u32 {
  address
    .chars()
    .map(|c| {
      if let Some(digit) = c.to_digit(10) {
        digit
      } else if let Some(decoded) = decode(c) {
        u32::from(system.letter_value(&decoded))
      } else {
        0
      }
    })
    .sum()
}

/// House number for an address, using the Pythagorean letter table.
///
/// # Errors
/// Returns [`NumerologyError::EmptyAddress`] if the address has no digits or
/// letters.
pub fn house_number(address: &str) -> Result<CalculatedNumber, NumerologyError> {
  let sum = address_sum(address, System::Pythagorean);
  if sum == 0 {
    return Err(NumerologyError::EmptyAddress);
  }
  Ok(reduce(sum, PYTHAGOREAN_MASTERS))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn alphanumeric_address() {
    // 4 + 2 + B(2) = 8.
    assert_eq!(house_number("42B").unwrap().value, 8);
  }

  #[test]
  fn pure_digits_reduce() {
    // 7 + 7 = 14 -> 5.
    assert_eq!(house_number("77").unwrap().value, 5);
    // 1 + 0 + 0 = 1.
    assert_eq!(house_number("100").unwrap().value, 1);
  }

  #[test]
  fn punctuation_and_spaces_are_ignored() {
    // "Apt 42-B" -> 4 + 2 + B(2) = 8, same as "42B".
    assert_eq!(
      address_sum("Apt 42-B", System::Pythagorean),
      address_sum("42B", System::Pythagorean) + address_sum("APT", System::Pythagorean)
    );
    assert_eq!(house_number("42 B").unwrap().value, house_number("42B").unwrap().value);
  }

  #[test]
  fn masters_are_preserved() {
    // Digits summing to 29 -> 11 (master).
    assert!(house_number("9992").unwrap().is_master);
  }

  #[test]
  fn empty_address_errors() {
    assert_eq!(house_number("   "), Err(NumerologyError::EmptyAddress));
    assert_eq!(house_number("!!!"), Err(NumerologyError::EmptyAddress));
  }
}
