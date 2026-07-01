//! Relationship harmony, harmonic numbers and favorable days.
//!
//! These are interpretive lookup tables rather than derived formulas. They are
//! seeded from a documented reference table for the number 7. Entries for other
//! numbers can be added as their source tables become available; unknown
//! numbers return `None` instead of a guessed value.

/// How a governing number relates to the other numbers in a relationship.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RelationshipHarmony {
  /// The governing number.
  pub number: u8,
  /// Numbers with strong attraction/passion.
  pub vibrates_with: Vec<u8>,
  /// Most compatible numbers.
  pub attracts: Vec<u8>,
  /// Opposite numbers.
  pub opposite_to: Vec<u8>,
  /// Passive/friendship numbers.
  pub passive_with: Vec<u8>,
}

/// Relationship harmony for a governing number, if known.
#[must_use]
pub fn relationship_harmony(number: u8) -> Option<RelationshipHarmony> {
  match number {
    7 => Some(RelationshipHarmony {
      number: 7,
      vibrates_with: vec![3],
      attracts: vec![2, 6],
      opposite_to: vec![1, 9],
      passive_with: vec![4, 5, 8],
    }),
    _ => None,
  }
}

/// Harmonic numbers for a birth/psychic number, if known.
#[must_use]
pub fn harmonic_numbers(day_number: u8) -> Option<Vec<u8>> {
  match day_number {
    7 => Some(vec![2, 4, 5, 7]),
    _ => None,
  }
}

/// Favorable days of the month for a birth/psychic number, if known.
#[must_use]
pub fn favorable_days(day_number: u8) -> Option<Vec<u8>> {
  match day_number {
    7 => Some(vec![2, 7, 14, 16, 23, 25]),
    _ => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn number_seven_matches_reference_table() {
    let harmony = relationship_harmony(7).unwrap();
    assert_eq!(harmony.number, 7);
    assert_eq!(harmony.vibrates_with, vec![3]);
    assert_eq!(harmony.attracts, vec![2, 6]);
    assert_eq!(harmony.opposite_to, vec![1, 9]);
    assert_eq!(harmony.passive_with, vec![4, 5, 8]);

    assert_eq!(harmonic_numbers(7), Some(vec![2, 4, 5, 7]));
    assert_eq!(favorable_days(7), Some(vec![2, 7, 14, 16, 23, 25]));
  }

  #[test]
  fn unknown_numbers_return_none() {
    for n in [0u8, 1, 2, 3, 4, 5, 6, 8, 9] {
      assert!(relationship_harmony(n).is_none(), "harmony {n}");
      assert!(harmonic_numbers(n).is_none(), "harmonic {n}");
      assert!(favorable_days(n).is_none(), "favorable {n}");
    }
  }
}
