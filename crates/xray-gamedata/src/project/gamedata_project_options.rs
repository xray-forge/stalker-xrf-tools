use crate::project::gamedata_verification_type::GamedataVerificationType;
use std::path::PathBuf;

#[derive(Default)]
pub struct GamedataProjectReadOptions {
  pub root: PathBuf,
  pub ignored: Vec<String>,
  pub is_verbose: bool,
  pub is_silent: bool,
  pub is_strict: bool,
}

impl GamedataProjectReadOptions {
  pub fn is_logging_enabled(&self) -> bool {
    !self.is_silent
  }

  pub fn is_verbose_logging_enabled(&self) -> bool {
    !self.is_silent && self.is_verbose
  }
}

#[derive(Default)]
pub struct GamedataProjectVerifyOptions {
  pub is_verbose: bool,
  pub is_silent: bool,
  pub is_strict: bool,
  pub checks: Vec<GamedataVerificationType>,
}

impl GamedataProjectVerifyOptions {
  pub fn selected_checks(&self) -> Vec<GamedataVerificationType> {
    let mut checks: Vec<GamedataVerificationType> = Vec::with_capacity(self.checks.len());

    for check in &self.checks {
      if !checks.contains(check) {
        checks.push(*check);
      }
    }

    checks
  }

  pub fn is_logging_enabled(&self) -> bool {
    !self.is_silent
  }

  pub fn is_verbose_logging_enabled(&self) -> bool {
    !self.is_silent && self.is_verbose
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataProjectVerifyOptions;
  use crate::GamedataVerificationType;

  #[test]
  fn selected_checks_preserves_first_requested_order_and_removes_duplicates() {
    let options: GamedataProjectVerifyOptions = GamedataProjectVerifyOptions {
      checks: vec![
        GamedataVerificationType::Textures,
        GamedataVerificationType::Scripts,
        GamedataVerificationType::Textures,
      ],
      ..Default::default()
    };

    assert_eq!(
      options.selected_checks(),
      vec![
        GamedataVerificationType::Textures,
        GamedataVerificationType::Scripts,
      ]
    );
  }
}
