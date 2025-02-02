use crate::asset::asset_type::AssetType;
use crate::project::meshes::verify_meshes_result::GamedataMeshesVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions};
use colored::Colorize;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;
use xray_db::{OgfFile, OmfFile, XRayByteOrder};
use xray_error::XRayResult;

impl GamedataProject {
  pub fn verify_meshes(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataMeshesVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify meshes:".green());
    }

    let started_at: Instant = Instant::now();
    let checked_meshes_count: Mutex<u32> = Mutex::new(0);
    let invalid_meshes_count: Mutex<u32> = Mutex::new(0);

    self
      .get_all_asset_paths_by_type(AssetType::Ogf)
      .par_iter()
      .for_each(|path| {
        if options.is_verbose_logging_enabled() {
          println!("Verify mesh: {}", path);
        }

        *checked_meshes_count.lock().unwrap() += 1;

        if let Some(path) = self.get_absolute_asset_path(path) {
          match self.verify_mesh_by_path(options, &path) {
            Ok(is_valid) => {
              if !is_valid {
                if options.is_logging_enabled() {
                  eprintln!("Mesh is not valid: {}", path.display());
                }

                *invalid_meshes_count.lock().unwrap() += 1;
              }
            }
            Err(error) => {
              if options.is_logging_enabled() {
                eprintln!("Mesh verification failed: {} - {}", path.display(), error);
              }

              *invalid_meshes_count.lock().unwrap() += 1;
            }
          }
        } else {
          if options.is_logging_enabled() {
            eprintln!("Mesh path not found: {}", path);
          }

          *invalid_meshes_count.lock().unwrap() += 1;
        }
      });

    let duration: u128 = started_at.elapsed().as_millis();
    let checked_meshes_count: u32 = *checked_meshes_count.lock().unwrap();
    let invalid_meshes_count: u32 = *invalid_meshes_count.lock().unwrap();

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

  pub fn verify_mesh_by_path<P: AsRef<Path>>(
    &self,
    options: &GamedataProjectVerifyOptions,
    path: &P,
  ) -> XRayResult<bool> {
    self.verify_mesh(options, &OgfFile::read_from_path::<XRayByteOrder, _>(path)?)
  }

  pub fn verify_mesh(
    &self,
    options: &GamedataProjectVerifyOptions,
    ogf: &OgfFile,
  ) -> XRayResult<bool> {
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
        let motion_paths: Vec<PathBuf> = self.get_omf_paths(motion_ref);

        if motion_paths.is_empty() {
          if options.is_logging_enabled() {
            eprintln!("Mesh motion refs not found by path: {motion_ref}");
          }

          is_valid = false;
        } else {
          for motion_path in motion_paths {
            match OmfFile::read_from_path::<XRayByteOrder, _>(&motion_path) {
              Ok(omf) => match self.verify_mesh_motion(options, ogf, &omf) {
                Ok(result) => {
                  if !result {
                    is_valid = false;
                  }
                }
                Err(error) => {
                  if options.is_logging_enabled() {
                    eprintln!(
                      "Mesh motion file failed to read: {}, error: {}",
                      motion_path.display(),
                      error
                    );
                  }

                  is_valid = false;
                }
              },
              Err(error) => {
                if options.is_logging_enabled() {
                  eprintln!(
                    "Mesh motion file failed to read: {}, error: {}",
                    motion_path.display(),
                    error
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
    &self,
    options: &GamedataProjectVerifyOptions,
    ogf: &OgfFile,
  ) -> XRayResult<bool> {
    let mut is_valid: bool = true;

    if let Some(texture) = &ogf.texture {
      if self.get_dds_path(&texture.texture_name).is_none() {
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
    &self,
    options: &GamedataProjectVerifyOptions,
    ogf: &OgfFile,
    omf: &OmfFile,
  ) -> XRayResult<bool> {
    let mut is_valid: bool = true;

    if let Some(bones) = &ogf.bones {
      let omf_bones: Vec<&str> = omf.get_bones();

      if bones.bones.len() != omf_bones.len() {
        if options.is_logging_enabled() {
          eprintln!(
            "Not matching bones count in ogf and reference omf: {} <-> {} : {} <-> {}",
            bones.bones.len(),
            omf_bones.len(),
            bones
              .bones
              .iter()
              .map(|it| it.name.as_str())
              .collect::<Vec<_>>()
              .join(","),
            omf_bones.join(",")
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
            "Missing bones in OMF file for OGF mesh: {} <-> {}",
            bones
              .bones
              .iter()
              .map(|it| it.name.as_str())
              .collect::<Vec<_>>()
              .join(","),
            omf_bones.join(",")
          );
        }

        is_valid = false;
      }
    }

    Ok(is_valid)
  }
}
