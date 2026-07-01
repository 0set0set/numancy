//! Aggregated profiles that compute a full chart for each system.

use crate::cycles::{challenges, decision_moments, life_cycles};
use crate::date::{self, BirthDate, ReferenceDate};
use crate::error::NumerologyError;
use crate::pyramid::{life_triangle, LifeTriangle};
use crate::reduction::{debt_of, CalculatedNumber, KarmicDebt};
use crate::{address, cabalistic, pythagorean};

/// A complete Pythagorean chart for a person.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PythagoreanChart {
  /// Expression (Destiny) number.
  pub expression: CalculatedNumber,
  /// Soul Urge (Motivation) number.
  pub soul_urge: CalculatedNumber,
  /// Personality (Impression) number.
  pub personality: CalculatedNumber,
  /// Life Path number.
  pub life_path: CalculatedNumber,
  /// Birthday number.
  pub birthday: CalculatedNumber,
  /// Maturity number.
  pub maturity: CalculatedNumber,
  /// Personal Year for the reference date.
  pub personal_year: CalculatedNumber,
  /// Personal Month for the reference date.
  pub personal_month: CalculatedNumber,
  /// Personal Day for the reference date.
  pub personal_day: CalculatedNumber,
  /// Universal Day for the reference date.
  pub universal_day: CalculatedNumber,
  /// Hidden Passion digit(s).
  pub hidden_passion: Vec<u8>,
  /// Karmic Lessons (missing digits).
  pub karmic_lessons: Vec<u8>,
  /// Subconscious Self (distinct digits present).
  pub subconscious_self: u32,
  /// Karmic Debts found in core positions.
  pub karmic_debts: Vec<KarmicDebt>,
  /// House number, when an address is supplied.
  pub house: Option<CalculatedNumber>,
}

impl PythagoreanChart {
  /// Compute a full Pythagorean chart.
  ///
  /// # Errors
  /// Returns a [`NumerologyError`] if the name has no usable letters or the
  /// supplied address has no usable characters.
  pub fn new(
    name: &str,
    birth: BirthDate,
    reference: ReferenceDate,
    address_line: Option<&str>,
  ) -> Result<Self, NumerologyError> {
    let expression = pythagorean::expression_number(name)?;
    let soul_urge = pythagorean::motivation_number(name)?;
    let personality = pythagorean::impression_number(name)?;
    let life_path = date::life_path_number(birth);
    let birthday = date::birthday_number(birth);
    let maturity = date::maturity_number(&life_path, &expression);

    let positions = [
      ("expression", &expression),
      ("life path", &life_path),
      ("birthday", &birthday),
    ];
    let mut karmic_debts = Vec::new();
    for (position, cn) in positions {
      if let Some(debt) = debt_of(cn) {
        karmic_debts.push(KarmicDebt { position, debt });
      }
    }

    let house = match address_line {
      Some(line) => Some(address::house_number(line)?),
      None => None,
    };

    Ok(Self {
      expression,
      soul_urge,
      personality,
      life_path,
      birthday,
      maturity,
      personal_year: date::personal_year(birth, reference.year),
      personal_month: date::personal_month(birth, reference.year, reference.month),
      personal_day: date::personal_day(birth, reference.year, reference.month, reference.day),
      universal_day: date::universal_day(reference),
      hidden_passion: pythagorean::hidden_passion_numbers(name)?,
      karmic_lessons: pythagorean::karmic_lessons(name)?,
      subconscious_self: pythagorean::subconscious_self_number(name)?,
      karmic_debts,
      house,
    })
  }
}

/// A complete cabalistic map for a person.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CabalisticMap {
  /// Motivation number.
  pub motivation: CalculatedNumber,
  /// Impression number.
  pub impression: CalculatedNumber,
  /// Expression number.
  pub expression: CalculatedNumber,
  /// Hidden Talent number.
  pub hidden_talent: CalculatedNumber,
  /// Birthday number.
  pub birth_day: CalculatedNumber,
  /// Psychic number.
  pub psychic: CalculatedNumber,
  /// Destiny number.
  pub destiny: CalculatedNumber,
  /// Mission number.
  pub mission: CalculatedNumber,
  /// Karmic Lessons (missing digits).
  pub karmic_lessons: Vec<u8>,
  /// Karmic Debts found in core positions.
  pub karmic_debts: Vec<KarmicDebt>,
  /// Hidden Tendencies (most frequent digits).
  pub hidden_tendencies: Vec<u8>,
  /// Subconscious Response (distinct digits present).
  pub subconscious_response: u32,
  /// The three Life Cycles.
  pub life_cycles: [CalculatedNumber; 3],
  /// The three Challenges.
  pub challenges: [u32; 3],
  /// The four Decision Moments.
  pub decision_moments: [CalculatedNumber; 4],
  /// Personal Year for the reference date.
  pub personal_year: CalculatedNumber,
  /// Personal Month for the reference date.
  pub personal_month: CalculatedNumber,
  /// Personal Day for the reference date.
  pub personal_day: CalculatedNumber,
  /// The inverted Life Triangle.
  pub life_triangle: LifeTriangle,
}

impl CabalisticMap {
  /// Compute a full cabalistic map.
  ///
  /// # Errors
  /// Returns [`NumerologyError::EmptyName`] if the name has no usable letters.
  pub fn new(name: &str, birth: BirthDate, reference: ReferenceDate) -> Result<Self, NumerologyError> {
    Ok(Self {
      motivation: cabalistic::motivation_number(name)?,
      impression: cabalistic::impression_number(name)?,
      expression: cabalistic::expression_number(name)?,
      hidden_talent: cabalistic::hidden_talent_number(name)?,
      birth_day: cabalistic::birth_day_number(birth),
      psychic: cabalistic::psychic_number(birth),
      destiny: cabalistic::destiny_number(birth),
      mission: cabalistic::mission_number(name, birth)?,
      karmic_lessons: cabalistic::karmic_lessons(name)?,
      karmic_debts: cabalistic::karmic_debts(name, birth)?,
      hidden_tendencies: cabalistic::hidden_tendencies(name)?,
      subconscious_response: cabalistic::subconscious_response(name)?,
      life_cycles: life_cycles(birth),
      challenges: challenges(birth),
      decision_moments: decision_moments(birth),
      personal_year: cabalistic::personal_year(birth, reference.year),
      personal_month: cabalistic::personal_month(birth, reference.year, reference.month),
      personal_day: cabalistic::personal_day(birth, reference.year, reference.month, reference.day),
      life_triangle: life_triangle(name)?,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pythagorean_chart_builds() {
    // Ada Lovelace (1815-12-10), reference 2020-03-15.
    let birth = BirthDate::new(1815, 12, 10).unwrap();
    let reference = BirthDate::new(2020, 3, 15).unwrap();
    let chart = PythagoreanChart::new("Ada Lovelace", birth, reference, Some("77")).unwrap();

    assert_eq!(chart.expression.value, 9);
    assert_eq!(chart.soul_urge.value, 1);
    assert_eq!(chart.personality.value, 8);
    assert_eq!(chart.life_path.value, 1);
    assert_eq!(chart.birthday.value, 1);
    assert_eq!(chart.maturity.value, 1);
    assert_eq!(chart.personal_year.value, 8);
    assert_eq!(chart.universal_day.value, 4);
    assert_eq!(chart.house.unwrap().value, 5);

    // Life Path totals 19, so a karmic debt is recorded there.
    assert_eq!(chart.karmic_debts.len(), 1);
    assert_eq!(chart.karmic_debts[0].position, "life path");
    assert_eq!(chart.karmic_debts[0].debt, 19);
  }

  #[test]
  fn pythagorean_chart_preserves_master_life_path() {
    // Dorothy Vaughan (1910-09-20): Life Path is the master number 22.
    let birth = BirthDate::new(1910, 9, 20).unwrap();
    let reference = BirthDate::new(2020, 3, 15).unwrap();
    let chart = PythagoreanChart::new("Dorothy Vaughan", birth, reference, None).unwrap();
    assert_eq!(chart.life_path.value, 22);
    assert!(chart.life_path.is_master);
    assert_eq!(chart.expression.value, 8);
  }

  #[test]
  fn pythagorean_chart_without_address_has_no_house() {
    let birth = BirthDate::new(1906, 12, 9).unwrap();
    let reference = BirthDate::new(2020, 3, 15).unwrap();
    let chart = PythagoreanChart::new("Grace Hopper", birth, reference, None).unwrap();
    assert!(chart.house.is_none());
  }

  #[test]
  fn cabalistic_map_builds_full_profile() {
    // Barbara Liskov (1939-11-07), reference 2020-03-15.
    let birth = BirthDate::new(1939, 11, 7).unwrap();
    let reference = BirthDate::new(2020, 3, 15).unwrap();
    let map = CabalisticMap::new("Barbara Liskov", birth, reference).unwrap();

    assert_eq!(map.motivation.value, 11);
    assert!(map.motivation.is_master);
    assert_eq!(map.impression.value, 4);
    assert_eq!(map.expression.value, 6);
    assert_eq!(map.hidden_talent.value, 5);
    assert_eq!(map.birth_day.value, 7);
    assert_eq!(map.psychic.value, 7);
    assert_eq!(map.destiny.value, 4);
    assert_eq!(map.mission.value, 1);
    assert_eq!(map.karmic_lessons, vec![4, 5, 8, 9]);
    assert_eq!(map.hidden_tendencies, vec![2]);
    assert_eq!(map.subconscious_response, 5);
    assert!(map.karmic_debts.is_empty());
    assert_eq!(
      [
        map.life_cycles[0].value,
        map.life_cycles[1].value,
        map.life_cycles[2].value
      ],
      [11, 7, 22]
    );
    assert_eq!(map.challenges, [5, 3, 2]);
    assert_eq!(map.decision_moments.map(|m| m.value), [9, 11, 11, 6]);
    assert_eq!(map.personal_year.value, 4);
    let arcana: Vec<u8> = map.life_triangle.arcana.iter().map(|a| a.code()).collect();
    assert_eq!(arcana, vec![21, 12, 22, 21, 12, 21, 13, 31, 13, 32, 27, 76]);
    assert_eq!(map.life_triangle.regent, 7);
  }

  #[test]
  fn empty_name_fails_both_charts() {
    let birth = BirthDate::new(2000, 1, 1).unwrap();
    assert!(PythagoreanChart::new("123", birth, birth, None).is_err());
    assert!(CabalisticMap::new("123", birth, birth).is_err());
  }
}
