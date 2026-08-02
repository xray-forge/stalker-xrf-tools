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
  pub fn aggregate(statuses: impl IntoIterator<Item = Self>) -> Self {
    let mut aggregate: Self = Self::Skipped;

    for status in statuses {
      aggregate = match (aggregate, status) {
        (Self::Error, _) | (_, Self::Error) => Self::Error,
        (Self::Incomplete, _) | (_, Self::Incomplete) => Self::Incomplete,
        (Self::Failed, _) | (_, Self::Failed) => Self::Failed,
        (Self::Passed, _) | (_, Self::Passed) => Self::Passed,
        _ => Self::Skipped,
      };
    }

    aggregate
  }

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

  #[test]
  fn aggregates_statuses_by_severity() {
    use GamedataVerificationStatus::{Error, Failed, Incomplete, Passed, Skipped};

    assert_eq!(GamedataVerificationStatus::aggregate([]), Skipped);
    assert_eq!(
      GamedataVerificationStatus::aggregate([Skipped, Passed]),
      Passed
    );
    assert_eq!(
      GamedataVerificationStatus::aggregate([Passed, Incomplete]),
      Incomplete
    );
    assert_eq!(
      GamedataVerificationStatus::aggregate([Failed, Incomplete]),
      Incomplete
    );
    assert_eq!(
      GamedataVerificationStatus::aggregate([Incomplete, Error]),
      Error
    );
  }
}
