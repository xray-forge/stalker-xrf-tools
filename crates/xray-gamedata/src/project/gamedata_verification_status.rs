use derive_more::Display;

#[derive(Clone, Copy, Debug, Display, Eq, PartialEq)]
pub enum GamedataVerificationStatus {
  #[display("passed")]
  Passed,
  #[display("failed")]
  Failed,
  #[display("error")]
  Error,
  #[display("skipped")]
  Skipped,
  #[display("incomplete")]
  Incomplete,
}

impl GamedataVerificationStatus {
  pub const fn from_is_valid(is_valid: bool) -> Self {
    if is_valid { Self::Passed } else { Self::Failed }
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataVerificationStatus;

  #[test]
  fn maps_legacy_validity_and_displays_stable_names() {
    assert_eq!(
      GamedataVerificationStatus::from_is_valid(true),
      GamedataVerificationStatus::Passed
    );
    assert_eq!(
      GamedataVerificationStatus::from_is_valid(false),
      GamedataVerificationStatus::Failed
    );
    assert_eq!(
      GamedataVerificationStatus::Incomplete.to_string(),
      "incomplete"
    );
  }
}
