use crate::GamedataCheckResult;
use crate::GamedataVerificationStatus;
use crate::project::animations::verify_animations_result::GamedataAnimationsVerificationResult;
use xray_error::XRayResult;

use crate::project::levels::verify_levels_result::GamedataLevelVerificationResult;
use crate::project::ltx::verify_ltx_result::GamedataLtxVerificationResult;
use crate::project::meshes::verify_meshes_result::GamedataMeshesVerificationResult;
use crate::project::particles::verify_particles_result::GamedataParticlesVerificationResult;
use crate::project::particles::verify_particles_usage_result::GamedataParticlesUsageVerificationResult;
use crate::project::scripts::verify_scripts_result::GamedataScriptsVerificationResult;
use crate::project::shaders::verify_shaders_result::GamedataShadersVerificationResult;
use crate::project::sounds::verify_sounds_result::GamedataSoundsVerificationResult;
use crate::project::spawns::verify_spawns_result::GamedataSpawnsVerificationResult;
use crate::project::textures::verify_textures_result::GamedataTexturesVerificationResult;
use crate::project::weapons::verify_weapons_result::GamedataWeaponVerificationResult;
use crate::project::weathers::verify_weathers_result::GamedataWeathersVerificationResult;

#[derive(Default)]
pub struct GamedataVerificationResult {
  pub duration: u128,
  pub animations_result: Option<XRayResult<GamedataAnimationsVerificationResult>>,
  pub ltx_result: Option<XRayResult<GamedataLtxVerificationResult>>,
  pub levels_result: Option<XRayResult<GamedataLevelVerificationResult>>,
  pub meshes_result: Option<XRayResult<GamedataMeshesVerificationResult>>,
  pub particles_result: Option<XRayResult<GamedataParticlesVerificationResult>>,
  pub particles_usage_result: Option<XRayResult<GamedataParticlesUsageVerificationResult>>,
  pub scripts_result: Option<XRayResult<GamedataScriptsVerificationResult>>,
  pub shaders_result: Option<XRayResult<GamedataShadersVerificationResult>>,
  pub sounds_result: Option<XRayResult<GamedataSoundsVerificationResult>>,
  pub spawns_result: Option<XRayResult<GamedataSpawnsVerificationResult>>,
  pub textures_result: Option<XRayResult<GamedataTexturesVerificationResult>>,
  pub weapons_result: Option<XRayResult<GamedataWeaponVerificationResult>>,
  pub weathers_result: Option<XRayResult<GamedataWeathersVerificationResult>>,
}

impl GamedataVerificationResult {
  pub fn status(&self) -> GamedataVerificationStatus {
    Self::aggregate_status([
      Self::get_optional_result_status(&self.animations_result),
      Self::get_optional_result_status(&self.ltx_result),
      Self::get_optional_result_status(&self.levels_result),
      Self::get_optional_result_status(&self.meshes_result),
      Self::get_optional_result_status(&self.particles_result),
      Self::get_optional_result_status(&self.particles_usage_result),
      Self::get_optional_result_status(&self.scripts_result),
      Self::get_optional_result_status(&self.shaders_result),
      Self::get_optional_result_status(&self.sounds_result),
      Self::get_optional_result_status(&self.spawns_result),
      Self::get_optional_result_status(&self.textures_result),
      Self::get_optional_result_status(&self.weapons_result),
      Self::get_optional_result_status(&self.weathers_result),
    ])
  }

  pub fn is_valid(&self) -> bool {
    self.status() == GamedataVerificationStatus::Passed
  }

  pub fn get_failure_messages(&self) -> Vec<String> {
    vec![
      Self::get_optional_result_failure_message(&self.animations_result, "animations"),
      Self::get_optional_result_failure_message(&self.ltx_result, "ltx"),
      Self::get_optional_result_failure_message(&self.levels_result, "levels"),
      Self::get_optional_result_failure_message(&self.meshes_result, "meshes"),
      Self::get_optional_result_failure_message(&self.particles_result, "particles"),
      Self::get_optional_result_failure_message(&self.particles_usage_result, "particles-usage"),
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

  fn get_optional_result_status<T>(result: &Option<XRayResult<T>>) -> GamedataVerificationStatus
  where
    T: GamedataCheckResult,
  {
    match result {
      Some(Ok(result)) => result.status(),
      Some(Err(_)) => GamedataVerificationStatus::Error,
      None => GamedataVerificationStatus::Skipped,
    }
  }

  fn get_optional_result_failure_message<T>(
    result: &Option<XRayResult<T>>,
    comment: &str,
  ) -> Option<String>
  where
    T: GamedataCheckResult,
  {
    match result {
      Some(Ok(result)) => match result.status() {
        GamedataVerificationStatus::Passed | GamedataVerificationStatus::Skipped => None,
        GamedataVerificationStatus::Failed
        | GamedataVerificationStatus::Error
        | GamedataVerificationStatus::Incomplete => Some(result.failure_message()),
      },
      Some(Err(error)) => Some(format!("Check failed ({comment}): {error}")),
      None => None,
    }
  }

  fn aggregate_status(
    statuses: impl IntoIterator<Item = GamedataVerificationStatus>,
  ) -> GamedataVerificationStatus {
    let mut aggregate: GamedataVerificationStatus = GamedataVerificationStatus::Skipped;

    for status in statuses {
      aggregate = match (aggregate, status) {
        (GamedataVerificationStatus::Error, _) | (_, GamedataVerificationStatus::Error) => {
          GamedataVerificationStatus::Error
        }
        (GamedataVerificationStatus::Incomplete, _)
        | (_, GamedataVerificationStatus::Incomplete) => GamedataVerificationStatus::Incomplete,
        (GamedataVerificationStatus::Failed, _) | (_, GamedataVerificationStatus::Failed) => {
          GamedataVerificationStatus::Failed
        }
        (GamedataVerificationStatus::Passed, _) | (_, GamedataVerificationStatus::Passed) => {
          GamedataVerificationStatus::Passed
        }
        _ => GamedataVerificationStatus::Skipped,
      };
    }

    aggregate
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataVerificationResult;
  use crate::GamedataVerificationStatus;
  use crate::project::animations::verify_animations_result::GamedataAnimationsVerificationResult;
  use crate::project::levels::verify_levels_result::GamedataLevelVerificationResult;
  use crate::project::shaders::verify_shaders_result::GamedataShadersVerificationResult;
  use crate::project::sounds::verify_sounds_result::GamedataSoundsVerificationResult;
  use crate::project::weathers::verify_weathers_result::GamedataWeathersVerificationResult;
  use xray_error::XRayError;

  #[test]
  fn aggregates_check_statuses_by_severity() {
    use GamedataVerificationStatus::{Error, Failed, Incomplete, Passed, Skipped};

    assert_eq!(GamedataVerificationResult::aggregate_status([]), Skipped);
    assert_eq!(
      GamedataVerificationResult::aggregate_status([Skipped, Passed]),
      Passed
    );
    assert_eq!(
      GamedataVerificationResult::aggregate_status([Passed, Incomplete]),
      Incomplete
    );
    assert_eq!(
      GamedataVerificationResult::aggregate_status([Failed, Incomplete]),
      Incomplete
    );
    assert_eq!(
      GamedataVerificationResult::aggregate_status([Incomplete, Error]),
      Error
    );
  }

  #[test]
  fn empty_verification_result_is_skipped_and_not_valid() {
    let result = GamedataVerificationResult::default();

    assert_eq!(result.status(), GamedataVerificationStatus::Skipped);
    assert!(!result.is_valid());
  }

  #[test]
  fn maps_check_results_and_checker_errors_to_statuses() {
    let mut result = GamedataVerificationResult {
      animations_result: Some(Ok(GamedataAnimationsVerificationResult::default())),
      ..Default::default()
    };

    assert_eq!(result.status(), GamedataVerificationStatus::Passed);

    result.animations_result = Some(Ok(GamedataAnimationsVerificationResult {
      checked_huds_count: 1,
      invalid_huds_count: 1,
      ..Default::default()
    }));

    assert_eq!(result.status(), GamedataVerificationStatus::Failed);
    assert_eq!(
      result.get_failure_messages(),
      vec![String::from("1/1 HUD animations are invalid")]
    );

    result.animations_result = Some(Err(XRayError::new_unexpected_error("boom")));

    assert_eq!(result.status(), GamedataVerificationStatus::Error);
    assert!(result.get_failure_messages()[0].contains("Check failed (animations):"));
  }

  #[test]
  fn reports_unimplemented_checks_as_incomplete() {
    let result = GamedataVerificationResult {
      levels_result: Some(Ok(GamedataLevelVerificationResult::default())),
      shaders_result: Some(Ok(GamedataShadersVerificationResult::default())),
      sounds_result: Some(Ok(GamedataSoundsVerificationResult::default())),
      weathers_result: Some(Ok(GamedataWeathersVerificationResult {
        checked_weather_files_count: 1,
        ..Default::default()
      })),
      ..Default::default()
    };

    assert_eq!(result.status(), GamedataVerificationStatus::Incomplete);
    assert_eq!(
      result.get_failure_messages(),
      vec![
        String::from("Level validation is not implemented"),
        String::from("Shader validation is not implemented"),
        String::from("Sound validation is not implemented"),
      ]
    );
  }
}
