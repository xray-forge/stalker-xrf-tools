use crate::project::particles::verify_particles_result::GamedataParticlesVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationFinding};
use colored::Colorize;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Instant;
use xray_db::{ParticlesFile, XRayByteOrder};
use xray_error::XRayResult;

impl GamedataProject {
  pub fn verify_particles(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataParticlesVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify particles:".green());
    }

    let started_at: Instant = Instant::now();
    let checked_particles_count: Mutex<u32> = Mutex::new(0);
    let findings: Mutex<Vec<GamedataVerificationFinding>> = Mutex::new(Vec::new());
    let invalid_particles_count: Mutex<u32> = Mutex::new(0);

    self
      .get_all_asset_absolute_paths_by_ends_with("particles.xr")
      .par_iter()
      .for_each(|path| {
        if options.is_verbose_logging_enabled() {
          println!("Verify particles file: {}", path.display());
        }

        *checked_particles_count.lock().unwrap() += 1;

        match ParticlesFile::read_from_path::<XRayByteOrder, &PathBuf>(&path) {
          Ok(particles_file) => {
            let particle_findings: Vec<GamedataVerificationFinding> =
              self.verify_particle(options, &particles_file, path);

            if !particle_findings.is_empty() {
              if options.is_logging_enabled() {
                println!("Particle library is invalid: {}", path.display());
              }

              findings.lock().unwrap().extend(particle_findings);
              *invalid_particles_count.lock().unwrap() += 1;
            }
          }
          Err(error) => {
            if options.is_logging_enabled() {
              println!(
                "Failed to read particle library '{}': {}",
                path.display(),
                error
              );
            }

            findings
              .lock()
              .unwrap()
              .push(GamedataVerificationFinding::for_asset(
                path,
                format!("Failed to read particle library: {error}"),
              ));
            *invalid_particles_count.lock().unwrap() += 1;
          }
        }
      });

    let duration: u128 = started_at.elapsed().as_millis();
    let checked_particle_files_count: u32 = *checked_particles_count.lock().unwrap();
    let invalid_particle_files_count: u32 = *invalid_particles_count.lock().unwrap();
    let mut findings: Vec<GamedataVerificationFinding> =
      std::mem::take(&mut *findings.lock().unwrap());

    findings.sort_by(|left, right| left.asset_path.cmp(&right.asset_path));

    if options.is_logging_enabled() {
      println!(
        "Verified gamedata particle files in {} sec, {}/{} valid",
        (duration as f64) / 1000.0,
        checked_particle_files_count - invalid_particle_files_count,
        checked_particle_files_count
      );
    }

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
  ) -> Vec<GamedataVerificationFinding> {
    let mut findings: Vec<GamedataVerificationFinding> = Vec::new();

    for particle in &particles_file.effects.effects {
      if options.is_verbose_logging_enabled() {
        println!("Verify particle: {}", particle.name);
      }

      for texture_relative_path in particle.sprite.texture_name.split(",") {
        if let Some(texture) = self.resolve_dds_texture_path(texture_relative_path) {
          match self.verify_texture_by_path(options, &texture) {
            Ok(result) => {
              if !result {
                findings.push(GamedataVerificationFinding::for_asset(
                  &texture,
                  format!(
                    "Particle effect '{}' references an invalid texture",
                    particle.name
                  ),
                ));
              }
            }
            Err(error) => {
              findings.push(GamedataVerificationFinding::for_asset(
                &texture,
                format!(
                  "Failed to verify texture for particle effect '{}': {error}",
                  particle.name
                ),
              ));
            }
          }
        } else {
          findings.push(GamedataVerificationFinding::for_asset(
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
