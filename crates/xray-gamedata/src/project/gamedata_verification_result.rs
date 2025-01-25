use crate::project::animations::verify_animations_result::GamedataAnimationsVerificationResult;
use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;

use crate::project::levels::verify_levels_result::GamedataLevelVerificationResult;
use crate::project::ltx::verify_ltx_result::{
  GamedataLtxFormatVerificationResult, GamedataLtxVerificationResult,
};
use crate::project::meshes::verify_meshes_result::GamedataMeshesVerificationResult;
use crate::project::scripts::verify_scripts_result::GamedataScriptsVerificationResult;
use crate::project::shaders::verify_shaders_result::GamedataShadersVerificationResult;
use crate::project::sounds::verify_sounds_result::GamedataSoundsVerificationResult;
use crate::project::spawns::verify_spawns_result::GamedataSpawnsVerificationResult;
use crate::project::textures::verify_textures_result::GamedataTexturesVerificationResult;
use crate::project::weapons::verify_weapons_result::GamedataWeaponVerificationResult;
use crate::project::weathers::verify_weathers_result::GamedataWeathersVerificationResult;
use crate::GamedataResult;

pub struct GamedataVerificationResult {
  pub duration: u128,
  pub animations_result: GamedataResult<GamedataAnimationsVerificationResult>,
  pub format_result: GamedataResult<GamedataLtxFormatVerificationResult>,
  pub levels_result: GamedataResult<GamedataLevelVerificationResult>,
  pub meshes_result: GamedataResult<GamedataMeshesVerificationResult>,
  pub schemes_result: GamedataResult<GamedataLtxVerificationResult>,
  pub scripts_result: GamedataResult<GamedataScriptsVerificationResult>,
  pub shaders_result: GamedataResult<GamedataShadersVerificationResult>,
  pub sounds_result: GamedataResult<GamedataSoundsVerificationResult>,
  pub spawns_result: GamedataResult<GamedataSpawnsVerificationResult>,
  pub textures_result: GamedataResult<GamedataTexturesVerificationResult>,
  pub weapons_result: GamedataResult<GamedataWeaponVerificationResult>,
  pub weathers_result: GamedataResult<GamedataWeathersVerificationResult>,
}

impl GamedataVerificationResult {
  pub fn is_valid(&self) -> bool {
    self
      .format_result
      .as_ref()
      .is_ok_and(GamedataLtxFormatVerificationResult::is_valid)
      && self
        .schemes_result
        .as_ref()
        .is_ok_and(GamedataLtxVerificationResult::is_valid)
      && self.spawns_result.is_ok()
      && self
        .weapons_result
        .as_ref()
        .is_ok_and(GamedataWeaponVerificationResult::is_valid)
      && self.meshes_result.is_ok()
      && self
        .animations_result
        .as_ref()
        .is_ok_and(GamedataAnimationsVerificationResult::is_valid)
      && self.textures_result.is_ok()
      && self.scripts_result.is_ok()
      && self.sounds_result.is_ok()
      && self.levels_result.is_ok()
      && self.weathers_result.is_ok()
      && self.shaders_result.is_ok()
  }

  pub fn get_failure_messages(&self) -> Vec<String> {
    vec![
      Self::get_result_failure_message(&self.animations_result, "animations"),
      Self::get_result_failure_message(&self.format_result, "format"),
      Self::get_result_failure_message(&self.levels_result, "levels"),
      Self::get_result_failure_message(&self.meshes_result, "meshes"),
      Self::get_result_failure_message(&self.scripts_result, "scripts"),
      Self::get_result_failure_message(&self.schemes_result, "schemes"),
      Self::get_result_failure_message(&self.shaders_result, "shaders"),
      Self::get_result_failure_message(&self.sounds_result, "sounds"),
      Self::get_result_failure_message(&self.spawns_result, "spawns"),
      Self::get_result_failure_message(&self.textures_result, "textures"),
      Self::get_result_failure_message(&self.weapons_result, "weapons"),
      Self::get_result_failure_message(&self.weathers_result, "weathers"),
    ]
    .into_iter()
    .flatten()
    .collect()
  }

  fn get_result_failure_message<T>(result: &GamedataResult<T>, comment: &str) -> Option<String>
  where
    T: GamedataGenericVerificationResult,
  {
    match result {
      Ok(result) => {
        if result.is_valid() {
          None
        } else {
          Some(result.get_failure_message())
        }
      }
      Err(error) => Some(format!("Check failed ({comment}): {error}")),
    }
  }
}
