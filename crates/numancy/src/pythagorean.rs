//! Pythagorean (Western) name-based calculations.

use crate::error::NumerologyError;
use crate::name::{distinct_present, missing_digits, most_frequent, NameAnalysis};
use crate::reduction::{reduce, CalculatedNumber, PYTHAGOREAN_MASTERS};
use crate::System;

fn analysis(name: &str) -> Result<NameAnalysis, NumerologyError> {
  NameAnalysis::new(name, System::Pythagorean)
}

/// Expression (Destiny) number: reduction of all letters in the full name.
///
/// # Errors
/// Returns [`NumerologyError::EmptyName`] if the name has no usable letters.
pub fn expression_number(name: &str) -> Result<CalculatedNumber, NumerologyError> {
  Ok(reduce(analysis(name)?.total_sum(), PYTHAGOREAN_MASTERS))
}

/// Alias for [`expression_number`].
///
/// # Errors
/// See [`expression_number`].
pub fn destiny_number(name: &str) -> Result<CalculatedNumber, NumerologyError> {
  expression_number(name)
}

/// Motivation (Soul Urge / Heart's Desire) number: reduction of the vowels.
///
/// # Errors
/// Returns [`NumerologyError::EmptyName`] if the name has no usable letters.
pub fn motivation_number(name: &str) -> Result<CalculatedNumber, NumerologyError> {
  Ok(reduce(analysis(name)?.vowel_sum(), PYTHAGOREAN_MASTERS))
}

/// Alias for [`motivation_number`].
///
/// # Errors
/// See [`motivation_number`].
pub fn soul_urge_number(name: &str) -> Result<CalculatedNumber, NumerologyError> {
  motivation_number(name)
}

/// Impression (Personality) number: reduction of the consonants.
///
/// # Errors
/// Returns [`NumerologyError::EmptyName`] if the name has no usable letters.
pub fn impression_number(name: &str) -> Result<CalculatedNumber, NumerologyError> {
  Ok(reduce(analysis(name)?.consonant_sum(), PYTHAGOREAN_MASTERS))
}

/// Alias for [`impression_number`].
///
/// # Errors
/// See [`impression_number`].
pub fn personality_number(name: &str) -> Result<CalculatedNumber, NumerologyError> {
  impression_number(name)
}

/// Hidden Passion: the digit(s) that appear most often in the name.
///
/// # Errors
/// Returns [`NumerologyError::EmptyName`] if the name has no usable letters.
pub fn hidden_passion_numbers(name: &str) -> Result<Vec<u8>, NumerologyError> {
  Ok(most_frequent(&analysis(name)?.digit_frequency()))
}

/// Karmic Lessons: digits `1..=9` absent from the name.
///
/// # Errors
/// Returns [`NumerologyError::EmptyName`] if the name has no usable letters.
pub fn karmic_lessons(name: &str) -> Result<Vec<u8>, NumerologyError> {
  Ok(missing_digits(&analysis(name)?.digit_frequency(), 9))
}

/// Subconscious Self: count of distinct digits `1..=9` present in the name.
///
/// # Errors
/// Returns [`NumerologyError::EmptyName`] if the name has no usable letters.
pub fn subconscious_self_number(name: &str) -> Result<u32, NumerologyError> {
  Ok(distinct_present(&analysis(name)?.digit_frequency()))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vowels_and_consonants_split() {
    // Grace Hopper (compiler pioneer): Expression 4, Soul Urge 8, Personality 5.
    assert_eq!(expression_number("Grace Hopper").unwrap().value, 4);
    assert_eq!(motivation_number("Grace Hopper").unwrap().value, 8);
    assert_eq!(impression_number("Grace Hopper").unwrap().value, 5);
  }

  #[test]
  fn expression_can_be_a_master_number() {
    // Annie Easley (NASA): her Pythagorean Expression is the master number 11.
    let cn = expression_number("Annie Easley").unwrap();
    assert_eq!(cn.value, 11);
    assert!(cn.is_master);
  }

  #[test]
  fn expression_can_carry_a_karmic_debt() {
    // Grace Hopper: letters total 49 -> 13 -> 4, passing through the debt 13.
    let cn = expression_number("Grace Hopper").unwrap();
    assert_eq!(cn.value, 4);
    assert!(cn.passes_through(13));
  }

  #[test]
  fn aliases_match_their_primary_names() {
    assert_eq!(
      destiny_number("Grace Hopper").unwrap(),
      expression_number("Grace Hopper").unwrap()
    );
    assert_eq!(
      soul_urge_number("Grace Hopper").unwrap(),
      motivation_number("Grace Hopper").unwrap()
    );
    assert_eq!(
      personality_number("Grace Hopper").unwrap(),
      impression_number("Grace Hopper").unwrap()
    );
  }

  #[test]
  fn hidden_passion_karmic_and_subconscious() {
    assert_eq!(hidden_passion_numbers("Grace Hopper").unwrap(), vec![7]);
    assert_eq!(karmic_lessons("Grace Hopper").unwrap(), vec![2, 4]);
    assert_eq!(subconscious_self_number("Grace Hopper").unwrap(), 7);
  }

  #[test]
  fn accents_are_ignored_by_the_pythagorean_system() {
    // Karen Spärck Jones: the "ä" is treated as a plain "a".
    assert_eq!(
      expression_number("Karen Spärck Jones").unwrap(),
      expression_number("Karen Sparck Jones").unwrap()
    );
  }

  #[test]
  fn empty_name_errors() {
    assert_eq!(expression_number("123"), Err(NumerologyError::EmptyName));
    assert_eq!(motivation_number("  "), Err(NumerologyError::EmptyName));
    assert_eq!(karmic_lessons("!!!"), Err(NumerologyError::EmptyName));
  }
}
