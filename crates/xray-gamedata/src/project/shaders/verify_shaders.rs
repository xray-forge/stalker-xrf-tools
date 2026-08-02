use crate::project::shaders::verify_shaders_result::GamedataShadersVerificationResult;
use crate::{
  GamedataCheckResult, GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationFinding,
};
use colored::Colorize;
use std::path::PathBuf;
use std::time::Instant;
use xray_db::ShaderLibraryFile;
use xray_error::XRayResult;

impl GamedataProject {
  pub fn verify_shaders(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataShadersVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify shaders:".green());
    }

    let started_at: Instant = Instant::now();
    let path: PathBuf = self.get_shader_library_path();

    let mut result: GamedataShadersVerificationResult = GamedataShadersVerificationResult {
      checked_shader_libraries_count: 1,
      ..Default::default()
    };

    match ShaderLibraryFile::read_from_path(&path) {
      Ok(shader_library) => result.blender_count = shader_library.blenders_count(),
      Err(error) => {
        if options.is_logging_enabled() {
          eprintln!(
            "Shader library verification failed: {} - {}",
            path.display(),
            error
          );
        }

        result.findings.push(GamedataVerificationFinding::for_asset(
          &path,
          format!("Failed to read shader library: {error}"),
        ));
        result.invalid_shader_libraries_count = 1;
      }
    }

    result.duration = started_at.elapsed().as_millis();

    if options.is_logging_enabled() {
      println!(
        "Verified gamedata shader library in {} sec, {}",
        (result.duration as f64) / 1000.0,
        result.failure_message()
      );
    }

    Ok(result)
  }
}
