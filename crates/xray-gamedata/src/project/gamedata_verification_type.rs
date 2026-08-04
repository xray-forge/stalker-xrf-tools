use crate::{
  GamedataCheckResult, GamedataProject, GamedataProjectVerifyOptions,
  GamedataVerificationCheckReport,
};
use derive_more::Display;
use std::str::FromStr;
use xray_error::XRayError;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Display)]
pub enum GamedataVerificationType {
  #[display("animations")]
  Animations,
  #[display("levels")]
  Levels,
  #[display("ltx")]
  Ltx,
  #[display("meshes")]
  Meshes,
  #[display("particles")]
  Particles,
  #[display("particles-usage")]
  ParticlesUsage,
  #[display("scripts")]
  Scripts,
  #[display("shaders")]
  Shaders,
  #[display("sounds")]
  Sounds,
  #[display("spawns")]
  Spawns,
  #[display("textures")]
  Textures,
  #[display("weapons")]
  Weapons,
  #[display("weathers")]
  Weathers,
}

impl GamedataVerificationType {
  pub const ALL: [Self; 13] = [
    Self::Animations,
    Self::Levels,
    Self::Ltx,
    Self::Meshes,
    Self::Particles,
    Self::ParticlesUsage,
    Self::Scripts,
    Self::Shaders,
    Self::Sounds,
    Self::Spawns,
    Self::Textures,
    Self::Weapons,
    Self::Weathers,
  ];

  pub fn get_all() -> Vec<GamedataVerificationType> {
    Self::ALL.to_vec()
  }

  pub fn run(
    self,
    project: &GamedataProject,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataVerificationCheckReport {
    match self {
      Self::Animations => Self::check_report(self, project.verify_animations(options)),
      Self::Levels => Self::check_report(self, project.verify_levels(options)),
      Self::Ltx => Self::check_report(self, project.verify_ltx(options)),
      Self::Meshes => Self::check_report(self, project.verify_meshes(options)),
      Self::Particles => Self::check_report(self, project.verify_particles(options)),
      Self::ParticlesUsage => Self::check_report(self, project.verify_particles_usage(options)),
      Self::Scripts => Self::check_report(self, project.verify_scripts(options)),
      Self::Shaders => Self::check_report(self, project.verify_shaders(options)),
      Self::Sounds => Self::check_report(self, project.verify_sounds(options)),
      Self::Spawns => Self::check_report(self, project.verify_spawns(options)),
      Self::Textures => Self::check_report(self, project.verify_textures(options)),
      Self::Weapons => Self::check_report(self, project.verify_weapons(options)),
      Self::Weathers => Self::check_report(self, project.verify_weathers(options)),
    }
  }

  fn check_report<T>(
    verification_type: Self,
    result: xray_error::XRayResult<T>,
  ) -> GamedataVerificationCheckReport
  where
    T: GamedataCheckResult,
  {
    GamedataVerificationCheckReport::from_check_result(verification_type, result)
  }
}

impl FromStr for GamedataVerificationType {
  type Err = XRayError;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    Self::ALL
      .into_iter()
      .find(|verification_type| verification_type.to_string() == string)
      .ok_or_else(|| {
        XRayError::new_unexpected_error(format!(
          "Unexpected verification type '{verification}' provided",
          verification = string
        ))
      })
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataVerificationType;

  #[test]
  fn parses_every_registered_verification_type() {
    for verification_type in GamedataVerificationType::ALL {
      let parsed: GamedataVerificationType = verification_type
        .to_string()
        .parse()
        .expect("Expected verification type to parse");

      assert_eq!(parsed, verification_type);
    }
  }
}
