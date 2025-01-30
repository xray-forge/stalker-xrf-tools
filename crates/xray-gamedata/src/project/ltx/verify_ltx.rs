use crate::project::ltx::verify_ltx_result::{
  GamedataLtxFormatVerificationResult, GamedataLtxVerificationResult,
};
use crate::{GamedataProject, GamedataProjectVerifyOptions};
use colored::Colorize;
use xray_error::XRayResult;
use xray_ltx::{
  LtxFormatOptions, LtxProjectFormatResult, LtxProjectVerifyResult, LtxVerifyOptions,
};

impl GamedataProject {
  // todo: Add used LTX files paths based on system ltx / spawn files.
  // todo: Add used LTX files paths based on system ltx / spawn files.
  // todo: Add used LTX files paths based on system ltx / spawn files.

  pub fn verify_ltx_format(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataLtxFormatVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata LTX files formatting".green());
    }

    let result: LtxProjectFormatResult =
      self
        .ltx_project
        .check_format_all_files_opt(LtxFormatOptions {
          is_silent: options.is_silent,
          is_verbose: options.is_verbose,
        })?;

    Ok(GamedataLtxFormatVerificationResult { inner: result })
  }

  pub fn verify_ltx_schemes(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataLtxVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata LTX schemas".green());
    };

    let result: LtxProjectVerifyResult = self.ltx_project.verify_entries_opt(LtxVerifyOptions {
      is_silent: options.is_silent,
      is_verbose: options.is_verbose,
      is_strict: options.is_strict,
    })?;

    Ok(GamedataLtxVerificationResult { inner: result })
  }
}
