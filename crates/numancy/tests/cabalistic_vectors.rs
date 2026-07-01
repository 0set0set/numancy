//! Black-box vectors for the cabalistic system.
//!
//! Fixtures use pioneering women in computing and their real birth dates.

use numancy::chart::CabalisticMap;
use numancy::pyramid::life_triangle;
use numancy::{cabalistic, compatibility, BirthDate};

#[test]
fn full_map_for_barbara_liskov() {
  // Barbara Liskov (1939-11-07), reference 2020-03-15.
  let birth = BirthDate::new(1939, 11, 7).unwrap();
  let reference = BirthDate::new(2020, 3, 15).unwrap();
  let map = CabalisticMap::new("Barbara Liskov", birth, reference).unwrap();

  assert_eq!(map.motivation.value, 11);
  assert!(map.motivation.is_master);
  assert_eq!(map.impression.value, 4);
  assert_eq!(map.expression.value, 6);
  assert_eq!(map.hidden_talent.value, 5);
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

  let arcana: Vec<u8> = map.life_triangle.arcana.iter().map(|a| a.code()).collect();
  assert_eq!(arcana, vec![21, 12, 22, 21, 12, 21, 13, 31, 13, 32, 27, 76]);
  assert_eq!(map.life_triangle.regent, 7);
}

#[test]
fn accented_letters_carry_value() {
  // Karen Spärck Jones: the "ä" adds its accent value (total 57 -> 12 -> 3).
  assert_eq!(cabalistic::expression_number("Karen Spärck Jones").unwrap().value, 3);
  assert_eq!(cabalistic::motivation_number("Karen Spärck Jones").unwrap().value, 3);
  assert_eq!(cabalistic::impression_number("Karen Spärck Jones").unwrap().value, 9);
  // Dropping the accent changes the expression.
  assert_ne!(
    cabalistic::expression_number("Karen Sparck Jones").unwrap().value,
    cabalistic::expression_number("Karen Spärck Jones").unwrap().value
  );
}

#[test]
fn expression_karmic_debt_thirteen() {
  // Grace Hopper: her name totals 49 -> 13 (a karmic debt) -> 4.
  let debts = cabalistic::karmic_debts("Grace Hopper", BirthDate::new(1906, 12, 9).unwrap()).unwrap();
  assert_eq!(debts.len(), 1);
  assert_eq!(debts[0].position, "expression");
  assert_eq!(debts[0].debt, 13);
}

#[test]
fn personal_day_can_be_a_master() {
  // Katherine Johnson (1918-08-26): Personal Day on 2020-03-15 is a master 11.
  let birth = BirthDate::new(1918, 8, 26).unwrap();
  let day = cabalistic::personal_day(birth, 2020, 3, 15);
  assert_eq!(day.value, 11);
  assert!(day.is_master);
}

#[test]
fn triangle_regent_and_arcana() {
  // Grace Hopper: 11 letters give 10 arcana and a regent arcane of 9.
  let triangle = life_triangle("Grace Hopper").unwrap();
  assert_eq!(triangle.base, vec![3, 2, 1, 3, 5, 5, 7, 8, 8, 5, 2]);
  assert_eq!(triangle.arcana.len(), 10);
  assert_eq!(triangle.regent, 9);
}

#[test]
fn compatibility_lookup_only_known_for_seven() {
  assert!(compatibility::relationship_harmony(7).is_some());
  assert!(compatibility::harmonic_numbers(7).is_some());
  assert!(compatibility::favorable_days(7).is_some());
  assert!(compatibility::relationship_harmony(3).is_none());
}
