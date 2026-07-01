//! Calendar inputs and date-based Pythagorean calculations.

use crate::error::NumerologyError;
use crate::reduction::{digit_sum, reduce, reduce_single, CalculatedNumber, PYTHAGOREAN_MASTERS};

/// A validated calendar date used both for birth dates and reference dates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BirthDate {
  /// Four-digit year (must be non-zero).
  pub year: u32,
  /// Month `1..=12`.
  pub month: u32,
  /// Day `1..=31`.
  pub day: u32,
}

impl BirthDate {
  /// Create a date, validating basic calendar ranges.
  ///
  /// # Errors
  /// Returns [`NumerologyError::InvalidDate`] when the month, day or year are
  /// out of range.
  pub fn new(year: u32, month: u32, day: u32) -> Result<Self, NumerologyError> {
    if year == 0 || !(1..=12).contains(&month) || !(1..=31).contains(&day) {
      return Err(NumerologyError::InvalidDate { year, month, day });
    }
    Ok(Self { year, month, day })
  }

  /// Sum of the digits of the full date (day, month, year).
  #[must_use]
  pub fn digit_total(self) -> u32 {
    digit_sum(self.year) + digit_sum(self.month) + digit_sum(self.day)
  }
}

/// A reference date (e.g. "today") uses the same shape as a birth date.
pub type ReferenceDate = BirthDate;

/// Life Path number: reduction of the full birth date, preserving masters.
#[must_use]
pub fn life_path_number(birth: BirthDate) -> CalculatedNumber {
  reduce(birth.digit_total(), PYTHAGOREAN_MASTERS)
}

/// Birthday number: the day of birth, reduced and preserving masters.
#[must_use]
pub fn birthday_number(birth: BirthDate) -> CalculatedNumber {
  reduce(birth.day, PYTHAGOREAN_MASTERS)
}

/// Maturity number: reduction of Life Path + Expression.
#[must_use]
pub fn maturity_number(life_path: &CalculatedNumber, expression: &CalculatedNumber) -> CalculatedNumber {
  reduce(life_path.value + expression.value, PYTHAGOREAN_MASTERS)
}

/// Universal Day: reduction of the given calendar date to a single digit.
#[must_use]
pub fn universal_day(date: BirthDate) -> CalculatedNumber {
  reduce_single(date.digit_total())
}

/// Personal Year: birth month + birth day + reference year, single digit.
#[must_use]
pub fn personal_year(birth: BirthDate, year: u32) -> CalculatedNumber {
  reduce_single(digit_sum(birth.day) + digit_sum(birth.month) + digit_sum(year))
}

/// Personal Month: Personal Year + calendar month, single digit.
#[must_use]
pub fn personal_month(birth: BirthDate, year: u32, month: u32) -> CalculatedNumber {
  reduce_single(personal_year(birth, year).value + digit_sum(month))
}

/// Personal Day: Personal Month + calendar day, single digit.
#[must_use]
pub fn personal_day(birth: BirthDate, year: u32, month: u32, day: u32) -> CalculatedNumber {
  reduce_single(personal_month(birth, year, month).value + digit_sum(day))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rejects_invalid_dates() {
    assert!(BirthDate::new(2000, 13, 1).is_err());
    assert!(BirthDate::new(2000, 0, 1).is_err());
    assert!(BirthDate::new(2000, 2, 32).is_err());
    assert!(BirthDate::new(2000, 2, 0).is_err());
    assert!(BirthDate::new(0, 2, 1).is_err());
    assert!(BirthDate::new(2000, 1, 1).is_ok());
    assert!(BirthDate::new(2000, 12, 31).is_ok());
  }

  #[test]
  fn digit_total_sums_the_whole_date() {
    // 2000-03-15 -> 2 + 3 + 6 = 11.
    assert_eq!(BirthDate::new(2000, 3, 15).unwrap().digit_total(), 11);
  }

  #[test]
  fn life_path_known_example() {
    // March 15, 1990 -> 1.
    let cn = life_path_number(BirthDate::new(1990, 3, 15).unwrap());
    assert_eq!(cn.value, 1);
  }

  #[test]
  fn life_path_can_carry_a_karmic_debt() {
    // Ada Lovelace (1815-12-10) totals 19, reducing to 1 through the debt 19.
    let cn = life_path_number(BirthDate::new(1815, 12, 10).unwrap());
    assert_eq!(cn.value, 1);
    assert!(cn.passes_through(19));
  }

  #[test]
  fn life_path_preserves_masters() {
    // Dorothy Vaughan (1910-09-20) totals a master number 22.
    let cn = life_path_number(BirthDate::new(1910, 9, 20).unwrap());
    assert_eq!(cn.value, 22);
    assert!(cn.is_master);
  }

  #[test]
  fn birthday_and_maturity() {
    // Dorothy Vaughan: Birthday from day 20 -> 2; Life Path 22.
    let birth = BirthDate::new(1910, 9, 20).unwrap();
    assert_eq!(birthday_number(birth).value, 2);
    let life_path = life_path_number(birth);
    let expression = reduce(7, PYTHAGOREAN_MASTERS); // stand-in Expression 7.
                                                     // 22 + 7 = 29 -> 11 (master).
    let maturity = maturity_number(&life_path, &expression);
    assert_eq!(maturity.value, 11);
    assert!(maturity.is_master);
  }

  #[test]
  fn personal_year_known_example() {
    // Birth March 8, reference year 2026 -> 3.
    let birth = BirthDate::new(1990, 3, 8).unwrap();
    assert_eq!(personal_year(birth, 2026).value, 3);
  }

  #[test]
  fn personal_month_and_day_chain_from_personal_year() {
    // Grace Hopper (1906-12-09), reference 2020-03-15.
    let birth = BirthDate::new(1906, 12, 9).unwrap();
    // Personal Year 2020 = 9 + 3 + 4 = 16 -> 7; + March (3) = 1; + day 15 (6) = 7.
    assert_eq!(personal_year(birth, 2020).value, 7);
    assert_eq!(personal_month(birth, 2020, 3).value, 1);
    assert_eq!(personal_day(birth, 2020, 3, 15).value, 7);
  }

  #[test]
  fn universal_day_reduces_the_reference_date() {
    // 2020-03-15 -> 4 + 3 + 6 = 13 -> 4.
    assert_eq!(universal_day(BirthDate::new(2020, 3, 15).unwrap()).value, 4);
  }
}
