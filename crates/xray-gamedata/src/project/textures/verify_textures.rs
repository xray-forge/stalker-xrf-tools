use crate::asset::asset_type::AssetType;
use crate::project::textures::verify_textures_result::GamedataTexturesVerificationResult;
use crate::{
  GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationFinding,
  GamedataVerificationRule,
};
use colored::Colorize;
use ddsfile::{Dds, DxgiFormat};
use rayon::prelude::*;
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use xray_error::{XRayError, XRayResult};

impl GamedataProject {
  pub fn verify_textures(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataTexturesVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify textures:".green());
    }

    let started_at: Instant = Instant::now();
    let texture_paths: Vec<String> = self.get_all_asset_paths_by_type(AssetType::Dds);
    let checked_textures_count: u32 = u32::try_from(texture_paths.len()).map_err(|_| {
      XRayError::new_verify_error("Texture count exceeds the supported result range")
    })?;

    let mut findings: Vec<GamedataVerificationFinding> = texture_paths
      .par_iter()
      .filter_map(|relative_path| {
        if options.is_verbose_logging_enabled() {
          println!("Verify texture: {relative_path}");
        }

        let Some(path) = self.get_absolute_asset_path(relative_path) else {
          if options.is_logging_enabled() {
            println!("Texture path not found: {relative_path}");
          }

          return Some(GamedataVerificationFinding::for_asset(
            GamedataVerificationRule::TexturesPath,
            Path::new(relative_path),
            "Texture path was not found in gamedata roots",
          ));
        };

        match self.verify_texture_by_path(options, &path) {
          Ok(true) => None,
          Ok(false) => {
            if options.is_logging_enabled() {
              println!("Texture is not valid: {}", path.display());
            }

            Some(GamedataVerificationFinding::for_asset(
              GamedataVerificationRule::TexturesValidation,
              &path,
              "Texture uses an unsupported format",
            ))
          }
          Err(error) => {
            if options.is_logging_enabled() {
              println!(
                "Texture verification failed: {} - {}",
                path.display(),
                error
              );
            }

            Some(GamedataVerificationFinding::for_asset(
              GamedataVerificationRule::TexturesRead,
              &path,
              error.to_string(),
            ))
          }
        }
      })
      .collect();

    let duration: u128 = started_at.elapsed().as_millis();
    let invalid_textures_count: u32 = u32::try_from(findings.len()).map_err(|_| {
      XRayError::new_verify_error("Invalid texture count exceeds the supported result range")
    })?;

    findings.sort_by(|left, right| left.asset_path.cmp(&right.asset_path));

    if options.is_logging_enabled() {
      println!(
        "Verified gamedata textures in {} sec, {}/{} valid",
        (duration as f64) / 1000.0,
        checked_textures_count - invalid_textures_count,
        checked_textures_count
      );
    }

    Ok(GamedataTexturesVerificationResult {
      duration,
      findings,
      invalid_textures_count,
      checked_textures_count,
    })
  }

  pub fn verify_texture_by_path(
    &self,
    options: &GamedataProjectVerifyOptions,
    path: &Path,
  ) -> XRayResult<bool> {
    self.verify_texture(
      options,
      &Dds::read(&mut File::open(path)?).map_err(|error| {
        XRayError::new_verify_error(format!(
          "Failed to read texture by path {}, error: {}",
          path.display(),
          error
        ))
      })?,
    )
  }

  pub fn verify_texture(
    &self,
    _options: &GamedataProjectVerifyOptions,
    dds: &Dds,
  ) -> XRayResult<bool> {
    let mut is_valid: bool = true;

    if let Some(header10) = &dds.header10 {
      if !Self::is_supported_texture_format(header10.dxgi_format) {
        is_valid = false;
      }
    } else if let Some(format) = DxgiFormat::try_from_pixel_format(&dds.header.spf) {
      if !Self::is_supported_texture_format(format) {
        is_valid = false;
      }
    } else {
      // Unknown format:
      // is_valid = false;
    }

    // todo: Verify bump availability?

    Ok(is_valid)
  }
}

impl GamedataProject {
  pub fn is_supported_texture_format(format: DxgiFormat) -> bool {
    matches!(
      format,
      DxgiFormat::BC1_UNorm
        | DxgiFormat::BC1_UNorm_sRGB
        | DxgiFormat::BC2_UNorm
        | DxgiFormat::BC2_UNorm_sRGB
        | DxgiFormat::BC3_UNorm
        | DxgiFormat::BC3_UNorm_sRGB
    )
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataProject;
  use crate::GamedataProjectVerifyOptions;
  use ddsfile::{AlphaMode, D3D10ResourceDimension, Dds, DxgiFormat, NewDxgiParams};
  use std::collections::HashMap;
  use std::path::PathBuf;
  use xray_ltx::LtxProject;

  fn empty_project() -> GamedataProject {
    GamedataProject {
      assets: HashMap::new(),
      ltx_project: LtxProject {
        root: PathBuf::new(),
        ltx_file_entries: Vec::new(),
        ltx_files: Vec::new(),
        ltx_scheme_files: Vec::new(),
        ltx_scheme_file_entries: Vec::new(),
        ltx_scheme_declarations: Default::default(),
      },
      root: PathBuf::new(),
    }
  }

  fn dx10_texture(format: DxgiFormat) -> Dds {
    Dds::new_dxgi(NewDxgiParams {
      height: 4,
      width: 4,
      depth: None,
      format,
      mipmap_levels: None,
      array_layers: None,
      caps2: None,
      is_cubemap: false,
      resource_dimension: D3D10ResourceDimension::Texture2D,
      alpha_mode: AlphaMode::Unknown,
    })
    .expect("Expected test DDS to be constructible")
  }

  #[test]
  fn accepts_supported_dx10_formats_and_rejects_unsupported_formats() {
    let project = empty_project();
    let options = GamedataProjectVerifyOptions::default();

    assert!(
      project
        .verify_texture(&options, &dx10_texture(DxgiFormat::BC1_UNorm_sRGB))
        .unwrap()
    );
    assert!(
      project
        .verify_texture(&options, &dx10_texture(DxgiFormat::BC3_UNorm))
        .unwrap()
    );
    assert!(
      !project
        .verify_texture(&options, &dx10_texture(DxgiFormat::BC4_UNorm))
        .unwrap()
    );
  }
}
