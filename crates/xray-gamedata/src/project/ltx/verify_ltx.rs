use crate::project::ltx::verify_ltx_result::GamedataLtxVerificationResult;
use crate::{
  GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationFinding,
  GamedataVerificationRule,
};
use colored::Colorize;
use std::path::Path;
use std::time::Instant;
use xray_error::{XRayError, XRayResult};
use xray_ltx::{
  LtxFormatOptions, LtxProjectFormatResult, LtxProjectVerifyResult, LtxVerifyOptions,
};

impl GamedataProject {
  pub fn verify_ltx(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataLtxVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify LTX files".green());
    }

    let started_at: Instant = Instant::now();

    let format_result: LtxProjectFormatResult = self.verify_ltx_format(options)?;
    let verification_result: LtxProjectVerifyResult = self.verify_ltx_schemes(options)?;
    let findings: Vec<GamedataVerificationFinding> =
      Self::collect_ltx_findings(&format_result, &verification_result);

    let duration: u128 = started_at.elapsed().as_millis();

    if options.is_logging_enabled() {
      println!(
        "Verified gamedata ltx files in {} sec",
        (duration as f64) / 1000.0,
      );
    }

    Ok(GamedataLtxVerificationResult {
      duration,
      findings,
      format_result,
      verification_result,
    })
  }

  fn collect_ltx_findings(
    format_result: &LtxProjectFormatResult,
    verification_result: &LtxProjectVerifyResult,
  ) -> Vec<GamedataVerificationFinding> {
    let mut findings: Vec<GamedataVerificationFinding> = format_result
      .to_format
      .iter()
      .map(|path| {
        GamedataVerificationFinding::for_asset(
          GamedataVerificationRule::LtxFormatting,
          path,
          "LTX file needs formatting",
        )
      })
      .collect();

    for error in &verification_result.errors {
      match error {
        XRayError::LtxScheme {
          at: Some(path),
          field,
          message,
          section,
        } => findings.push(GamedataVerificationFinding::for_asset(
          GamedataVerificationRule::LtxSchema,
          Path::new(path),
          format!("[{section}] {field}: {message}"),
        )),
        error => findings.push(GamedataVerificationFinding::without_asset(
          GamedataVerificationRule::LtxVerification,
          error.to_string(),
        )),
      }
    }

    findings.sort_by(|left, right| {
      left
        .asset_path
        .cmp(&right.asset_path)
        .then_with(|| left.message.cmp(&right.message))
    });

    findings
  }

  fn verify_ltx_format(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<LtxProjectFormatResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify LTX files formatting".green());
    }

    self
      .ltx_project
      .check_format_all_files_opt(LtxFormatOptions {
        is_silent: options.is_silent,
        is_verbose: options.is_verbose,
      })
  }

  fn verify_ltx_schemes(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<LtxProjectVerifyResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify LTX schemas".green());
    };

    self.ltx_project.verify_entries_opt(LtxVerifyOptions {
      is_silent: options.is_silent,
      is_verbose: options.is_verbose,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataProject;
  use crate::{GamedataVerificationFinding, GamedataVerificationRule};
  use std::path::PathBuf;
  use xray_error::XRayError;
  use xray_ltx::{LtxProjectFormatResult, LtxProjectVerifyResult};

  #[test]
  fn collects_format_and_scheme_findings_with_source_paths() {
    let format_result: LtxProjectFormatResult = LtxProjectFormatResult {
      to_format: vec![PathBuf::from("configs/system.ltx")],
      ..Default::default()
    };
    let verification_result: LtxProjectVerifyResult = LtxProjectVerifyResult {
      errors: vec![XRayError::new_scheme_error_at(
        "weather",
        "fog_density",
        "Expected a number",
        "configs/environment/weathers/test.ltx",
      )],
      ..Default::default()
    };

    let findings: Vec<GamedataVerificationFinding> =
      GamedataProject::collect_ltx_findings(&format_result, &verification_result);

    assert_eq!(
      findings,
      vec![
        GamedataVerificationFinding::for_asset(
          GamedataVerificationRule::LtxSchema,
          "configs/environment/weathers/test.ltx",
          "[weather] fog_density: Expected a number",
        ),
        GamedataVerificationFinding::for_asset(
          GamedataVerificationRule::LtxFormatting,
          "configs/system.ltx",
          "LTX file needs formatting",
        ),
      ]
    );
  }
}
