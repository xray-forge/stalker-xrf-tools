use crate::project::particles::verify_particles_result::GamedataParticlesVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions};
use colored::Colorize;
use rayon::prelude::*;
use std::path::PathBuf;
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
          Ok(particles_file) => match self.verify_particle(options, &particles_file) {
            Ok(result) => {
              if !result {
                if options.is_logging_enabled() {
                  eprintln!("Particle file is not valid: {}", path.display());
                }

                *invalid_particles_count.lock().unwrap() += 1;
              }
            }
            Err(error) => {
              if options.is_logging_enabled() {
                println!(
                  "Particles file verification failed: {} - {}",
                  path.display(),
                  error
                );
              }

              *invalid_particles_count.lock().unwrap() += 1;
            }
          },
          Err(error) => {
            if options.is_logging_enabled() {
              eprintln!(
                "Particles verification failed: {} - {}",
                path.display(),
                error
              );
            }

            *invalid_particles_count.lock().unwrap() += 1;
          }
        }
      });

    let duration: u128 = started_at.elapsed().as_millis();
    let checked_particle_files_count: u32 = *checked_particles_count.lock().unwrap();
    let invalid_particle_files_count: u32 = *invalid_particles_count.lock().unwrap();

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
      invalid_particle_files_count,
    })
  }

  pub fn verify_particle(
    &self,
    options: &GamedataProjectVerifyOptions,
    particles_file: &ParticlesFile,
  ) -> XRayResult<bool> {
    let mut is_valid: bool = true;

    for particle in &particles_file.effects.effects {
      if options.is_verbose_logging_enabled() {
        println!("Verify particle: {}", particle.name);
      }

      for texture_relative_path in particle.sprite.texture_name.split(",") {
        if let Some(texture) = self.get_dds_path(texture_relative_path) {
          match self.verify_texture_by_path(options, &texture) {
            Ok(result) => {
              if !result {
                if options.is_logging_enabled() {
                  println!(
                    "Particle texture is not valid: {} - {}",
                    particle.name,
                    texture.display()
                  );
                }

                is_valid = false;
              }
            }
            Err(error) => {
              if options.is_logging_enabled() {
                println!(
                  "Particle texture verification failed: {} - {} - {}",
                  particle.name,
                  texture.display(),
                  error
                );
              }

              is_valid = false;
            }
          }
        } else {
          // Just log message.
          if options.is_logging_enabled() {
            eprintln!(
              "Not found texture for particle: {} - {}",
              particle.name, texture_relative_path
            );

            is_valid = false;
          }
        }
      }
    }

    Ok(is_valid)
  }
}
