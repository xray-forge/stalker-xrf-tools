use crate::asset::asset_type::AssetType;
use crate::project::textures::verify_textures_result::GamedataTexturesVerificationResult;
use crate::{GamedataError, GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;
use ddsfile::{Dds, DxgiFormat};
use rayon::prelude::*;
use std::fs::File;
use std::path::Path;
use std::sync::Mutex;
use std::time::Instant;

impl GamedataProject {
  pub fn verify_textures(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataTexturesVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata textures:".green());
    }

    let started_at: Instant = Instant::now();
    let checked_textures_count: Mutex<u32> = Mutex::new(0);
    let invalid_textures_count: Mutex<u32> = Mutex::new(0);

    self
      .get_all_asset_paths_by_type(AssetType::Dds)
      .par_iter()
      .for_each(|path| {
        if options.is_verbose_logging_enabled() {
          println!("Verify gamedata texture: {}", path);
        }

        *checked_textures_count.lock().unwrap() += 1;

        if let Some(path) = self.get_absolute_asset_path(path) {
          match self.verify_texture_by_path(options, &path) {
            Ok(is_valid) => {
              if !is_valid {
                if options.is_logging_enabled() {
                  println!("Texture is not valid: {}", path.display());
                }

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

              *invalid_textures_count.lock().unwrap() += 1;
            }
          }
        } else {
          if options.is_logging_enabled() {
            println!("Texture path not found: {}", path);
          }

          *invalid_textures_count.lock().unwrap() += 1;
        }
      });

    let duration: u128 = started_at.elapsed().as_millis();
    let checked_textures_count: u32 = *checked_textures_count.lock().unwrap();
    let invalid_textures_count: u32 = *invalid_textures_count.lock().unwrap();

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
      invalid_textures_count,
      checked_textures_count,
    })
  }

  pub fn verify_texture_by_path(
    &self,
    options: &GamedataProjectVerifyOptions,
    path: &Path,
  ) -> GamedataResult<bool> {
    self.verify_texture(
      options,
      &Dds::read(&mut File::open(path)?).map_err(|error| {
        GamedataError::new_check_error(format!(
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
  ) -> GamedataResult<bool> {
    let mut is_valid: bool = true;

    if let Some(header10) = &dds.header10 {
      if Self::is_supported_texture_format(header10.dxgi_format) {
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
    format == DxgiFormat::BC1_UNorm_sRGB
      || format == DxgiFormat::BC2_UNorm_sRGB
      || format == DxgiFormat::BC3_UNorm_sRGB
  }
}
