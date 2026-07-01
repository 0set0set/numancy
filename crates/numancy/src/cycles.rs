//! Life cycles, challenges and decision moments (cabalistic timeline).
//!
//! These are computed from the reduced month, day and year. Life cycles and
//! decision moments preserve master numbers; challenges always reduce to a
//! single digit (they are differences, so `0` is a valid result).

use crate::date::BirthDate;
use crate::reduction::{reduce, reduce_single, CalculatedNumber, CABALISTIC_MASTERS};

/// The three Life Cycles: reduced month, day and year (masters preserved).
#[must_use]
pub fn life_cycles(birth: BirthDate) -> [CalculatedNumber; 3] {
  [
    reduce(birth.month, CABALISTIC_MASTERS),
    reduce(birth.day, CABALISTIC_MASTERS),
    reduce(birth.year, CABALISTIC_MASTERS),
  ]
}

/// The three Challenges: `|m-d|`, `|d-y|`, and `|c1-c2|` (single digits).
#[must_use]
pub fn challenges(birth: BirthDate) -> [u32; 3] {
  let m = reduce_single(birth.month).value;
  let d = reduce_single(birth.day).value;
  let y = reduce_single(birth.year).value;
  let c1 = m.abs_diff(d);
  let c2 = d.abs_diff(y);
  let c3 = c1.abs_diff(c2);
  [c1, c2, c3]
}

/// The four Decision Moments (pinnacles), preserving master numbers.
#[must_use]
pub fn decision_moments(birth: BirthDate) -> [CalculatedNumber; 4] {
  let m = reduce_single(birth.month).value;
  let d = reduce_single(birth.day).value;
  let y = reduce_single(birth.year).value;
  let m1 = reduce(m + d, CABALISTIC_MASTERS);
  let m2 = reduce(d + y, CABALISTIC_MASTERS);
  let m3 = reduce(
    reduce_single(m1.value).value + reduce_single(m2.value).value,
    CABALISTIC_MASTERS,
  );
  let m4 = reduce(m + y, CABALISTIC_MASTERS);
  [m1, m2, m3, m4]
}

#[cfg(test)]
mod tests {
  use super::*;

  // Fixture: Barbara Liskov, born 1939-11-07.
  fn birth() -> BirthDate {
    BirthDate::new(1939, 11, 7).unwrap()
  }

  #[test]
  fn life_cycles_reduce_month_day_year() {
    // Month 11 -> 11 (master), day 7 -> 7, year 1939 -> 22 (master).
    let cycles = life_cycles(birth());
    assert_eq!([cycles[0].value, cycles[1].value, cycles[2].value], [11, 7, 22]);
    assert!(cycles[0].is_master);
    assert!(!cycles[1].is_master);
    assert!(cycles[2].is_master);
  }

  #[test]
  fn challenges_are_absolute_differences() {
    // m=2 d=7 y=4 -> |2-7|=5, |7-4|=3, |5-3|=2.
    assert_eq!(challenges(birth()), [5, 3, 2]);
  }

  #[test]
  fn challenge_of_zero_is_valid() {
    // Katherine Johnson (1918-08-26): reduced month and day are equal, so the
    // first challenge is zero.
    let katherine = BirthDate::new(1918, 8, 26).unwrap();
    assert_eq!(challenges(katherine), [0, 7, 7]);
  }

  #[test]
  fn decision_moments_preserve_masters() {
    // m=2 d=7 y=4 -> 9, 11, 11, 6.
    let moments = decision_moments(birth());
    let values = [moments[0].value, moments[1].value, moments[2].value, moments[3].value];
    assert_eq!(values, [9, 11, 11, 6]);
    assert!(!moments[0].is_master);
    assert!(moments[1].is_master);
    assert!(moments[2].is_master);
    assert!(!moments[3].is_master);
  }
}
