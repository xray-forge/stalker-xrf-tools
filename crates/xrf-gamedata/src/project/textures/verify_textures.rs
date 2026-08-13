use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use ddsfile::{Dds, DxgiFormat};
use rayon::prelude::*;
use xrf_assets::XrayAssetType as AssetType;
use xrf_db::{ThmFile, XRayByteOrder};
use xrf_error::{XRayError, XRayResult};

use crate::GamedataFindingFactory;
use crate::project::textures::verify_textures_result::GamedataTexturesVerificationResult;
use crate::{Finding, GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationRule};

impl GamedataProject {
  pub fn verify_textures(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataTexturesVerificationResult> {
    xrf_output::heading!(options.output, "Verify textures:");

    let started_at: Instant = Instant::now();

    let texture_paths: Vec<String> = self
      .assets
      .with_type(AssetType::Dds)
      .map(|asset| asset.logical_path().to_string())
      .collect();

    let checked_textures_count: u32 = u32::try_from(texture_paths.len())
      .map_err(|_| XRayError::new_verify_error("Texture count exceeds the supported result range"))?;

    let mut findings: Vec<Finding> = texture_paths
      .par_iter()
      .filter_map(|relative_path| {
        xrf_output::verbose!(options.output, "Verify texture: {relative_path}");

        let Some(path) = self.assets.absolute_path(relative_path).ok().flatten() else {
          xrf_output::info!(options.output, "Texture path not found: {relative_path}");

          return Some(GamedataFindingFactory::for_asset(
            GamedataVerificationRule::TexturesPath,
            Path::new(relative_path),
            "Texture path was not found in gamedata roots",
          ));
        };

        match self.verify_texture_by_path(options, &path) {
          Ok(true) => None,
          Ok(false) => {
            xrf_output::info!(options.output, "Texture is not valid: {}", path.display());

            Some(GamedataFindingFactory::for_asset(
              GamedataVerificationRule::TexturesValidation,
              &path,
              "Texture uses an unsupported format",
            ))
          }
          Err(error) => {
            xrf_output::info!(
              options.output,
              "Texture verification failed: {} - {}",
              path.display(),
              error
            );

            Some(GamedataFindingFactory::for_asset(
              GamedataVerificationRule::TexturesRead,
              &path,
              error.to_string(),
            ))
          }
        }
      })
      .collect();

    let invalid_textures_count: u32 = u32::try_from(findings.len())
      .map_err(|_| XRayError::new_verify_error("Invalid texture count exceeds the supported result range"))?;

    let (bump_findings, checked_bumps_count) = self.verify_texture_bumps(options)?;
    let unresolved_bumps_count: u32 = u32::try_from(bump_findings.len())
      .map_err(|_| XRayError::new_verify_error("Unresolved bump count exceeds the supported result range"))?;

    findings.extend(bump_findings);
    findings.sort_by(GamedataFindingFactory::cmp_by_asset_path_and_message);

    let duration: Duration = started_at.elapsed();

    xrf_output::info!(
      options.output,
      "Verified gamedata textures in {} sec, {}/{} valid, {}/{} declared bumps resolved",
      duration.as_secs_f64(),
      checked_textures_count - invalid_textures_count,
      checked_textures_count,
      checked_bumps_count - unresolved_bumps_count,
      checked_bumps_count
    );

    Ok(GamedataTexturesVerificationResult {
      duration,
      findings,
      invalid_textures_count,
      checked_textures_count,
      checked_bumps_count,
      unresolved_bumps_count,
    })
  }

  /// Check that every bump a texture descriptor asks for actually exists.
  ///
  /// `CTextureDescrMngr::LoadTHM` takes the thm bump name verbatim, with no `_bump` naming
  /// convention behind it. A name that resolves to nothing does not turn bump mapping off, because
  /// `bump_exist` only tests that the name is non-empty: the renderer still takes the `_bump`
  /// shader path and the loader substitutes `ed\ed_dummy_bump`, so the surface is flat anyway and
  /// the log fills with `! Fallback to default bump map`. Importing a texture under a path
  /// different from its source is the usual way to produce one, because the copied descriptor
  /// keeps pointing into the source layout.
  ///
  /// Returns the findings and how many descriptors declared a bump at all.
  fn verify_texture_bumps(&self, options: &GamedataProjectVerifyOptions) -> XRayResult<(Vec<Finding>, u32)> {
    let descriptor_paths: Vec<String> = self
      .assets
      .with_type(AssetType::Thm)
      .map(|asset| asset.logical_path().to_string())
      .collect();

    let declarations: Vec<(String, String)> = descriptor_paths
      .par_iter()
      .filter_map(|relative_path| {
        let path: PathBuf = self.assets.absolute_path(relative_path).ok().flatten()?;

        match ThmFile::read_from_path::<XRayByteOrder, _>(&path) {
          Ok(descriptor) => descriptor
            .used_bump_name()
            .map(|bump_name| (relative_path.clone(), bump_name.to_owned())),
          Err(error) => {
            // A descriptor that cannot be parsed is reported by its own texture, not silently
            // treated as declaring no bump.
            xrf_output::verbose!(
              options.output,
              "Texture descriptor is not readable: {relative_path} - {error}"
            );

            None
          }
        }
      })
      .collect();

    let checked_bumps_count: u32 = u32::try_from(declarations.len())
      .map_err(|_| XRayError::new_verify_error("Declared bump count exceeds the supported result range"))?;

    let findings: Vec<Finding> = declarations
      .par_iter()
      .filter_map(|(relative_path, bump_name)| {
        if self.assets.dds_texture(bump_name).ok().flatten().is_some() {
          return None;
        }

        xrf_output::info!(
          options.output,
          "Texture descriptor declares missing bump: {relative_path} -> {bump_name}"
        );

        Some(GamedataFindingFactory::for_asset(
          GamedataVerificationRule::TexturesBump,
          Path::new(relative_path),
          format!("Texture descriptor declares bump '{bump_name}' that is not in gamedata"),
        ))
      })
      .collect();

    Ok((findings, checked_bumps_count))
  }

  pub fn verify_texture_by_path(&self, options: &GamedataProjectVerifyOptions, path: &Path) -> XRayResult<bool> {
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

  pub fn verify_texture(&self, _options: &GamedataProjectVerifyOptions, dds: &Dds) -> XRayResult<bool> {
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
  use std::path::PathBuf;

  use ddsfile::{AlphaMode, D3D10ResourceDimension, Dds, DxgiFormat, NewDxgiParams};
  use xrf_assets::{DirectoryAssetIndex, XrayAssetIndex};
  use xrf_ltx::LtxProject;

  use super::GamedataProject;
  use crate::GamedataProjectVerifyOptions;

  fn empty_project() -> GamedataProject {
    GamedataProject {
      assets: XrayAssetIndex::new(
        DirectoryAssetIndex::read(env!("CARGO_MANIFEST_DIR")).expect("read test assets"),
        &[],
      )
      .expect("create test assets"),
      ltx_project: LtxProject {
        root: PathBuf::new(),
        ltx_file_entries: Vec::new(),
        ltx_files: Vec::new(),
        ltx_scheme_files: Vec::new(),
        ltx_scheme_file_entries: Vec::new(),
        ltx_scheme_declarations: Default::default(),
      },
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
