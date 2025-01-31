use derive_more::Display;
use std::str::FromStr;
use xray_error::XRayError;

#[derive(Clone, Debug, PartialEq, Display)]
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
  pub fn get_all() -> Vec<GamedataVerificationType> {
    vec![
      GamedataVerificationType::Animations,
      GamedataVerificationType::Levels,
      GamedataVerificationType::Ltx,
      GamedataVerificationType::Meshes,
      GamedataVerificationType::Particles,
      GamedataVerificationType::Scripts,
      GamedataVerificationType::Shaders,
      GamedataVerificationType::Sounds,
      GamedataVerificationType::Spawns,
      GamedataVerificationType::Textures,
      GamedataVerificationType::Weapons,
      GamedataVerificationType::Weathers,
    ]
  }

  pub fn contains_and_then<T, F>(&self, checks: &[Self], cb: F) -> Option<T>
  where
    F: Fn() -> T,
  {
    if checks.contains(self) {
      Some(cb())
    } else {
      None
    }
  }
}

impl FromStr for GamedataVerificationType {
  type Err = XRayError;

  fn from_str(language: &str) -> Result<Self, Self::Err> {
    match language {
      "animations" => Ok(Self::Animations),
      "levels" => Ok(Self::Levels),
      "ltx" => Ok(Self::Ltx),
      "meshes" => Ok(Self::Meshes),
      "particles" => Ok(Self::Particles),
      "scripts" => Ok(Self::Scripts),
      "shaders" => Ok(Self::Shaders),
      "sounds" => Ok(Self::Sounds),
      "spawns" => Ok(Self::Spawns),
      "textures" => Ok(Self::Textures),
      "weapons" => Ok(Self::Weapons),
      "weathers" => Ok(Self::Weathers),
      verification => Err(XRayError::new_unexpected_error(format!(
        "Unexpected verification type '{verification}' provided",
      ))),
    }
  }
}
