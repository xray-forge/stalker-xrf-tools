use crate::asset::asset_type::AssetType;
use crate::project::meshes::verify_meshes_result::GamedataMeshesVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataResult};
use colored::Colorize;
use std::path::{Path, PathBuf};
use std::time::Instant;
use xray_db::{OgfFile, OmfFile, XRayByteOrder};

impl GamedataProject {
  pub fn verify_meshes(
    &mut self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataMeshesVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata meshes:".green());
    }

    let started_at: Instant = Instant::now();
    let mut checked_meshes_count: u32 = 0;
    let mut invalid_meshes_count: u32 = 0;

    for path in self.get_all_asset_paths_by_type(AssetType::Ogf) {
      if options.is_verbose_logging_enabled() {
        println!("Verify gamedata mesh: {}", path);
      }

      checked_meshes_count += 1;

      if let Some(path) = self.get_absolute_asset_path(&path) {
        match self.verify_mesh_by_path(options, &path) {
          Ok(is_valid) => {
            if !is_valid {
              if options.is_logging_enabled() {
                eprintln!("Mesh is not valid: {:?}", path);
              }

              invalid_meshes_count += 1;
            }
          }
          Err(error) => {
            if options.is_logging_enabled() {
              eprintln!("Mesh verification failed: {:?} - {error}", path);
            }

            invalid_meshes_count += 1;
          }
        }
      } else {
        if options.is_logging_enabled() {
          eprintln!("Mesh path not found: {:?}", path);
        }

        invalid_meshes_count += 1;
      }
    }

    let duration: u128 = started_at.elapsed().as_millis();

    if options.is_logging_enabled() {
      println!(
        "Verified gamedata meshes in {} sec, {}/{} valid",
        (duration as f64) / 1000.0,
        checked_meshes_count - invalid_meshes_count,
        checked_meshes_count
      );
    }

    Ok(GamedataMeshesVerificationResult {
      duration,
      invalid_meshes_count,
      checked_meshes_count,
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

    // Verify all nested children in mesh object.
    if let Some(children) = &ogf.children {
      for child in &children.nested {
        if !self.verify_mesh(options, child)? {
          is_valid = false;
        }
      }
    }

    // Verify all motion refs injected in OGF file.
    if let Some(kinematics) = &ogf.kinematics {
      for motion_ref in &kinematics.motion_refs {
        let motion_paths: Vec<PathBuf> = self.get_omf_paths_hit(motion_ref);

        if motion_paths.is_empty() {
          if options.is_logging_enabled() {
            eprintln!("Mesh motion refs not found by path: {motion_ref}");
          }

          is_valid = false;
        } else {
          for motion_path in motion_paths {
            match OmfFile::read_from_path::<XRayByteOrder, &Path>(&motion_path) {
              Ok(omf) => match self.verify_mesh_motion(options, ogf, &omf) {
                Ok(result) => {
                  if !result {
                    is_valid = false;
                  }
                }
                Err(error) => {
                  if options.is_logging_enabled() {
                    eprintln!(
                      "Mesh motion file failed to read: {:?}, error: {}",
                      motion_path, error
                    );
                  }

                  is_valid = false;
                }
              },
              Err(error) => {
                if options.is_logging_enabled() {
                  eprintln!(
                    "Mesh motion file failed to read: {:?}, error: {}",
                    motion_path, error
                  );
                }

                is_valid = false;
              }
            }
          }
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

  pub fn verify_mesh_motion(
    &mut self,
    options: &GamedataProjectVerifyOptions,
    ogf: &OgfFile,
    omf: &OmfFile,
  ) -> GamedataResult<bool> {
    let mut is_valid: bool = true;

    if let Some(bones) = &ogf.bones {
      let omf_bones: Vec<&str> = omf.get_bones();

      if bones.bones.len() != omf_bones.len() {
        if options.is_logging_enabled() {
          eprintln!(
            "Not matching bones count in ogf and reference omf: {} <-> {} : {:?} <-> {:?}",
            bones.bones.len(),
            omf_bones.len(),
            bones.bones.iter().map(|it| &it.name).collect::<Vec<_>>(),
            omf_bones
          );
        }

        is_valid = false;
      } else if bones
        .bones
        .iter()
        .any(|it| !omf_bones.contains(&it.name.as_str()))
      {
        if options.is_logging_enabled() {
          eprintln!(
            "Missing bones in OMF file for OGF mesh: {:?} <-> {:?}",
            bones.bones.iter().map(|it| &it.name).collect::<Vec<_>>(),
            omf_bones
          );
        }

        is_valid = false;
      }
    }

    Ok(is_valid)
  }
}
