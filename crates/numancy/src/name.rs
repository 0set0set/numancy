//! Parsing a full name into per-letter values for a given system.

use crate::alphabet::{decode, is_vowel, Accent};
use crate::error::NumerologyError;
use crate::System;

/// A single decoded letter of a name, with its computed value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetterValue {
  /// The original character as written.
  pub original: char,
  /// Base letter in `A..=Z`.
  pub base: char,
  /// Accent on the letter, if any.
  pub accent: Option<Accent>,
  /// Whether this letter counts as a vowel.
  pub is_vowel: bool,
  /// Numeric value under the active system.
  pub value: u8,
}

/// One whitespace/hyphen separated word of a name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameToken {
  /// The letters composing the word.
  pub letters: Vec<LetterValue>,
}

/// A full name decomposed into tokens and per-letter values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameAnalysis {
  /// The system used to value the letters.
  pub system: System,
  /// Words of the name, in order.
  pub tokens: Vec<NameToken>,
}

impl NameAnalysis {
  /// Build an analysis from a name under `system`.
  ///
  /// # Errors
  /// Returns [`NumerologyError::EmptyName`] if no usable letters are found.
  pub fn new(name: &str, system: System) -> Result<Self, NumerologyError> {
    let mut tokens = Vec::new();
    for word in name.split(|c: char| c.is_whitespace() || c == '-') {
      let mut letters = Vec::new();
      for ch in word.chars() {
        if let Some(d) = decode(ch) {
          letters.push(LetterValue {
            original: ch,
            base: d.base,
            accent: d.accent,
            is_vowel: is_vowel(d.base) && !d.cedilla,
            value: system.letter_value(&d),
          });
        }
      }
      if !letters.is_empty() {
        tokens.push(NameToken { letters });
      }
    }
    if tokens.is_empty() {
      return Err(NumerologyError::EmptyName);
    }
    Ok(Self { system, tokens })
  }

  /// Iterate over every letter of the name, across all tokens.
  pub fn letters(&self) -> impl Iterator<Item = &LetterValue> {
    self.tokens.iter().flat_map(|t| t.letters.iter())
  }

  /// Sum of all letter values.
  #[must_use]
  pub fn total_sum(&self) -> u32 {
    self.letters().map(|l| u32::from(l.value)).sum()
  }

  /// Sum of vowel values.
  #[must_use]
  pub fn vowel_sum(&self) -> u32 {
    self.letters().filter(|l| l.is_vowel).map(|l| u32::from(l.value)).sum()
  }

  /// Sum of consonant values.
  #[must_use]
  pub fn consonant_sum(&self) -> u32 {
    self.letters().filter(|l| !l.is_vowel).map(|l| u32::from(l.value)).sum()
  }

  /// Sum of the first letter value of each word (used for hidden talent).
  #[must_use]
  pub fn initials_sum(&self) -> u32 {
    self
      .tokens
      .iter()
      .filter_map(|t| t.letters.first())
      .map(|l| u32::from(l.value))
      .sum()
  }

  /// Count of how many times each digit `0..=9` appears as a letter value.
  #[must_use]
  pub fn digit_frequency(&self) -> [u32; 10] {
    let mut freq = [0u32; 10];
    for l in self.letters() {
      let v = usize::from(l.value);
      if v <= 9 {
        freq[v] += 1;
      }
    }
    freq
  }

  /// All letter values in order (used to build the Life Triangle).
  #[must_use]
  pub fn base_values(&self) -> Vec<u8> {
    self.letters().map(|l| l.value).collect()
  }
}

/// Digits in `1..=max` that never appear in the frequency table.
#[must_use]
pub fn missing_digits(freq: &[u32; 10], max: u8) -> Vec<u8> {
  (1u8..=max).filter(|&d| freq[usize::from(d)] == 0).collect()
}

/// Digits in `1..=9` that appear most often (empty if none appear).
#[must_use]
pub fn most_frequent(freq: &[u32; 10]) -> Vec<u8> {
  let max = (1u8..=9).map(|d| freq[usize::from(d)]).max().unwrap_or(0);
  if max == 0 {
    return Vec::new();
  }
  (1u8..=9).filter(|&d| freq[usize::from(d)] == max).collect()
}

/// Count of distinct digits in `1..=9` present in the frequency table.
#[must_use]
pub fn distinct_present(freq: &[u32; 10]) -> u32 {
  let count = (1u8..=9).filter(|&d| freq[usize::from(d)] > 0).count();
  u32::try_from(count).unwrap_or(0)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn splits_tokens_and_skips_punctuation() {
    // Whitespace and hyphens both split; punctuation is dropped.
    let a = NameAnalysis::new("Ada Lovelace-King!", System::Cabalistic).unwrap();
    assert_eq!(a.tokens.len(), 3);
    // Accents do not create extra tokens.
    let b = NameAnalysis::new("Karen Spärck Jones", System::Cabalistic).unwrap();
    assert_eq!(b.tokens.len(), 3);
  }

  #[test]
  fn empty_name_errors() {
    assert_eq!(
      NameAnalysis::new("  123 !!", System::Pythagorean),
      Err(NumerologyError::EmptyName)
    );
    assert_eq!(
      NameAnalysis::new("", System::Cabalistic),
      Err(NumerologyError::EmptyName)
    );
  }

  #[test]
  fn sums_split_into_vowels_and_consonants() {
    // "Ada Lovelace" cabalistic: A1 D4 A1 | L3 O7 V6 E5 L3 A1 C3 E5.
    let a = NameAnalysis::new("Ada Lovelace", System::Cabalistic).unwrap();
    assert_eq!(a.total_sum(), 39);
    assert_eq!(a.vowel_sum(), 20);
    assert_eq!(a.consonant_sum(), 19);
    // Total is always vowels + consonants.
    assert_eq!(a.total_sum(), a.vowel_sum() + a.consonant_sum());
  }

  #[test]
  fn initials_sum_uses_first_letter_of_each_token() {
    // First letters A and L -> 1 + 3 = 4.
    let a = NameAnalysis::new("Ada Lovelace", System::Cabalistic).unwrap();
    assert_eq!(a.initials_sum(), 4);
  }

  #[test]
  fn base_values_preserve_letter_order() {
    let a = NameAnalysis::new("Ada Lovelace", System::Cabalistic).unwrap();
    assert_eq!(a.base_values(), vec![1, 4, 1, 3, 7, 6, 5, 3, 1, 3, 5]);
  }

  #[test]
  fn frequency_helpers() {
    // "Ada Lovelace": 1 and 3 each appear three times.
    let a = NameAnalysis::new("Ada Lovelace", System::Cabalistic).unwrap();
    let freq = a.digit_frequency();
    assert_eq!(freq[1], 3);
    assert_eq!(freq[3], 3);
    assert_eq!(missing_digits(&freq, 9), vec![2, 8, 9]);
    assert_eq!(most_frequent(&freq), vec![1, 3]);
    assert_eq!(distinct_present(&freq), 6);
  }

  #[test]
  fn most_frequent_can_return_a_tie() {
    // "Ada Lovelace" ties 1 and 3 as the most frequent values.
    let a = NameAnalysis::new("Ada Lovelace", System::Cabalistic).unwrap();
    assert_eq!(most_frequent(&a.digit_frequency()), vec![1, 3]);
  }
}
