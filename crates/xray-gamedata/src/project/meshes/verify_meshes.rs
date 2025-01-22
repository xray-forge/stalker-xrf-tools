use crate::asset::asset_type::AssetType;
use crate::project::meshes::verify_meshes_result::GamedataMeshesVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;
use std::path::Path;
use xray_db::{OgfFile, XRayByteOrder};

impl GamedataProject {
  pub fn verify_meshes(
    &mut self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataMeshesVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata meshes:".green());
    }

    let mut checked_meshes_count: usize = 0;
    let mut invalid_meshes_count: usize = 0;

    let meshes: Vec<String> = self
      .assets
      .iter()
      .filter_map(|(path, descriptor)| {
        if descriptor.asset_type == AssetType::Ogf {
          Some(path.clone())
        } else {
          None
        }
      })
      .collect::<Vec<_>>();

    for path in meshes {
      if options.is_verbose_logging_enabled() {
        println!("Verify gamedata mesh: {}", path);
      }

      checked_meshes_count += 1;

      if let Some(path) = self.get_absolute_asset_path_hit(&path) {
        match self.verify_mesh(options, &path) {
          Ok(is_valid) => {
            if !is_valid {
              invalid_meshes_count += 1;
            }
          }
          Err(error) => {
            if options.is_logging_enabled() {
              println!("Mesh verification failed: {:?} - {error}", path);
            }

            invalid_meshes_count += 1;
          }
        }
      } else {
        invalid_meshes_count += 1;
      }
    }

    if options.is_logging_enabled() {
      println!(
        "Verified gamedata meshes, {}/{checked_meshes_count} valid",
        checked_meshes_count - invalid_meshes_count,
      );
    }

    Ok(GamedataMeshesVerificationResult {
      invalid_meshes: invalid_meshes_count as u64,
      checked_meshes: checked_meshes_count as u64,
    })
  }

  pub fn verify_mesh(
    &self,
    options: &GamedataProjectVerifyOptions,
    path: &Path,
  ) -> GamedataResult<bool> {
    let ogf: OgfFile = OgfFile::read_from_path::<XRayByteOrder, &Path>(path)?;

    self.verify_mesh_textures(options, &ogf)
  }

  pub fn verify_mesh_textures(
    &self,
    options: &GamedataProjectVerifyOptions,
    ogf: &OgfFile,
  ) -> GamedataResult<bool> {
    Ok(true)
  }
}
