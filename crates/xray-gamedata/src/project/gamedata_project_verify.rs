use crate::project::gamedata_project_result::GamedataProjectVerificationResult;
use crate::{
  GamedataProject, GamedataProjectVerifyOptions, GamedataProjectWeaponVerificationResult,
  GamedataResult,
};
use std::time::Instant;
use xray_ltx::{LtxProjectFormatResult, LtxProjectVerifyResult};

impl GamedataProject {
  pub fn verify(
    &mut self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataProjectVerificationResult> {
    if options.is_logging_enabled() {
      println!(
        "Verifying gamedata project: {:?} | {:?}",
        self.roots, self.configs
      );
    }

    let started_at: Instant = Instant::now();

    let format_result: GamedataResult<LtxProjectFormatResult> = self.verify_ltx_format(options);
    let schemes_result: GamedataResult<LtxProjectVerifyResult> = self.verify_ltx_schemes(options);
    let spawns_results: GamedataResult = self.verify_spawns(options);
    let weapons_result: GamedataResult<GamedataProjectWeaponVerificationResult> =
      self.verify_ltx_weapons(options);
    let meshes_result: GamedataResult = self.verify_meshes(options);
    let animations_result: GamedataResult = self.verify_animations(options);
    let textures_result: GamedataResult = self.verify_textures(options);
    let sounds_result: GamedataResult = self.verify_sounds(options);
    let levels_result: GamedataResult = self.verify_levels(options);
    let weathers_result: GamedataResult = self.verify_weathers(options);
    let shaders_result: GamedataResult = self.verify_shaders(options);
    let resources_usage_result: GamedataResult = self.verify_resources_usage(options);

    let is_everything_valid: bool = format_result.is_ok_and(|it| it.invalid_files == 0)
      && schemes_result.is_ok_and(|it| it.errors.is_empty())
      && spawns_results.is_ok()
      && weapons_result.is_ok_and(|it| it.is_valid)
      && meshes_result.is_ok()
      && animations_result.is_ok()
      && textures_result.is_ok()
      && sounds_result.is_ok()
      && levels_result.is_ok()
      && weathers_result.is_ok()
      && shaders_result.is_ok()
      && resources_usage_result.is_ok();

    Ok(GamedataProjectVerificationResult {
      is_valid: is_everything_valid,
      duration: started_at.elapsed().as_millis(),
    })
  }
}
