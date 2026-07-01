//! Numeric reduction (digital root) with optional master-number preservation.
//!
//! Every numerology number is derived by summing values and then repeatedly
//! summing digits until a single digit remains, unless the running value is a
//! preserved "master number". This module records the full reduction trail so
//! callers can inspect intermediate values (needed for karmic-debt detection).

/// Master numbers preserved by the Pythagorean system.
pub const PYTHAGOREAN_MASTERS: &[u32] = &[11, 22, 33];

/// Master numbers preserved by the cabalistic system.
pub const CABALISTIC_MASTERS: &[u32] = &[11, 22];

/// Compound numbers traditionally treated as karmic debts.
pub const KARMIC_DEBTS: &[u32] = &[13, 14, 16, 19];

/// A number together with the trail of how it was reduced.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CalculatedNumber {
  /// The initial total before any reduction.
  pub raw: u32,
  /// The final reduced value (single digit, or a preserved master number).
  pub value: u32,
  /// Whether `value` is a preserved master number.
  pub is_master: bool,
  /// Every value visited during reduction, starting with `raw`.
  pub steps: Vec<u32>,
}

impl CalculatedNumber {
  /// Whether the reduction passed through `n` at any step.
  #[must_use]
  pub fn passes_through(&self, n: u32) -> bool {
    self.steps.contains(&n)
  }
}

/// Sum of the decimal digits of `n`.
#[must_use]
pub fn digit_sum(mut n: u32) -> u32 {
  let mut sum = 0;
  while n > 0 {
    sum += n % 10;
    n /= 10;
  }
  sum
}

/// Reduce `raw` to a single digit, stopping early on any value in `masters`.
#[must_use]
pub fn reduce(raw: u32, masters: &[u32]) -> CalculatedNumber {
  let mut steps = vec![raw];
  let mut cur = raw;
  while cur > 9 && !masters.contains(&cur) {
    cur = digit_sum(cur);
    steps.push(cur);
  }
  let is_master = cur > 9 && masters.contains(&cur);
  CalculatedNumber {
    raw,
    value: cur,
    is_master,
    steps,
  }
}

/// Reduce `raw` all the way to a single digit (no master numbers preserved).
#[must_use]
pub fn reduce_single(raw: u32) -> CalculatedNumber {
  reduce(raw, &[])
}

/// The karmic debt a number carries, if its reduction passes through one.
#[must_use]
pub fn debt_of(cn: &CalculatedNumber) -> Option<u32> {
  cn.steps.iter().copied().find(|s| KARMIC_DEBTS.contains(s))
}

/// A karmic debt detected at a named chart position.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KarmicDebt {
  /// Human-readable name of the chart position carrying the debt.
  pub position: &'static str,
  /// The compound debt value (13, 14, 16 or 19).
  pub debt: u32,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn digit_sum_adds_digits() {
    assert_eq!(digit_sum(0), 0);
    assert_eq!(digit_sum(28), 10);
    assert_eq!(digit_sum(499), 22);
    assert_eq!(digit_sum(999_999), 54);
  }

  #[test]
  fn reduce_to_single_digit() {
    let cn = reduce_single(28);
    assert_eq!(cn.value, 1);
    assert_eq!(cn.steps, vec![28, 10, 1]);
    assert!(!cn.is_master);
  }

  #[test]
  fn single_digit_input_is_left_untouched() {
    let cn = reduce_single(7);
    assert_eq!(cn.value, 7);
    assert_eq!(cn.steps, vec![7]);
    assert!(!cn.is_master);
  }

  #[test]
  fn reduce_preserves_master_numbers() {
    // 499 -> 22 (cabalistic keeps 22).
    let cn = reduce(499, CABALISTIC_MASTERS);
    assert_eq!(cn.value, 22);
    assert!(cn.is_master);

    // 38 -> 11 (both systems keep 11).
    let cn = reduce(38, PYTHAGOREAN_MASTERS);
    assert_eq!(cn.value, 11);
    assert!(cn.is_master);

    // 33 is a master only in the Pythagorean system.
    assert_eq!(reduce(33, PYTHAGOREAN_MASTERS).value, 33);
    assert_eq!(reduce(33, CABALISTIC_MASTERS).value, 6);
  }

  #[test]
  fn reduce_single_never_keeps_masters() {
    let cn = reduce_single(38);
    assert_eq!(cn.value, 2);
    assert!(!cn.is_master);
  }

  #[test]
  fn detects_karmic_debt_in_trail() {
    for debt in [13u32, 14, 16, 19] {
      let cn = reduce(debt, CABALISTIC_MASTERS);
      assert_eq!(debt_of(&cn), Some(debt), "debt {debt} should be detected");
      assert!(cn.passes_through(debt));
    }

    let clean = reduce(25, PYTHAGOREAN_MASTERS);
    assert_eq!(debt_of(&clean), None);
    assert!(!clean.passes_through(16));
  }
}
