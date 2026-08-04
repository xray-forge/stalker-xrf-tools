use crate::{
  GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationReport,
  GamedataVerificationType,
};
use std::time::Instant;
use xray_error::{XRayError, XRayResult};

impl GamedataProject {
  pub fn verify(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataVerificationReport> {
    let checks: Vec<GamedataVerificationType> = options.selected_checks();

    if checks.is_empty() {
      return Err(XRayError::new_unexpected_error(
        "No gamedata checks to perform provided",
      ));
    }

    if options.is_logging_enabled() {
      println!("Verifying gamedata project: {}", self.root.display());

      println!(
        "Verifying modules: \n  -{}",
        checks
          .iter()
          .map(ToString::to_string)
          .collect::<Vec<_>>()
          .join("\n  -")
      );

      println!();
    }

    let started_at: Instant = Instant::now();
    let mut result: GamedataVerificationReport = GamedataVerificationReport::default();

    for check in checks {
      result.add_report(check.run(self, options));
    }

    result.set_duration(started_at.elapsed());

    Ok(result)
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataProject;
  use crate::{GamedataProjectVerifyOptions, GamedataVerificationStatus, GamedataVerificationType};
  use std::collections::HashMap;
  use std::path::PathBuf;
  use xray_ltx::LtxProject;

  fn empty_project() -> GamedataProject {
    GamedataProject {
      assets: HashMap::new(),
      ltx_project: LtxProject {
        root: PathBuf::new(),
        ltx_file_entries: Vec::new(),
        ltx_files: Vec::new(),
        ltx_scheme_files: Vec::new(),
        ltx_scheme_file_entries: Vec::new(),
        ltx_scheme_declarations: Default::default(),
      },
      root: PathBuf::new(),
    }
  }

  #[test]
  fn runs_each_selected_check_once_in_request_order() {
    let project: GamedataProject = empty_project();
    let options: GamedataProjectVerifyOptions = GamedataProjectVerifyOptions {
      checks: vec![
        GamedataVerificationType::Levels,
        GamedataVerificationType::Levels,
      ],
      is_silent: true,
      ..Default::default()
    };

    let report = project
      .verify(&options)
      .expect("Expected level verification to complete");

    assert_eq!(report.checks().len(), 1);
    assert_eq!(
      report.checks()[0].verification_type(),
      GamedataVerificationType::Levels
    );
    assert_eq!(report.status(), GamedataVerificationStatus::Incomplete);
  }
}
