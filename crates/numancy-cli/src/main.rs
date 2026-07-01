//! Command-line interface for the `numancy` numerology library.
//!
//! It computes a single numerology number, chosen by `--system` and `--number`,
//! and prints the result as JSON.

use std::error::Error;

use clap::{Parser, ValueEnum};
use numancy::{cabalistic, date, pythagorean, BirthDate, CalculatedNumber, NumerologyError, ReferenceDate};

/// Compute a single numerology number and print it as JSON.
#[derive(Parser)]
#[command(name = "numancy", version, about)]
struct Cli {
  /// Full name to analyze.
  name: String,
  /// Birth date, formatted as YYYY-MM-DD.
  birth: String,
  /// Numerology system.
  #[arg(short, long, value_enum)]
  system: System,
  /// Which number to compute. If omitted, every number of the system is returned.
  #[arg(short, long, value_enum)]
  number: Option<Number>,
  /// Reference date (YYYY-MM-DD), used by personal-year/month/day and
  /// universal-day. Defaults to today (UTC).
  #[arg(short, long)]
  reference: Option<String>,
}

/// The numerology system to use.
#[derive(Clone, Copy, ValueEnum)]
enum System {
  /// Western Pythagorean system.
  Pythagorean,
  /// Brazilian cabalistic system.
  Cabalistic,
}

impl System {
  fn as_str(self) -> &'static str {
    match self {
      Self::Pythagorean => "pythagorean",
      Self::Cabalistic => "cabalistic",
    }
  }
}

/// The number to compute. Availability depends on the chosen system.
#[derive(Clone, Copy, ValueEnum)]
enum Number {
  Expression,
  Motivation,
  Impression,
  LifePath,
  Birthday,
  Maturity,
  PersonalYear,
  PersonalMonth,
  PersonalDay,
  UniversalDay,
  HiddenTalent,
  Psychic,
  Destiny,
  Mission,
}

impl Number {
  fn as_str(self) -> &'static str {
    match self {
      Self::Expression => "expression",
      Self::Motivation => "motivation",
      Self::Impression => "impression",
      Self::LifePath => "life-path",
      Self::Birthday => "birthday",
      Self::Maturity => "maturity",
      Self::PersonalYear => "personal-year",
      Self::PersonalMonth => "personal-month",
      Self::PersonalDay => "personal-day",
      Self::UniversalDay => "universal-day",
      Self::HiddenTalent => "hidden-talent",
      Self::Psychic => "psychic",
      Self::Destiny => "destiny",
      Self::Mission => "mission",
    }
  }
}

/// Compute the requested number, or `None` if it is not defined for the system.
fn compute(
  system: System,
  number: Number,
  name: &str,
  birth: BirthDate,
  reference: ReferenceDate,
) -> Result<Option<CalculatedNumber>, NumerologyError> {
  let value = match (system, number) {
    (System::Pythagorean, Number::Expression) => Some(pythagorean::expression_number(name)?),
    (System::Pythagorean, Number::Motivation) => Some(pythagorean::motivation_number(name)?),
    (System::Pythagorean, Number::Impression) => Some(pythagorean::impression_number(name)?),
    (System::Pythagorean, Number::LifePath) => Some(date::life_path_number(birth)),
    (System::Pythagorean, Number::Birthday) => Some(date::birthday_number(birth)),
    (System::Pythagorean, Number::Maturity) => {
      let life_path = date::life_path_number(birth);
      let expression = pythagorean::expression_number(name)?;
      Some(date::maturity_number(&life_path, &expression))
    }
    (System::Pythagorean, Number::PersonalYear) => Some(date::personal_year(birth, reference.year)),
    (System::Pythagorean, Number::PersonalMonth) => Some(date::personal_month(birth, reference.year, reference.month)),
    (System::Pythagorean, Number::PersonalDay) => Some(date::personal_day(
      birth,
      reference.year,
      reference.month,
      reference.day,
    )),
    (System::Pythagorean, Number::UniversalDay) => Some(date::universal_day(reference)),

    (System::Cabalistic, Number::Motivation) => Some(cabalistic::motivation_number(name)?),
    (System::Cabalistic, Number::Impression) => Some(cabalistic::impression_number(name)?),
    (System::Cabalistic, Number::Expression) => Some(cabalistic::expression_number(name)?),
    (System::Cabalistic, Number::HiddenTalent) => Some(cabalistic::hidden_talent_number(name)?),
    (System::Cabalistic, Number::Birthday) => Some(cabalistic::birth_day_number(birth)),
    (System::Cabalistic, Number::Psychic) => Some(cabalistic::psychic_number(birth)),
    (System::Cabalistic, Number::Destiny) => Some(cabalistic::destiny_number(birth)),
    (System::Cabalistic, Number::Mission) => Some(cabalistic::mission_number(name, birth)?),
    (System::Cabalistic, Number::PersonalYear) => Some(cabalistic::personal_year(birth, reference.year)),
    (System::Cabalistic, Number::PersonalMonth) => {
      Some(cabalistic::personal_month(birth, reference.year, reference.month))
    }
    (System::Cabalistic, Number::PersonalDay) => Some(cabalistic::personal_day(
      birth,
      reference.year,
      reference.month,
      reference.day,
    )),

    _ => None,
  };

  Ok(value)
}

fn parse_date(input: &str) -> Result<BirthDate, Box<dyn Error>> {
  let mut parts = input.split('-');
  let (Some(year), Some(month), Some(day), None) = (parts.next(), parts.next(), parts.next(), parts.next()) else {
    return Err(format!("expected date as YYYY-MM-DD, got '{input}'").into());
  };
  Ok(BirthDate::new(year.parse()?, month.parse()?, day.parse()?)?)
}

fn today() -> Result<BirthDate, Box<dyn Error>> {
  let secs = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)?
    .as_secs();
  let (year, month, day) = civil_from_days((secs / 86_400) as i64);
  Ok(BirthDate::new(u32::try_from(year)?, month, day)?)
}

/// Convert a count of days since 1970-01-01 (UTC) into a `(year, month, day)`
/// Gregorian date. Adapted from Howard Hinnant's public-domain `civil_from_days`.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
  let z = days + 719_468;
  let era = (if z >= 0 { z } else { z - 146_096 }) / 146_097;
  let doe = z - era * 146_097;
  let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
  let year = yoe + era * 400;
  let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
  let mp = (5 * doy + 2) / 153;
  let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
  let month = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
  (year + i64::from(month <= 2), month, day)
}

fn main() -> Result<(), Box<dyn Error>> {
  let cli = Cli::parse();
  let birth = parse_date(&cli.birth)?;
  let reference: ReferenceDate = match &cli.reference {
    Some(value) => parse_date(value)?,
    None => today()?,
  };

  let output = match cli.number {
    Some(number) => {
      let Some(result) = compute(cli.system, number, &cli.name, birth, reference)? else {
        return Err(
          format!(
            "the '{}' number is not available for the {} system",
            number.as_str(),
            cli.system.as_str()
          )
          .into(),
        );
      };
      serde_json::json!({
        "name": cli.name,
        "system": cli.system.as_str(),
        "number": number.as_str(),
        "value": result.value,
        "master": result.is_master,
      })
    }
    None => {
      let mut numbers = serde_json::Map::new();
      for &number in Number::value_variants() {
        if let Some(result) = compute(cli.system, number, &cli.name, birth, reference)? {
          numbers.insert(
            number.as_str().to_owned(),
            serde_json::json!({ "value": result.value, "master": result.is_master }),
          );
        }
      }
      serde_json::json!({
        "name": cli.name,
        "system": cli.system.as_str(),
        "numbers": serde_json::Value::Object(numbers),
      })
    }
  };

  println!("{}", serde_json::to_string_pretty(&output)?);
  Ok(())
}
