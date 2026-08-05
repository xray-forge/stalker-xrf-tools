use crate::GamedataFindingFactory;
use crate::project::particles::verify_particles_result::GamedataParticlesVerificationResult;
use crate::{Finding, GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationRule};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use xray_db::{ParticlesFile, XRayByteOrder};
use xray_error::{XRayError, XRayResult};

impl GamedataProject {
  pub fn verify_particles(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataParticlesVerificationResult> {
    xray_output::heading!(options.output, "Verify particles:");

    let started_at: Instant = Instant::now();
    let particle_paths: Vec<PathBuf> = self
      .assets
      .with_suffix("particles.xr")?
      .map(|asset| asset.absolute_path())
      .collect();

    let checked_particle_files_count: u32 = u32::try_from(particle_paths.len()).map_err(|_| {
      XRayError::new_verify_error("Particle library count exceeds the supported result range")
    })?;

    let particle_findings: Vec<Vec<Finding>> = particle_paths
      .par_iter()
      .map(|path| {
        xray_output::verbose!(options.output, "Verify particles file: {}", path.display());

        match ParticlesFile::read_from_path::<XRayByteOrder, &PathBuf>(&path) {
          Ok(particles_file) => {
            let particle_findings: Vec<Finding> =
              self.verify_particle(options, &particles_file, path);

            if !particle_findings.is_empty() {
              xray_output::info!(
                options.output,
                "Particle library is invalid: {}",
                path.display()
              );
            }

            particle_findings
          }
          Err(error) => {
            xray_output::info!(
              options.output,
              "Failed to read particle library '{}': {}",
              path.display(),
              error
            );

            vec![GamedataFindingFactory::for_asset(
              GamedataVerificationRule::ParticlesLibrary,
              path,
              format!("Failed to read particle library: {error}"),
            )]
          }
        }
      })
      .collect();

    let duration: Duration = started_at.elapsed();
    let invalid_particle_files_count: u32 = u32::try_from(
      particle_findings
        .iter()
        .filter(|findings| !findings.is_empty())
        .count(),
    )
    .map_err(|_| {
      XRayError::new_verify_error(
        "Invalid particle library count exceeds the supported result range",
      )
    })?;

    let mut findings: Vec<Finding> = particle_findings.into_iter().flatten().collect();

    findings.sort_by(GamedataFindingFactory::cmp_by_asset_path_and_message);

    xray_output::info!(
      options.output,
      "Verified gamedata particle files in {} sec, {}/{} valid",
      duration.as_secs_f64(),
      checked_particle_files_count - invalid_particle_files_count,
      checked_particle_files_count
    );

    Ok(GamedataParticlesVerificationResult {
      duration,
      checked_particle_files_count,
      findings,
      invalid_particle_files_count,
    })
  }

  pub fn verify_particle(
    &self,
    options: &GamedataProjectVerifyOptions,
    particles_file: &ParticlesFile,
    particle_library_path: &Path,
  ) -> Vec<Finding> {
    let mut findings: Vec<Finding> = Vec::new();

    for particle in &particles_file.effects.effects {
      xray_output::verbose!(options.output, "Verify particle: {}", particle.name);

      for texture_relative_path in particle.sprite.texture_name.split(",") {
        if let Some(texture) = self
          .assets
          .dds_texture(texture_relative_path)
          .ok()
          .flatten()
          .map(|asset| asset.absolute_path())
        {
          match self.verify_texture_by_path(options, &texture) {
            Ok(result) => {
              if !result {
                findings.push(GamedataFindingFactory::for_asset(
                  GamedataVerificationRule::ParticlesTexture,
                  &texture,
                  format!(
                    "Particle effect '{}' references an invalid texture",
                    particle.name
                  ),
                ));
              }
            }
            Err(error) => {
              findings.push(GamedataFindingFactory::for_asset(
                GamedataVerificationRule::ParticlesTexture,
                &texture,
                format!(
                  "Failed to verify texture for particle effect '{}': {error}",
                  particle.name
                ),
              ));
            }
          }
        } else {
          findings.push(GamedataFindingFactory::for_asset(
            GamedataVerificationRule::ParticlesTexture,
            particle_library_path,
            format!(
              "Particle effect '{}' references missing texture '{}'",
              particle.name, texture_relative_path
            ),
          ));
        }
      }
    }

    findings
  }
}
