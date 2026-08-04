use crate::GamedataFindingFactory;
use crate::project::shaders::gamedata_shader_source_loader::GamedataShaderSourceLoader;
use crate::project::shaders::verify_shaders_result::GamedataShadersVerificationResult;
use crate::{GamedataCheckResult, GamedataProjectVerifyOptions, GamedataVerificationRule};
use colored::Colorize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::{DirEntry, WalkDir};
use xray_error::XRayError;
use xray_shaders::{
  SHADER_SCRIPT_FILE_EXTENSION, ShaderRenderer, XRayShader, XRayShaderScript, is_shader_source_path,
};

pub(crate) struct ShadersVerifier<'a> {
  options: &'a GamedataProjectVerifyOptions,
  shaders_root: PathBuf,
}

impl<'a> ShadersVerifier<'a> {
  pub(crate) fn new(shaders_root: PathBuf, options: &'a GamedataProjectVerifyOptions) -> Self {
    Self {
      options,
      shaders_root,
    }
  }

  pub(crate) fn verify(&self) -> GamedataShadersVerificationResult {
    if self.options.is_logging_enabled() {
      println!("{}", "Verify renderer shaders:".green());
    }

    let started_at: Instant = Instant::now();

    let mut result: GamedataShadersVerificationResult =
      GamedataShadersVerificationResult::default();

    for renderer in [ShaderRenderer::DirectX11, ShaderRenderer::OpenGl] {
      self.verify_renderer(renderer, &mut result);
    }

    result.sort_findings();

    result.duration = started_at.elapsed();

    if self.options.is_logging_enabled() {
      println!(
        "Verified renderer shaders in {} sec, {}",
        result.duration.as_secs_f64(),
        result.failure_message()
      );
    }

    result
  }

  fn verify_renderer(
    &self,
    renderer: ShaderRenderer,
    result: &mut GamedataShadersVerificationResult,
  ) {
    if self.options.is_verbose_logging_enabled() {
      println!("Verify {} renderer shaders", renderer.display_name());
    }

    let renderer_root: PathBuf = self.shaders_root.join(renderer.directory_name());

    if !renderer_root.is_dir() {
      result.add_finding(GamedataFindingFactory::for_asset(
        GamedataVerificationRule::ShadersRendererRoot,
        &renderer_root,
        format!(
          "{} renderer shader root is missing",
          renderer.display_name()
        ),
      ));

      return;
    }

    self.verify_root_shader_scripts(&renderer_root, result);

    let source_loader: GamedataShaderSourceLoader = GamedataShaderSourceLoader;
    let mut checked_sources: HashSet<PathBuf> = HashSet::new();

    for entry in WalkDir::new(&renderer_root) {
      match entry {
        Ok(entry) if Self::is_shader_source_file(&entry) => self.verify_shader_source(
          entry.path(),
          renderer,
          result,
          &source_loader,
          &mut checked_sources,
        ),
        Ok(_) => {}
        Err(error) => result.add_finding(GamedataFindingFactory::for_asset(
          GamedataVerificationRule::ShadersSourceRead,
          error.path().unwrap_or(&renderer_root),
          format!("Failed to traverse renderer shader sources: {error}"),
        )),
      }
    }
  }

  fn verify_root_shader_scripts(
    &self,
    renderer_root: &Path,
    result: &mut GamedataShadersVerificationResult,
  ) {
    let entries = match fs::read_dir(renderer_root) {
      Ok(entries) => entries,
      Err(error) => {
        result.add_finding(GamedataFindingFactory::for_asset(
          GamedataVerificationRule::ShadersSourceRead,
          renderer_root,
          format!("Failed to read renderer shader root: {error}"),
        ));

        return;
      }
    };

    for entry in entries {
      let entry = match entry {
        Ok(entry) => entry,
        Err(error) => {
          result.add_finding(GamedataFindingFactory::for_asset(
            GamedataVerificationRule::ShadersSourceRead,
            renderer_root,
            format!("Failed to read renderer shader entry: {error}"),
          ));
          continue;
        }
      };
      let path: PathBuf = entry.path();

      if !path.is_file() || !Self::has_extension(&path, SHADER_SCRIPT_FILE_EXTENSION) {
        continue;
      }

      result.increment_checked_scripts_count();

      match fs::read_to_string(&path) {
        Ok(source) => {
          if let Err(error) = XRayShaderScript::parse(&path, &source) {
            result.add_finding(GamedataFindingFactory::for_asset(
              GamedataVerificationRule::ShadersLuaSyntax,
              &path,
              error.to_string(),
            ));
          }
        }
        Err(error) => result.add_finding(GamedataFindingFactory::for_asset(
          GamedataVerificationRule::ShadersSourceRead,
          &path,
          format!("Failed to read shader script: {error}"),
        )),
      }
    }
  }

  fn verify_shader_source(
    &self,
    path: &Path,
    renderer: ShaderRenderer,
    result: &mut GamedataShadersVerificationResult,
    source_loader: &GamedataShaderSourceLoader,
    checked_sources: &mut HashSet<PathBuf>,
  ) {
    if checked_sources.contains(path) {
      return;
    }

    match XRayShader::load(path, renderer, &self.shaders_root, source_loader) {
      Ok(shader) => Self::record_checked_shader_sources(&shader, checked_sources, result),
      Err(error) => {
        checked_sources.insert(path.to_path_buf());
        result.increment_checked_sources_count();
        result.add_finding(GamedataFindingFactory::for_asset(
          Self::shader_error_rule_id(&error),
          path,
          error.to_string(),
        ));
      }
    }
  }

  fn is_shader_source_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file() && is_shader_source_path(entry.path())
  }

  fn has_extension(path: &Path, extension: &str) -> bool {
    path
      .extension()
      .and_then(|value| value.to_str())
      .is_some_and(|value| value.eq_ignore_ascii_case(extension))
  }

  fn shader_error_rule_id(error: &XRayError) -> GamedataVerificationRule {
    match error {
      XRayError::Invalid { .. } => GamedataVerificationRule::ShadersIncludeSyntax,
      XRayError::NotFound { .. } => GamedataVerificationRule::ShadersIncludeMissing,
      XRayError::Read { .. } | XRayError::Io { .. } => GamedataVerificationRule::ShadersSourceRead,
      XRayError::Verify { .. } => GamedataVerificationRule::ShadersIncludeCycle,
      _ => GamedataVerificationRule::ShadersSourceInvalid,
    }
  }

  fn record_checked_shader_sources(
    shader: &XRayShader,
    checked_sources: &mut HashSet<PathBuf>,
    result: &mut GamedataShadersVerificationResult,
  ) {
    if !checked_sources.insert(shader.path().to_path_buf()) {
      return;
    }

    result.increment_checked_sources_count();

    for import in shader.imports() {
      Self::record_checked_shader_sources(import.shader(), checked_sources, result);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::ShadersVerifier;
  use crate::{GamedataCheckResult, GamedataProjectVerifyOptions, GamedataVerificationRule};
  use std::fs;
  use std::path::{Path, PathBuf};
  use xray_error::{XRayError, XRayResult};

  #[test]
  fn validates_d3d11_scripts_and_renderer_then_root_includes() -> XRayResult {
    let root: PathBuf = create_shader_root("d3d11")?;
    let options: GamedataProjectVerifyOptions = GamedataProjectVerifyOptions {
      is_silent: true,
      ..Default::default()
    };

    write_file(
      &root.join("r3/basic.s"),
      "function normal(shader, t_base, t_second, t_detail) end\n",
    )?;
    write_file(&root.join("r3/main.ps"), "#include \"shared/common.h\"\n")?;
    write_file(&root.join("shared/common.h"), "float value;\n")?;

    let result = ShadersVerifier::new(root.clone(), &options).verify();

    assert_eq!(
      result.failure_message(),
      "1 shader scripts and 2 shader sources checked, 0 problems"
    );
    assert!(result.findings().is_empty());

    fs::remove_dir_all(root)?;

    Ok(())
  }

  #[test]
  fn reports_lua_include_and_cycle_problems_together() -> XRayResult {
    let root: PathBuf = create_shader_root("static-problems")?;
    let options: GamedataProjectVerifyOptions = GamedataProjectVerifyOptions {
      is_silent: true,
      ..Default::default()
    };

    write_file(&root.join("gl/invalid.s"), "function normal(\n")?;
    write_file(&root.join("gl/missing.ps"), "#include \"missing.h\"\n")?;
    write_file(&root.join("gl/first.h"), "#include \"second.h\"\n")?;
    write_file(&root.join("gl/second.h"), "#include \"first.h\"\n")?;

    let result = ShadersVerifier::new(root.clone(), &options).verify();
    let rule_ids: Vec<String> = result
      .findings()
      .iter()
      .map(|finding| finding.rule_id().to_string())
      .collect();

    assert!(rule_ids.contains(&GamedataVerificationRule::ShadersLuaSyntax.to_string()));
    assert!(rule_ids.contains(&GamedataVerificationRule::ShadersIncludeMissing.to_string()));
    assert!(rule_ids.contains(&GamedataVerificationRule::ShadersIncludeCycle.to_string()));

    fs::remove_dir_all(root)?;

    Ok(())
  }

  fn create_shader_root(test_name: &str) -> XRayResult<PathBuf> {
    let root: PathBuf = std::env::temp_dir()
      .join("xray-gamedata-shader-tests")
      .join(test_name);

    if root.exists() {
      fs::remove_dir_all(&root)?;
    }

    fs::create_dir_all(root.join("r3"))?;
    fs::create_dir_all(root.join("gl"))?;

    Ok(root)
  }

  fn write_file(path: &Path, contents: &str) -> XRayResult {
    let parent: &Path = path.parent().ok_or_else(|| {
      XRayError::new_unexpected_error(format!(
        "Shader test path has no parent: {}",
        path.display()
      ))
    })?;

    fs::create_dir_all(parent)?;
    fs::write(path, contents)?;

    Ok(())
  }
}
