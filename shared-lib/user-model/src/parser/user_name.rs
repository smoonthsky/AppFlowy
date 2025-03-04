use crate::errors::UserErrorCode;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct UserName(pub String);

impl UserName {
  pub fn parse(s: String) -> Result<UserName, UserErrorCode> {
    let is_empty_or_whitespace = s.trim().is_empty();
    if is_empty_or_whitespace {
      return Err(UserErrorCode::UserNameIsEmpty);
    }
    // A grapheme is defined by the Unicode standard as a "user-perceived"
    // character: `å` is a single grapheme, but it is composed of two characters
    // (`a` and `̊`).
    //
    // `graphemes` returns an iterator over the graphemes in the input `s`.
    // `true` specifies that we want to use the extended grapheme definition set,
    // the recommended one.
    let is_too_long = s.graphemes(true).count() > 256;
    if is_too_long {
      return Err(UserErrorCode::UserNameTooLong);
    }

    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

    if contains_forbidden_characters {
      return Err(UserErrorCode::UserNameContainForbiddenCharacters);
    }

    Ok(Self(s))
  }
}

impl AsRef<str> for UserName {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

#[cfg(test)]
mod tests {
  use super::UserName;
  use claim::{assert_err, assert_ok};

  #[test]
  fn a_256_grapheme_long_name_is_valid() {
    let name = "a̐".repeat(256);
    assert_ok!(UserName::parse(name));
  }

  #[test]
  fn a_name_longer_than_256_graphemes_is_rejected() {
    let name = "a".repeat(257);
    assert_err!(UserName::parse(name));
  }

  #[test]
  fn whitespace_only_names_are_rejected() {
    let name = " ".to_string();
    assert_err!(UserName::parse(name));
  }

  #[test]
  fn empty_string_is_rejected() {
    let name = "".to_string();
    assert_err!(UserName::parse(name));
  }

  #[test]
  fn names_containing_an_invalid_character_are_rejected() {
    for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
      let name = name.to_string();
      assert_err!(UserName::parse(name));
    }
  }

  #[test]
  fn a_valid_name_is_parsed_successfully() {
    let name = "nathan".to_string();
    assert_ok!(UserName::parse(name));
  }
}
