//! Black-box vectors for the Pythagorean system.
//!
//! Fixtures use pioneering women in computing and their real birth dates.

use numancy::chart::PythagoreanChart;
use numancy::{address, date, pythagorean, BirthDate};

#[test]
fn expression_motivation_impression_for_grace_hopper() {
  // Grace Hopper: Expression 4, Soul Urge 8, Personality 5.
  assert_eq!(pythagorean::expression_number("Grace Hopper").unwrap().value, 4);
  assert_eq!(pythagorean::motivation_number("Grace Hopper").unwrap().value, 8);
  assert_eq!(pythagorean::impression_number("Grace Hopper").unwrap().value, 5);
}

#[test]
fn expression_master_number_for_annie_easley() {
  // Annie Easley's letters reduce to the master number 11.
  let cn = pythagorean::expression_number("Annie Easley").unwrap();
  assert_eq!(cn.value, 11);
  assert!(cn.is_master);
}

#[test]
fn life_path_master_and_debt() {
  // Dorothy Vaughan (1910-09-20) -> master number 22.
  let master = date::life_path_number(BirthDate::new(1910, 9, 20).unwrap());
  assert_eq!(master.value, 22);
  assert!(master.is_master);

  // Ada Lovelace (1815-12-10) -> 19 -> 1, passing through the karmic debt 19.
  let debt = date::life_path_number(BirthDate::new(1815, 12, 10).unwrap());
  assert_eq!(debt.value, 1);
  assert!(debt.passes_through(19));
}

#[test]
fn house_numbers() {
  assert_eq!(address::house_number("42B").unwrap().value, 8);
  assert_eq!(address::house_number("77").unwrap().value, 5);
  assert_eq!(address::house_number("100").unwrap().value, 1);
}

#[test]
fn full_chart_for_dorothy_vaughan() {
  let birth = BirthDate::new(1910, 9, 20).unwrap();
  let reference = BirthDate::new(2020, 3, 15).unwrap();
  let chart = PythagoreanChart::new("Dorothy Vaughan", birth, reference, Some("77")).unwrap();

  assert_eq!(chart.expression.value, 8);
  assert_eq!(chart.life_path.value, 22);
  assert!(chart.life_path.is_master);
  assert_eq!(chart.birthday.value, 2);
  assert_eq!(chart.personal_year.value, 6);
  assert_eq!(chart.universal_day.value, 4);
  assert_eq!(chart.house.unwrap().value, 5);
  assert!(chart.karmic_debts.is_empty());
}

#[test]
fn accents_do_not_change_pythagorean_results() {
  // Karen Spärck Jones: the "ä" is stripped to a plain "a".
  assert_eq!(
    pythagorean::expression_number("Karen Spärck Jones").unwrap(),
    pythagorean::expression_number("Karen Sparck Jones").unwrap()
  );
}
