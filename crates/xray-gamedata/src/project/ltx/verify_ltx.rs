use crate::project::ltx::verify_ltx_result::GamedataLtxVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions};
use colored::Colorize;
use std::time::Instant;
use xray_error::XRayResult;
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

    let format_result: LtxProjectFormatResult =
      self
        .ltx_project
        .check_format_all_files_opt(LtxFormatOptions {
          is_silent: options.is_silent,
          is_verbose: options.is_verbose,
        })?;

    let verification_result: LtxProjectVerifyResult =
      self.ltx_project.verify_entries_opt(LtxVerifyOptions {
        is_silent: options.is_silent,
        is_verbose: options.is_verbose,
        is_strict: options.is_strict,
      })?;

    let duration: u128 = started_at.elapsed().as_millis();

    if options.is_logging_enabled() {
      println!(
        "Verified gamedata ltx files in {} sec",
        (duration as f64) / 1000.0,
      );
    }

    Ok(GamedataLtxVerificationResult {
      duration,
      format_result,
      verification_result,
    })
  }

  pub fn verify_ltx_format(
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

  pub fn verify_ltx_schemes(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<LtxProjectVerifyResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify LTX schemas".green());
    };

    self.ltx_project.verify_entries_opt(LtxVerifyOptions {
      is_silent: options.is_silent,
      is_verbose: options.is_verbose,
      is_strict: options.is_strict,
    })
  }
}
