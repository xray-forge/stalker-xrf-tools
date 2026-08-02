use crate::asset::asset_type::AssetType;
use crate::project::meshes::verify_meshes_result::GamedataMeshesVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationFinding};
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
    let findings: Mutex<Vec<GamedataVerificationFinding>> = Mutex::new(Vec::new());
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
          match OgfFile::read_from_path::<XRayByteOrder, _>(&path) {
            Ok(ogf) => match self.verify_mesh_findings(options, &ogf, Some(&path)) {
              Ok(mesh_findings) if !mesh_findings.is_empty() => {
                if options.is_logging_enabled() {
                  eprintln!("Mesh is not valid: {}", path.display());
                }

                findings.lock().unwrap().extend(mesh_findings);
                *invalid_meshes_count.lock().unwrap() += 1;
              }
              Ok(_) => {}
              Err(error) => {
                if options.is_logging_enabled() {
                  eprintln!("Mesh verification failed: {} - {}", path.display(), error);
                }

                findings
                  .lock()
                  .unwrap()
                  .push(GamedataVerificationFinding::for_asset(
                    &path,
                    format!("Failed to verify mesh: {error}"),
                  ));
                *invalid_meshes_count.lock().unwrap() += 1;
              }
            },
            Err(error) => {
              if options.is_logging_enabled() {
                eprintln!("Mesh verification failed: {} - {}", path.display(), error);
              }

              findings
                .lock()
                .unwrap()
                .push(GamedataVerificationFinding::for_asset(
                  &path,
                  format!("Failed to read mesh: {error}"),
                ));
              *invalid_meshes_count.lock().unwrap() += 1;
            }
          }
        } else {
          if options.is_logging_enabled() {
            eprintln!("Mesh path not found: {}", path);
          }

          findings
            .lock()
            .unwrap()
            .push(GamedataVerificationFinding::for_asset(
              Path::new(path),
              "Mesh path was not found in gamedata roots",
            ));
          *invalid_meshes_count.lock().unwrap() += 1;
        }
      });

    let duration: u128 = started_at.elapsed().as_millis();
    let checked_meshes_count: u32 = *checked_meshes_count.lock().unwrap();
    let invalid_meshes_count: u32 = *invalid_meshes_count.lock().unwrap();
    let mut findings: Vec<GamedataVerificationFinding> =
      std::mem::take(&mut *findings.lock().unwrap());

    findings.sort_by(|left, right| {
      left
        .asset_path
        .cmp(&right.asset_path)
        .then_with(|| left.message.cmp(&right.message))
    });

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
      findings,
      invalid_meshes_count,
      checked_meshes_count,
    })
  }

  pub fn verify_mesh_by_path<P: AsRef<Path>>(
    &self,
    options: &GamedataProjectVerifyOptions,
    path: &P,
  ) -> XRayResult<bool> {
    let ogf: OgfFile = OgfFile::read_from_path::<XRayByteOrder, _>(path)?;

    Ok(
      self
        .verify_mesh_findings(options, &ogf, Some(path.as_ref()))?
        .is_empty(),
    )
  }

  pub fn verify_mesh(
    &self,
    options: &GamedataProjectVerifyOptions,
    ogf: &OgfFile,
  ) -> XRayResult<bool> {
    Ok(self.verify_mesh_findings(options, ogf, None)?.is_empty())
  }

  fn verify_mesh_findings(
    &self,
    options: &GamedataProjectVerifyOptions,
    ogf: &OgfFile,
    mesh_path: Option<&Path>,
  ) -> XRayResult<Vec<GamedataVerificationFinding>> {
    let mut findings: Vec<GamedataVerificationFinding> =
      self.verify_mesh_texture_findings(options, ogf, mesh_path);

    // Verify all nested children in mesh object.
    if let Some(children) = &ogf.children {
      for child in &children.nested {
        findings.extend(self.verify_mesh_findings(options, child, mesh_path)?);
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

          findings.push(Self::mesh_finding(
            mesh_path,
            format!("Mesh references missing motion '{motion_ref}'"),
          ));
        } else {
          for motion_path in motion_paths {
            match OmfFile::read_from_path::<XRayByteOrder, _>(&motion_path) {
              Ok(omf) => {
                match self.verify_mesh_motion_findings(options, ogf, &omf, Some(&motion_path)) {
                  Ok(motion_findings) => findings.extend(motion_findings),
                  Err(error) => {
                    if options.is_logging_enabled() {
                      eprintln!(
                        "Mesh motion verification failed: {}, error: {}",
                        motion_path.display(),
                        error
                      );
                    }

                    findings.push(GamedataVerificationFinding::for_asset(
                      &motion_path,
                      format!("Failed to verify referenced motion: {error}"),
                    ));
                  }
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

                findings.push(GamedataVerificationFinding::for_asset(
                  &motion_path,
                  format!("Failed to read referenced motion: {error}"),
                ));
              }
            }
          }
        }
      }
    }

    // todo: Verify LOD?

    Ok(findings)
  }

  pub fn verify_mesh_textures(
    &self,
    options: &GamedataProjectVerifyOptions,
    ogf: &OgfFile,
  ) -> XRayResult<bool> {
    Ok(
      self
        .verify_mesh_texture_findings(options, ogf, None)
        .is_empty(),
    )
  }

  fn verify_mesh_texture_findings(
    &self,
    options: &GamedataProjectVerifyOptions,
    ogf: &OgfFile,
    mesh_path: Option<&Path>,
  ) -> Vec<GamedataVerificationFinding> {
    let mut findings: Vec<GamedataVerificationFinding> = Vec::new();

    if let Some(texture) = &ogf.texture
      && self
        .resolve_dds_texture_path(&texture.texture_name)
        .is_none()
    {
      if options.is_logging_enabled() {
        eprintln!("Cannot read OGF texture: {}", texture.texture_name);
      }

      findings.push(Self::mesh_finding(
        mesh_path,
        format!("Mesh references missing texture '{}'", texture.texture_name),
      ));
    }

    // todo: Shader check?

    findings
  }

  pub fn verify_mesh_motion(
    &self,
    options: &GamedataProjectVerifyOptions,
    ogf: &OgfFile,
    omf: &OmfFile,
  ) -> XRayResult<bool> {
    Ok(
      self
        .verify_mesh_motion_findings(options, ogf, omf, None)?
        .is_empty(),
    )
  }

  fn verify_mesh_motion_findings(
    &self,
    options: &GamedataProjectVerifyOptions,
    ogf: &OgfFile,
    omf: &OmfFile,
    motion_path: Option<&Path>,
  ) -> XRayResult<Vec<GamedataVerificationFinding>> {
    let mut findings: Vec<GamedataVerificationFinding> = Vec::new();

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

        findings.push(Self::motion_finding(
          motion_path,
          format!(
            "Motion bone count does not match mesh: {} mesh bones, {} motion bones",
            bones.bones.len(),
            omf_bones.len()
          ),
        ));
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

        let missing_bones: Vec<&str> = bones
          .bones
          .iter()
          .filter_map(|bone| {
            (!omf_bones.contains(&bone.name.as_str())).then_some(bone.name.as_str())
          })
          .collect();

        findings.push(Self::motion_finding(
          motion_path,
          format!("Motion is missing mesh bones: {}", missing_bones.join(",")),
        ));
      }
    }

    Ok(findings)
  }

  fn mesh_finding(mesh_path: Option<&Path>, message: String) -> GamedataVerificationFinding {
    match mesh_path {
      Some(path) => GamedataVerificationFinding::for_asset(path, message),
      None => GamedataVerificationFinding::without_asset(message),
    }
  }

  fn motion_finding(motion_path: Option<&Path>, message: String) -> GamedataVerificationFinding {
    match motion_path {
      Some(path) => GamedataVerificationFinding::for_asset(path, message),
      None => GamedataVerificationFinding::without_asset(message),
    }
  }
}
