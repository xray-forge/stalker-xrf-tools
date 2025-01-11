use crate::project::gamedata_project_result::GamedataProjectVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use std::time::Instant;
use xray_ltx::{LtxProjectFormatResult, LtxProjectVerifyResult};

impl GamedataProject {
  pub fn verify(
    &mut self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataProjectVerificationResult> {
    log::info!(
      "Verifying gamedata project: {:?} | {:?}",
      self.roots,
      self.configs
    );

    if options.is_logging_enabled() {
      println!("Running gamedata verification");
    }

    let started_at: Instant = Instant::now();

    let format_result: GamedataResult<LtxProjectFormatResult> = self.verify_ltx_format(options);
    let schemes_result: GamedataResult<LtxProjectVerifyResult> = self.verify_ltx_schemes(options);
    let spawns_results: GamedataResult = self.verify_spawns(options);
    let weapons_result: GamedataResult = self.verify_ltx_weapons(options);
    let meshes_result: GamedataResult = self.verify_meshes(options);
    let textures_result: GamedataResult = self.verify_textures(options);
    let sounds_result: GamedataResult = self.verify_sounds(options);
    let resources_usage_result: GamedataResult = self.verify_resources_usage(options);

    let is_everything_valid: bool = format_result.is_ok()
      && schemes_result.is_ok()
      && spawns_results.is_ok()
      && weapons_result.is_ok()
      && meshes_result.is_ok()
      && textures_result.is_ok()
      && sounds_result.is_ok()
      && resources_usage_result.is_ok();

    Ok(GamedataProjectVerificationResult {
      is_valid: is_everything_valid,
      duration: started_at.elapsed().as_millis(),
    })
  }
}
