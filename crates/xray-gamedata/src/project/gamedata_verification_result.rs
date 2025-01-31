use crate::project::animations::verify_animations_result::GamedataAnimationsVerificationResult;
use crate::project::gamedata_generic_result::GamedataGenericVerificationResult;
use xray_error::XRayResult;

use crate::project::levels::verify_levels_result::GamedataLevelVerificationResult;
use crate::project::ltx::verify_ltx_result::GamedataLtxVerificationResult;
use crate::project::meshes::verify_meshes_result::GamedataMeshesVerificationResult;
use crate::project::particles::verify_particles_result::GamedataParticlesVerificationResult;
use crate::project::scripts::verify_scripts_result::GamedataScriptsVerificationResult;
use crate::project::shaders::verify_shaders_result::GamedataShadersVerificationResult;
use crate::project::sounds::verify_sounds_result::GamedataSoundsVerificationResult;
use crate::project::spawns::verify_spawns_result::GamedataSpawnsVerificationResult;
use crate::project::textures::verify_textures_result::GamedataTexturesVerificationResult;
use crate::project::weapons::verify_weapons_result::GamedataWeaponVerificationResult;
use crate::project::weathers::verify_weathers_result::GamedataWeathersVerificationResult;

pub struct GamedataVerificationResult {
  pub duration: u128,
  pub animations_result: Option<XRayResult<GamedataAnimationsVerificationResult>>,
  pub ltx_result: Option<XRayResult<GamedataLtxVerificationResult>>,
  pub levels_result: Option<XRayResult<GamedataLevelVerificationResult>>,
  pub meshes_result: Option<XRayResult<GamedataMeshesVerificationResult>>,
  pub particles_result: Option<XRayResult<GamedataParticlesVerificationResult>>,
  pub scripts_result: Option<XRayResult<GamedataScriptsVerificationResult>>,
  pub shaders_result: Option<XRayResult<GamedataShadersVerificationResult>>,
  pub sounds_result: Option<XRayResult<GamedataSoundsVerificationResult>>,
  pub spawns_result: Option<XRayResult<GamedataSpawnsVerificationResult>>,
  pub textures_result: Option<XRayResult<GamedataTexturesVerificationResult>>,
  pub weapons_result: Option<XRayResult<GamedataWeaponVerificationResult>>,
  pub weathers_result: Option<XRayResult<GamedataWeathersVerificationResult>>,
}

impl GamedataVerificationResult {
  pub fn is_valid(&self) -> bool {
    Self::is_optional_ok_and_valid(&self.animations_result)
      && Self::is_optional_ok_and_valid(&self.ltx_result)
      && Self::is_optional_ok_and_valid(&self.spawns_result)
      && Self::is_optional_ok_and_valid(&self.meshes_result)
      && Self::is_optional_ok_and_valid(&self.levels_result)
      && Self::is_optional_ok_and_valid(&self.particles_result)
      && Self::is_optional_ok_and_valid(&self.scripts_result)
      && Self::is_optional_ok_and_valid(&self.shaders_result)
      && Self::is_optional_ok_and_valid(&self.sounds_result)
      && Self::is_optional_ok_and_valid(&self.textures_result)
      && Self::is_optional_ok_and_valid(&self.weapons_result)
      && Self::is_optional_ok_and_valid(&self.weathers_result)
  }

  pub fn get_failure_messages(&self) -> Vec<String> {
    vec![
      Self::get_optional_result_failure_message(&self.animations_result, "animations"),
      Self::get_optional_result_failure_message(&self.ltx_result, "ltx"),
      Self::get_optional_result_failure_message(&self.levels_result, "levels"),
      Self::get_optional_result_failure_message(&self.meshes_result, "meshes"),
      Self::get_optional_result_failure_message(&self.particles_result, "particles"),
      Self::get_optional_result_failure_message(&self.scripts_result, "scripts"),
      Self::get_optional_result_failure_message(&self.shaders_result, "shaders"),
      Self::get_optional_result_failure_message(&self.sounds_result, "sounds"),
      Self::get_optional_result_failure_message(&self.spawns_result, "spawns"),
      Self::get_optional_result_failure_message(&self.textures_result, "textures"),
      Self::get_optional_result_failure_message(&self.weapons_result, "weapons"),
      Self::get_optional_result_failure_message(&self.weathers_result, "weathers"),
    ]
    .into_iter()
    .flatten()
    .collect()
  }

  fn is_optional_ok_and_valid<T>(result: &Option<XRayResult<T>>) -> bool
  where
    T: GamedataGenericVerificationResult,
  {
    T::is_optional_ok_and_valid(result)
  }

  fn get_optional_result_failure_message<T>(
    result: &Option<XRayResult<T>>,
    comment: &str,
  ) -> Option<String>
  where
    T: GamedataGenericVerificationResult,
  {
    T::get_optional_result_failure_message(result, comment)
  }
}
