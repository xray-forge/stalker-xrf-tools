use crate::asset::asset_type::AssetType;
use crate::project::textures::verify_textures_result::GamedataTexturesVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationFinding};
use colored::Colorize;
use ddsfile::{Dds, DxgiFormat};
use rayon::prelude::*;
use std::fs::File;
use std::path::Path;
use std::sync::Mutex;
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
    let checked_textures_count: Mutex<u32> = Mutex::new(0);
    let findings: Mutex<Vec<GamedataVerificationFinding>> = Mutex::new(Vec::new());
    let invalid_textures_count: Mutex<u32> = Mutex::new(0);

    self
      .get_all_asset_paths_by_type(AssetType::Dds)
      .par_iter()
      .for_each(|path| {
        if options.is_verbose_logging_enabled() {
          println!("Verify texture: {}", path);
        }

        *checked_textures_count.lock().unwrap() += 1;

        if let Some(path) = self.get_absolute_asset_path(path) {
          match self.verify_texture_by_path(options, &path) {
            Ok(is_valid) => {
              if !is_valid {
                if options.is_logging_enabled() {
                  println!("Texture is not valid: {}", path.display());
                }

                findings
                  .lock()
                  .unwrap()
                  .push(GamedataVerificationFinding::for_asset(
                    &path,
                    "Texture uses an unsupported format",
                  ));
                *invalid_textures_count.lock().unwrap() += 1;
              }
            }
            Err(error) => {
              if options.is_logging_enabled() {
                println!(
                  "Texture verification failed: {} - {}",
                  path.display(),
                  error
                );
              }

              findings
                .lock()
                .unwrap()
                .push(GamedataVerificationFinding::for_asset(
                  &path,
                  error.to_string(),
                ));
              *invalid_textures_count.lock().unwrap() += 1;
            }
          }
        } else {
          if options.is_logging_enabled() {
            println!("Texture path not found: {}", path);
          }

          findings
            .lock()
            .unwrap()
            .push(GamedataVerificationFinding::for_asset(
              Path::new(path),
              "Texture path was not found in gamedata roots",
            ));
          *invalid_textures_count.lock().unwrap() += 1;
        }
      });

    let duration: u128 = started_at.elapsed().as_millis();
    let checked_textures_count: u32 = *checked_textures_count.lock().unwrap();
    let invalid_textures_count: u32 = *invalid_textures_count.lock().unwrap();
    let mut findings: Vec<GamedataVerificationFinding> =
      std::mem::take(&mut *findings.lock().unwrap());

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
