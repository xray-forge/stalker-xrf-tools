use crate::project::animations::verify_animations_result::GamedataAnimationsVerificationResult;
use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;

use crate::project::levels::verify_levels_result::GamedataLevelVerificationResult;
use crate::project::ltx::verify_ltx_result::{
  GamedataLtxFormatVerificationResult, GamedataLtxVerificationResult,
};
use crate::project::meshes::verify_meshes_result::GamedataMeshesVerificationResult;
use crate::project::particles::verify_particles_result::GamedataParticlesVerificationResult;
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
  pub particles_result: GamedataResult<GamedataParticlesVerificationResult>,
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
      .animations_result
      .as_ref()
      .is_ok_and(GamedataAnimationsVerificationResult::is_valid)
      && self
        .format_result
        .as_ref()
        .is_ok_and(GamedataLtxFormatVerificationResult::is_valid)
      && self
        .spawns_result
        .as_ref()
        .is_ok_and(GamedataSpawnsVerificationResult::is_valid)
      && self
        .meshes_result
        .as_ref()
        .is_ok_and(GamedataMeshesVerificationResult::is_valid)
      && self
        .levels_result
        .as_ref()
        .is_ok_and(GamedataLevelVerificationResult::is_valid)
      && self
        .particles_result
        .as_ref()
        .is_ok_and(GamedataParticlesVerificationResult::is_valid)
      && self
        .schemes_result
        .as_ref()
        .is_ok_and(GamedataLtxVerificationResult::is_valid)
      && self
        .scripts_result
        .as_ref()
        .is_ok_and(GamedataScriptsVerificationResult::is_valid)
      && self
        .shaders_result
        .as_ref()
        .is_ok_and(GamedataShadersVerificationResult::is_valid)
      && self
        .sounds_result
        .as_ref()
        .is_ok_and(GamedataSoundsVerificationResult::is_valid)
      && self
        .textures_result
        .as_ref()
        .is_ok_and(GamedataTexturesVerificationResult::is_valid)
      && self
        .weapons_result
        .as_ref()
        .is_ok_and(GamedataWeaponVerificationResult::is_valid)
      && self
        .weathers_result
        .as_ref()
        .is_ok_and(GamedataWeathersVerificationResult::is_valid)
  }

  pub fn get_failure_messages(&self) -> Vec<String> {
    vec![
      Self::get_result_failure_message(&self.animations_result, "animations"),
      Self::get_result_failure_message(&self.format_result, "format"),
      Self::get_result_failure_message(&self.levels_result, "levels"),
      Self::get_result_failure_message(&self.meshes_result, "meshes"),
      Self::get_result_failure_message(&self.particles_result, "particles"),
      Self::get_result_failure_message(&self.schemes_result, "schemes"),
      Self::get_result_failure_message(&self.scripts_result, "scripts"),
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
