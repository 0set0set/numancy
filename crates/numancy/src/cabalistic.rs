//! Brazilian cabalistic numerology calculations.
//!
//! This mirrors the sections of a Brazilian numerology map. Names use the
//! `1..8` cabalistic table (see [`crate::alphabet`]); master numbers `11` and
//! `22` are preserved.

use crate::date::BirthDate;
use crate::error::NumerologyError;
use crate::name::{distinct_present, missing_digits, most_frequent, NameAnalysis};
use crate::reduction::{debt_of, digit_sum, reduce, CalculatedNumber, KarmicDebt, CABALISTIC_MASTERS};
use crate::System;

fn analysis(name: &str) -> Result<NameAnalysis, NumerologyError> {
  NameAnalysis::new(name, System::Cabalistic)
}

/// Motivation number: reduction of the vowels (inner self / soul).
///
/// # Errors
/// Returns [`NumerologyError::EmptyName`] if the name has no usable letters.
pub fn motivation_number(name: &str) -> Result<CalculatedNumber, NumerologyError> {
  Ok(reduce(analysis(name)?.vowel_sum(), CABALISTIC_MASTERS))
}

/// Impression number: reduction of the consonants (ego / outer image).
///
/// Impression is always reduced to a single digit (no master numbers).
///
/// # Errors
/// Returns [`NumerologyError::EmptyName`] if the name has no usable letters.
pub fn impression_number(name: &str) -> Result<CalculatedNumber, NumerologyError> {
  Ok(reduce(analysis(name)?.consonant_sum(), &[]))
}

/// Expression number: reduction of every letter of the name.
///
/// # Errors
/// Returns [`NumerologyError::EmptyName`] if the name has no usable letters.
pub fn expression_number(name: &str) -> Result<CalculatedNumber, NumerologyError> {
  Ok(reduce(analysis(name)?.total_sum(), CABALISTIC_MASTERS))
}

/// Hidden Talent number: reduction of the first letter of each name part.
///
/// # Errors
/// Returns [`NumerologyError::EmptyName`] if the name has no usable letters.
pub fn hidden_talent_number(name: &str) -> Result<CalculatedNumber, NumerologyError> {
  Ok(reduce(analysis(name)?.initials_sum(), CABALISTIC_MASTERS))
}

/// Destiny number: reduction of the full birth date.
#[must_use]
pub fn destiny_number(birth: BirthDate) -> CalculatedNumber {
  reduce(birth.digit_total(), CABALISTIC_MASTERS)
}

/// Birthday number: the day of birth, reduced.
#[must_use]
pub fn birth_day_number(birth: BirthDate) -> CalculatedNumber {
  reduce(birth.day, CABALISTIC_MASTERS)
}

/// Psychic number: the reduced day of birth (self-perception).
#[must_use]
pub fn psychic_number(birth: BirthDate) -> CalculatedNumber {
  reduce(birth.day, CABALISTIC_MASTERS)
}

/// Mission number: reduction of Expression + Destiny.
///
/// # Errors
/// Returns [`NumerologyError::EmptyName`] if the name has no usable letters.
pub fn mission_number(name: &str, birth: BirthDate) -> Result<CalculatedNumber, NumerologyError> {
  let expression = expression_number(name)?;
  let destiny = destiny_number(birth);
  Ok(reduce(expression.value + destiny.value, CABALISTIC_MASTERS))
}

/// Karmic Lessons: digits `1..=9` absent from the name. Because the cabalistic
/// table only reaches `8`, digit `9` is always reported as a lesson.
///
/// # Errors
/// Returns [`NumerologyError::EmptyName`] if the name has no usable letters.
pub fn karmic_lessons(name: &str) -> Result<Vec<u8>, NumerologyError> {
  Ok(missing_digits(&analysis(name)?.digit_frequency(), 9))
}

/// Hidden Tendencies: the digit(s) that appear most often in the name.
///
/// # Errors
/// Returns [`NumerologyError::EmptyName`] if the name has no usable letters.
pub fn hidden_tendencies(name: &str) -> Result<Vec<u8>, NumerologyError> {
  Ok(most_frequent(&analysis(name)?.digit_frequency()))
}

/// Subconscious Response: count of distinct digits present in the name.
///
/// # Errors
/// Returns [`NumerologyError::EmptyName`] if the name has no usable letters.
pub fn subconscious_response(name: &str) -> Result<u32, NumerologyError> {
  Ok(distinct_present(&analysis(name)?.digit_frequency()))
}

/// Karmic Debts: compound numbers 13/14/16/19 found while reducing core
/// positions (motivation, impression, expression, destiny, mission, birthday).
///
/// # Errors
/// Returns [`NumerologyError::EmptyName`] if the name has no usable letters.
pub fn karmic_debts(name: &str, birth: BirthDate) -> Result<Vec<KarmicDebt>, NumerologyError> {
  let positions = [
    ("motivation", motivation_number(name)?),
    ("impression", impression_number(name)?),
    ("expression", expression_number(name)?),
    ("destiny", destiny_number(birth)),
    ("mission", mission_number(name, birth)?),
    ("birthday", birth_day_number(birth)),
  ];
  let mut debts = Vec::new();
  for (position, cn) in positions {
    if let Some(debt) = debt_of(&cn) {
      debts.push(KarmicDebt { position, debt });
    }
  }
  Ok(debts)
}

/// Personal Year: birth day + birth month + reference year (masters preserved).
#[must_use]
pub fn personal_year(birth: BirthDate, year: u32) -> CalculatedNumber {
  reduce(
    digit_sum(birth.day) + digit_sum(birth.month) + digit_sum(year),
    CABALISTIC_MASTERS,
  )
}

/// Personal Month: Personal Year + calendar month (masters preserved).
#[must_use]
pub fn personal_month(birth: BirthDate, year: u32, month: u32) -> CalculatedNumber {
  reduce(personal_year(birth, year).value + digit_sum(month), CABALISTIC_MASTERS)
}

/// Personal Day: Personal Month + calendar day (masters preserved).
#[must_use]
pub fn personal_day(birth: BirthDate, year: u32, month: u32, day: u32) -> CalculatedNumber {
  reduce(
    personal_month(birth, year, month).value + digit_sum(day),
    CABALISTIC_MASTERS,
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  // Fixture: Barbara Liskov (Turing Award), born 1939-11-07.
  const NAME: &str = "Barbara Liskov";
  fn birth() -> BirthDate {
    BirthDate::new(1939, 11, 7).unwrap()
  }

  #[test]
  fn name_numbers() {
    // Vowels total 11 (master), consonants 22 -> 4, all letters 33 -> 6.
    let motivation = motivation_number(NAME).unwrap();
    assert_eq!(motivation.value, 11);
    assert!(motivation.is_master);
    assert_eq!(impression_number(NAME).unwrap().value, 4);
    assert_eq!(expression_number(NAME).unwrap().value, 6);
    assert_eq!(hidden_talent_number(NAME).unwrap().value, 5);
  }

  #[test]
  fn date_numbers() {
    // 1939-11-07: digit total 22 + 2 + 7 = 31 -> 4.
    assert_eq!(destiny_number(birth()).value, 4);
    assert_eq!(birth_day_number(birth()).value, 7);
    assert_eq!(psychic_number(birth()).value, 7);
    // Mission = Expression(6) + Destiny(4) = 10 -> 1.
    assert_eq!(mission_number(NAME, birth()).unwrap().value, 1);
  }

  #[test]
  fn impression_never_keeps_masters() {
    // Impression always reduces to a single digit (no master preservation),
    // even though Barbara Liskov's consonants sum to the master-looking 22.
    let cn = impression_number(NAME).unwrap();
    assert_eq!(cn.value, 4);
    assert!(!cn.is_master);
  }

  #[test]
  fn karmic_and_tendency() {
    assert_eq!(karmic_lessons(NAME).unwrap(), vec![4, 5, 8, 9]);
    assert_eq!(hidden_tendencies(NAME).unwrap(), vec![2]);
    assert_eq!(subconscious_response(NAME).unwrap(), 5);
  }

  #[test]
  fn digit_nine_is_always_a_karmic_lesson() {
    // The cabalistic table only reaches 8, so 9 can never appear as a value.
    assert!(karmic_lessons("Grace Hopper").unwrap().contains(&9));
  }

  #[test]
  fn karmic_debt_detected_on_expression() {
    // Grace Hopper: her name totals 49 -> 13 (a karmic debt) -> 4.
    let debts = karmic_debts("Grace Hopper", BirthDate::new(1906, 12, 9).unwrap()).unwrap();
    assert_eq!(debts.len(), 1);
    assert_eq!(debts[0].position, "expression");
    assert_eq!(debts[0].debt, 13);
  }

  #[test]
  fn clean_name_has_no_karmic_debts() {
    assert!(karmic_debts(NAME, birth()).unwrap().is_empty());
  }

  #[test]
  fn personal_cycles_preserve_masters() {
    // Barbara Liskov's Personal Year for 2020 = day(7) + month(2) + year(4) = 13 -> 4.
    assert_eq!(personal_year(birth(), 2020).value, 4);
    // Katherine Johnson (1918-08-26): her Personal Day on 2020-03-15 is a master 11.
    let katherine = BirthDate::new(1918, 8, 26).unwrap();
    let day = personal_day(katherine, 2020, 3, 15);
    assert_eq!(day.value, 11);
    assert!(day.is_master);
  }

  #[test]
  fn empty_name_errors() {
    assert_eq!(expression_number("123"), Err(NumerologyError::EmptyName));
  }
}
