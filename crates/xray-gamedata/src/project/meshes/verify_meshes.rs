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

      if let Some(path) = self.get_absolute_asset_path(&path) {
        match self.verify_mesh_by_path(options, &path) {
          Ok(is_valid) => {
            if !is_valid {
              if options.is_logging_enabled() {
                println!("Mesh is not valid: {:?}", path);
              }

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
        if options.is_logging_enabled() {
          println!("Mesh path not found: {:?}", path);
        }

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

  pub fn verify_mesh_by_path(
    &mut self,
    options: &GamedataProjectVerifyOptions,
    path: &Path,
  ) -> GamedataResult<bool> {
    self.verify_mesh(
      options,
      &OgfFile::read_from_path::<XRayByteOrder, &Path>(path)?,
    )
  }

  pub fn verify_mesh(
    &mut self,
    options: &GamedataProjectVerifyOptions,
    ogf: &OgfFile,
  ) -> GamedataResult<bool> {
    let mut is_valid: bool = true;

    if !self.verify_mesh_textures(options, ogf)? {
      is_valid = false;
    }

    if let Some(children) = &ogf.children {
      for child in &children.nested {
        if !self.verify_mesh(options, child)? {
          is_valid = false;
        }
      }
    }

    // todo: Verify LOD?

    Ok(is_valid)
  }

  pub fn verify_mesh_textures(
    &mut self,
    options: &GamedataProjectVerifyOptions,
    ogf: &OgfFile,
  ) -> GamedataResult<bool> {
    let mut is_valid: bool = true;

    if let Some(texture) = &ogf.texture {
      if self.get_dds_path_hit(&texture.texture_name).is_none() {
        if options.is_logging_enabled() {
          eprintln!("Cannot read OGF texture: {}", texture.texture_name);
        }

        is_valid = false;
      }

      // todo: Shader check?
    }

    Ok(is_valid)
  }
}
