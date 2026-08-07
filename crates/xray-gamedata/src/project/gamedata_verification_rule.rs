use derive_more::Display;

/// Stable rule identifiers assigned to gamedata verification findings.
#[derive(Clone, Copy, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum GamedataVerificationRule {
  #[display("animations.hud-item")]
  AnimationsHudItem,
  #[display("animations.motion-collision")]
  AnimationsMotionCollision,
  #[display("animations.player-hud")]
  AnimationsPlayerHud,
  #[display("checks.execution")]
  CheckExecution,
  #[display("ltx.formatting")]
  LtxFormatting,
  #[display("ltx.schema")]
  LtxSchema,
  #[display("ltx.verification")]
  LtxVerification,
  #[display("meshes.motion-read")]
  MeshesMotionRead,
  #[display("meshes.motion-validation")]
  MeshesMotionValidation,
  #[display("meshes.path")]
  MeshesPath,
  #[display("meshes.read")]
  MeshesRead,
  #[display("meshes.shader-library")]
  MeshesShaderLibrary,
  #[display("meshes.validation")]
  MeshesValidation,
  #[display("particles.library")]
  ParticlesLibrary,
  #[display("particles.texture")]
  ParticlesTexture,
  #[display("particles-usage.reference")]
  ParticlesUsageReference,
  #[display("particles-usage.spawn")]
  ParticlesUsageSpawn,
  #[display("particles-usage.spawn-custom-data")]
  ParticlesUsageSpawnCustomData,
  #[display("scripts.path")]
  ScriptsPath,
  #[display("scripts.read")]
  ScriptsRead,
  #[display("scripts.syntax")]
  ScriptsSyntax,
  #[display("shaders.include-cycle")]
  ShadersIncludeCycle,
  #[display("shaders.include-missing")]
  ShadersIncludeMissing,
  #[display("shaders.include-syntax")]
  ShadersIncludeSyntax,
  #[display("shaders.lua-syntax")]
  ShadersLuaSyntax,
  #[display("shaders.renderer-root")]
  ShadersRendererRoot,
  #[display("shaders.source-invalid")]
  ShadersSourceInvalid,
  #[display("shaders.source-read")]
  ShadersSourceRead,
  #[display("sounds.files")]
  SoundsFiles,
  #[display("sounds.references")]
  SoundsReferences,
  #[display("spawns.path")]
  SpawnsPath,
  #[display("spawns.read")]
  SpawnsRead,
  #[display("textures.path")]
  TexturesPath,
  #[display("textures.read")]
  TexturesRead,
  #[display("textures.dds")]
  TexturesValidation,
  #[display("weapons.validation")]
  WeaponsValidation,
  #[display("weathers.definitions")]
  WeathersDefinitions,
  #[display("weathers.files")]
  WeathersFiles,
  #[display("weathers.validation")]
  WeathersValidation,
}

#[cfg(test)]
mod tests {
  use super::GamedataVerificationRule;

  #[test]
  fn displays_stable_rule_ids() {
    assert_eq!(
      GamedataVerificationRule::ParticlesUsageSpawnCustomData.to_string(),
      "particles-usage.spawn-custom-data"
    );
    assert_eq!(
      GamedataVerificationRule::ShadersIncludeMissing.to_string(),
      "shaders.include-missing"
    );
  }
}
