//! Letter normalization and per-system letter values.
//!
//! Names are decomposed into a base ASCII letter plus an optional graphic
//! accent (and a cedilla flag). The Pythagorean system ignores accents; the
//! cabalistic system adds the accent's own value to the letter and reduces.

use crate::reduction::reduce_single;

/// A graphic accent that can sit on a letter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accent {
  /// Acute accent (´).
  Acute,
  /// Grave accent (`).
  Grave,
  /// Circumflex accent (^).
  Circumflex,
  /// Tilde (~).
  Tilde,
  /// Diaeresis / umlaut (¨).
  Diaeresis,
  /// Ring above (°).
  Ring,
}

/// A decoded character: base letter, optional accent, and cedilla flag.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecodedChar {
  /// Base letter in `A..=Z`.
  pub base: char,
  /// Accent sitting on the letter, if any.
  pub accent: Option<Accent>,
  /// Whether the original letter was a cedilla (ç).
  pub cedilla: bool,
}

/// Decode a character into a base letter plus accent, or `None` if it is not a
/// letter (digits, spaces and punctuation are not decodable).
#[must_use]
pub fn decode(c: char) -> Option<DecodedChar> {
  let upper = c.to_ascii_uppercase();
  if upper.is_ascii_uppercase() {
    return Some(DecodedChar {
      base: upper,
      accent: None,
      cedilla: false,
    });
  }
  let lower = c.to_lowercase().next().unwrap_or(c);
  let (base, accent, cedilla) = match lower {
    'á' => ('A', Some(Accent::Acute), false),
    'à' => ('A', Some(Accent::Grave), false),
    'â' => ('A', Some(Accent::Circumflex), false),
    'ã' => ('A', Some(Accent::Tilde), false),
    'ä' => ('A', Some(Accent::Diaeresis), false),
    'å' => ('A', Some(Accent::Ring), false),
    'é' => ('E', Some(Accent::Acute), false),
    'è' => ('E', Some(Accent::Grave), false),
    'ê' => ('E', Some(Accent::Circumflex), false),
    'ë' => ('E', Some(Accent::Diaeresis), false),
    'í' => ('I', Some(Accent::Acute), false),
    'ì' => ('I', Some(Accent::Grave), false),
    'î' => ('I', Some(Accent::Circumflex), false),
    'ï' => ('I', Some(Accent::Diaeresis), false),
    'ó' => ('O', Some(Accent::Acute), false),
    'ò' => ('O', Some(Accent::Grave), false),
    'ô' => ('O', Some(Accent::Circumflex), false),
    'õ' => ('O', Some(Accent::Tilde), false),
    'ö' => ('O', Some(Accent::Diaeresis), false),
    'ú' => ('U', Some(Accent::Acute), false),
    'ù' => ('U', Some(Accent::Grave), false),
    'û' => ('U', Some(Accent::Circumflex), false),
    'ü' => ('U', Some(Accent::Diaeresis), false),
    'ç' => ('C', None, true),
    'ñ' => ('N', Some(Accent::Tilde), false),
    _ => return None,
  };
  Some(DecodedChar { base, accent, cedilla })
}

/// Whether a base letter is a vowel (A, E, I, O, U). `Y` is treated as a
/// consonant, matching the default in both systems used here.
#[must_use]
pub fn is_vowel(base: char) -> bool {
  matches!(base, 'A' | 'E' | 'I' | 'O' | 'U')
}

/// Pythagorean value of a decoded letter (`A=1 .. I=9`, then repeating).
/// Accents are ignored and a cedilla is treated as a plain `C`.
#[must_use]
pub fn pythagorean_value(d: &DecodedChar) -> u8 {
  let idx = (d.base as u8) - b'A';
  (idx % 9) + 1
}

/// Cabalistic value of a decoded letter using the `1..8` table. A cedilla is
/// worth 8; an accented letter is the reduced sum of the letter and accent.
#[must_use]
pub fn cabalistic_value(d: &DecodedChar) -> u8 {
  if d.cedilla {
    return 8;
  }
  let base = cabalistic_base(d.base);
  match d.accent {
    None => base,
    Some(a) => reduce_single(u32::from(base) + u32::from(accent_value(a))).value as u8,
  }
}

fn cabalistic_base(base: char) -> u8 {
  match base {
    'A' | 'I' | 'Q' | 'J' | 'Y' => 1,
    'B' | 'K' | 'R' => 2,
    'C' | 'G' | 'L' | 'S' => 3,
    'D' | 'M' | 'T' | 'X' => 4,
    'E' | 'H' | 'N' => 5,
    'U' | 'V' | 'W' => 6,
    'O' | 'Z' => 7,
    'F' | 'P' => 8,
    _ => 0,
  }
}

fn accent_value(a: Accent) -> u8 {
  match a {
    Accent::Acute | Accent::Grave | Accent::Diaeresis => 2,
    Accent::Tilde => 3,
    Accent::Circumflex | Accent::Ring => 7,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn decodes_plain_and_accented_letters() {
    assert_eq!(
      decode('C'),
      Some(DecodedChar {
        base: 'C',
        accent: None,
        cedilla: false
      })
    );
    assert_eq!(
      decode('ê'),
      Some(DecodedChar {
        base: 'E',
        accent: Some(Accent::Circumflex),
        cedilla: false
      })
    );
    assert_eq!(
      decode('Ç'),
      Some(DecodedChar {
        base: 'C',
        accent: None,
        cedilla: true
      })
    );
    assert_eq!(decode('7'), None);
    assert_eq!(decode(' '), None);
  }

  #[test]
  fn pythagorean_table_wraps_every_nine() {
    assert_eq!(pythagorean_value(&decode('A').unwrap()), 1);
    assert_eq!(pythagorean_value(&decode('I').unwrap()), 9);
    assert_eq!(pythagorean_value(&decode('J').unwrap()), 1);
    assert_eq!(pythagorean_value(&decode('R').unwrap()), 9);
    assert_eq!(pythagorean_value(&decode('Z').unwrap()), 8);
  }

  #[test]
  fn cabalistic_table_matches_reference_map() {
    for (c, v) in [
      ('A', 1),
      ('B', 2),
      ('C', 3),
      ('E', 5),
      ('L', 3),
      ('S', 3),
      ('U', 6),
      ('V', 6),
      ('R', 2),
      ('Z', 7),
    ] {
      assert_eq!(cabalistic_value(&decode(c).unwrap()), v, "letter {c}");
    }
  }

  #[test]
  fn cabalistic_accent_adds_and_reduces() {
    // ê = E(5) + circumflex(7) = 12 -> 3.
    assert_eq!(cabalistic_value(&decode('ê').unwrap()), 3);
    // á = A(1) + acute(2) = 3.
    assert_eq!(cabalistic_value(&decode('á').unwrap()), 3);
    // ã = A(1) + tilde(3) = 4.
    assert_eq!(cabalistic_value(&decode('ã').unwrap()), 4);
    // õ = O(7) + tilde(3) = 10 -> 1.
    assert_eq!(cabalistic_value(&decode('õ').unwrap()), 1);
    // ü = U(6) + diaeresis(2) = 8.
    assert_eq!(cabalistic_value(&decode('ü').unwrap()), 8);
    // í = I(1) + acute(2) = 3.
    assert_eq!(cabalistic_value(&decode('í').unwrap()), 3);
  }

  #[test]
  fn cedilla_is_worth_eight_in_cabalistic() {
    let c = decode('ç').unwrap();
    assert!(c.cedilla);
    assert_eq!(cabalistic_value(&c), 8);
    // The Pythagorean system treats a cedilla as a plain C.
    assert_eq!(pythagorean_value(&c), pythagorean_value(&decode('C').unwrap()));
  }

  #[test]
  fn decode_is_case_insensitive() {
    assert_eq!(decode('a'), decode('A'));
    assert_eq!(decode('z'), decode('Z'));
    assert_eq!(decode('É'), decode('é'));
  }

  #[test]
  fn tilde_n_decodes_to_n_with_tilde() {
    assert_eq!(
      decode('ñ'),
      Some(DecodedChar {
        base: 'N',
        accent: Some(Accent::Tilde),
        cedilla: false
      })
    );
  }

  #[test]
  fn vowels_exclude_y() {
    for v in ['A', 'E', 'I', 'O', 'U'] {
      assert!(is_vowel(v), "{v} should be a vowel");
    }
    for c in ['Y', 'B', 'Z', 'M'] {
      assert!(!is_vowel(c), "{c} should be a consonant");
    }
  }

  #[test]
  fn cabalistic_full_alphabet_stays_in_range() {
    for c in 'A'..='Z' {
      let v = cabalistic_value(&decode(c).unwrap());
      assert!((1..=8).contains(&v), "letter {c} -> {v} out of 1..=8");
    }
  }

  #[test]
  fn pythagorean_full_alphabet_stays_in_range() {
    for c in 'A'..='Z' {
      let v = pythagorean_value(&decode(c).unwrap());
      assert!((1..=9).contains(&v), "letter {c} -> {v} out of 1..=9");
    }
  }
}
