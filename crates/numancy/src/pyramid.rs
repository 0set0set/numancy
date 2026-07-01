//! Inverted Life Triangle, arcana and negative sequences.
//!
//! The base row holds the cabalistic values of every letter of the name. Each
//! following row sums adjacent pairs (reduced to a single digit) until a single
//! apex value remains: the regent arcane. Adjacent pairs of the base row form
//! the dominant arcana (two-digit codes such as 31, 16, 63).

use crate::error::NumerologyError;
use crate::name::NameAnalysis;
use crate::reduction::reduce_single;
use crate::System;

/// A two-digit arcane formed by a pair of adjacent base values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArcaneCode(pub u8, pub u8);

impl ArcaneCode {
  /// The arcane as a two-digit code (e.g. `ArcaneCode(3, 1)` -> `31`).
  #[must_use]
  pub fn code(self) -> u8 {
    self.0 * 10 + self.1
  }
}

/// A run of three or more identical values in a single row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NegativeSequence {
  /// Row index within the triangle.
  pub row: usize,
  /// Start column of the run.
  pub start: usize,
  /// The repeated digit.
  pub digit: u8,
  /// Length of the run.
  pub len: usize,
}

/// The full inverted Life Triangle for a name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LifeTriangle {
  /// Base row: cabalistic value of every letter.
  pub base: Vec<u8>,
  /// Dominant arcana from adjacent base pairs.
  pub arcana: Vec<ArcaneCode>,
  /// Every row of the triangle, from base to apex.
  pub rows: Vec<Vec<u8>>,
  /// The apex value (regent arcane).
  pub regent: u8,
  /// Any negative sequences (three or more equal values in a row).
  pub sequences: Vec<NegativeSequence>,
}

impl LifeTriangle {
  /// Index of the dominant arcane at `age`, spreading arcana over `life_span`.
  #[must_use]
  pub fn dominant_index(&self, age: u32, life_span: u32) -> Option<usize> {
    if self.arcana.is_empty() || life_span == 0 {
      return None;
    }
    let per = f64::from(life_span) / self.arcana.len() as f64;
    let idx = (f64::from(age) / per).floor() as usize;
    Some(idx.min(self.arcana.len() - 1))
  }

  /// The dominant arcane at `age`.
  #[must_use]
  pub fn dominant(&self, age: u32, life_span: u32) -> Option<ArcaneCode> {
    self.dominant_index(age, life_span).map(|i| self.arcana[i])
  }
}

/// Build the Life Triangle for `name` using cabalistic letter values.
///
/// # Errors
/// Returns [`NumerologyError::EmptyName`] if the name has fewer than two
/// usable letters.
pub fn life_triangle(name: &str) -> Result<LifeTriangle, NumerologyError> {
  let analysis = NameAnalysis::new(name, System::Cabalistic)?;
  build(analysis.base_values())
}

fn build(base: Vec<u8>) -> Result<LifeTriangle, NumerologyError> {
  if base.len() < 2 {
    return Err(NumerologyError::EmptyName);
  }
  let arcana: Vec<ArcaneCode> = base.windows(2).map(|w| ArcaneCode(w[0], w[1])).collect();

  let mut rows = vec![base.clone()];
  loop {
    let prev = rows.last().expect("rows is never empty");
    if prev.len() <= 1 {
      break;
    }
    let next: Vec<u8> = prev
      .windows(2)
      .map(|w| reduce_single(u32::from(w[0]) + u32::from(w[1])).value as u8)
      .collect();
    rows.push(next);
  }

  let regent = rows.last().and_then(|r| r.first()).copied().unwrap_or(0);
  let sequences = find_sequences(&rows);

  Ok(LifeTriangle {
    base,
    arcana,
    rows,
    regent,
    sequences,
  })
}

fn find_sequences(rows: &[Vec<u8>]) -> Vec<NegativeSequence> {
  let mut sequences = Vec::new();
  for (row_idx, row) in rows.iter().enumerate() {
    let mut start = 0;
    while start < row.len() {
      let mut end = start + 1;
      while end < row.len() && row[end] == row[start] {
        end += 1;
      }
      let len = end - start;
      if len >= 3 {
        sequences.push(NegativeSequence {
          row: row_idx,
          start,
          digit: row[start],
          len,
        });
      }
      start = end;
    }
  }
  sequences
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn arcane_code_packs_two_digits() {
    assert_eq!(ArcaneCode(3, 1).code(), 31);
    assert_eq!(ArcaneCode(7, 7).code(), 77);
  }

  #[test]
  fn triangle_builds_from_letter_values() {
    // Barbara Liskov: 13 letters give 12 arcana from adjacent pairs.
    let triangle = life_triangle("Barbara Liskov").unwrap();
    assert_eq!(triangle.base, vec![2, 1, 2, 2, 1, 2, 1, 3, 1, 3, 2, 7, 6]);
    let codes: Vec<u8> = triangle.arcana.iter().map(|a| a.code()).collect();
    assert_eq!(codes, vec![21, 12, 22, 21, 12, 21, 13, 31, 13, 32, 27, 76]);
    assert_eq!(triangle.regent, 7);
  }

  #[test]
  fn dominant_arcane_by_age() {
    let triangle = life_triangle("Barbara Liskov").unwrap();
    // 12 arcana over 90 years -> 7.5 each.
    assert_eq!(triangle.dominant(10, 90).map(ArcaneCode::code), Some(12));
    assert_eq!(triangle.dominant(30, 90).map(ArcaneCode::code), Some(12));
    assert_eq!(triangle.dominant(55, 90).map(ArcaneCode::code), Some(31));
    // Ages beyond the life span clamp to the last arcane.
    assert_eq!(triangle.dominant(200, 90).map(ArcaneCode::code), Some(76));
  }

  #[test]
  fn dominant_is_none_for_zero_life_span() {
    let triangle = life_triangle("Barbara Liskov").unwrap();
    assert_eq!(triangle.dominant(30, 0), None);
  }

  #[test]
  fn single_letter_name_has_no_triangle() {
    assert!(life_triangle("A").is_err());
  }

  #[test]
  fn detects_repeated_sequences() {
    let triangle = build(vec![1, 1, 1, 2]).unwrap();
    assert_eq!(triangle.sequences.len(), 1);
    assert_eq!(triangle.sequences[0].digit, 1);
    assert_eq!(triangle.sequences[0].len, 3);
  }

  #[test]
  fn no_sequences_without_a_run_of_three() {
    let triangle = build(vec![3, 7, 5, 3, 4]).unwrap();
    assert!(triangle.sequences.is_empty());
  }
}
