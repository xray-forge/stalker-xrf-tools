use std::time::Instant;

use xrf_error::{XrfError, XrfResult};

use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationReport, GamedataVerificationType};

impl GamedataProject {
  pub fn verify(&self, options: &GamedataProjectVerifyOptions) -> XrfResult<GamedataVerificationReport> {
    let checks: Vec<GamedataVerificationType> = options.selected_checks();

    if checks.is_empty() {
      return Err(XrfError::new_unexpected_error("No gamedata checks to perform provided"));
    }

    xrf_output::info!(options.output, "Verifying gamedata project: {}", self.root().display());

    xrf_output::info!(
      options.output,
      "Verifying modules: \n  -{}",
      checks.iter().map(ToString::to_string).collect::<Vec<_>>().join("\n  -")
    );

    xrf_output::info!(options.output, "");

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
  use std::path::PathBuf;

  use xrf_assets::{DirectoryAssetIndex, XrayAssetIndex};
  use xrf_ltx::LtxProject;

  use super::GamedataProject;
  use crate::{GamedataProjectVerifyOptions, GamedataVerificationStatus, GamedataVerificationType};

  fn empty_project() -> GamedataProject {
    GamedataProject {
      assets: XrayAssetIndex::new(
        DirectoryAssetIndex::read(env!("CARGO_MANIFEST_DIR")).expect("read test assets"),
        &[],
      )
      .expect("create test assets"),
      ltx_project: LtxProject {
        root: PathBuf::new(),
        ltx_file_entries: Vec::new(),
        ltx_files: Vec::new(),
        ltx_scheme_files: Vec::new(),
        ltx_scheme_file_entries: Vec::new(),
        ltx_scheme_declarations: Default::default(),
      },
    }
  }

  #[test]
  fn runs_each_selected_check_once_in_request_order() {
    let project: GamedataProject = empty_project();
    let options: GamedataProjectVerifyOptions = GamedataProjectVerifyOptions {
      checks: vec![GamedataVerificationType::Levels, GamedataVerificationType::Levels],
      output: xrf_output::OutputOptions::default(),
      ..Default::default()
    };

    let report = project
      .verify(&options)
      .expect("Expected level verification to complete");

    assert_eq!(report.checks().len(), 1);
    assert_eq!(report.checks()[0].verification_type(), GamedataVerificationType::Levels);
    // The test project ships no spawn file, so the level roster is unknown and nothing is checked.
    assert_eq!(report.status(), GamedataVerificationStatus::Skipped);
  }
}
