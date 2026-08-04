use crate::GamedataFindingFactory;
use crate::asset::asset_type::AssetType;
use crate::project::particles::verify_particles_usage_result::GamedataParticlesUsageVerificationResult;
use crate::{GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationRule};
use colored::Colorize;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::Instant;
use xray_db::{ParticlesFile, SpawnFile, XRayByteOrder};
use xray_error::XRayResult;
use xray_ltx::{Ltx, LtxProject};

/// Values that appear in particle-typed keys but are not particle names.
const SKIPPED_REFERENCE_VALUES: [&str; 7] = ["true", "false", "on", "off", "0", "1", "nil"];

impl GamedataProject {
  /// Verify that every particle effect/group referenced from configs and spawn custom data
  /// exists in the shipped particles.xr libraries. A reference to a missing particle is fatal
  /// at runtime (engine asserts on spawn), so it is treated as a verification failure.
  pub fn verify_particles_usage(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataParticlesUsageVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify particles usage:".green());
    }

    let started_at: Instant = Instant::now();
    let particle_names: HashSet<String> = self.read_particle_names()?;

    let mut result: GamedataParticlesUsageVerificationResult =
      GamedataParticlesUsageVerificationResult::default();

    self.verify_particles_usage_in_configs(options, &particle_names, &mut result);
    self.verify_particles_usage_in_spawns(options, &particle_names, &mut result);

    result
      .findings
      .sort_by(GamedataFindingFactory::cmp_by_asset_path_and_message);
    result.duration = started_at.elapsed();

    if options.is_logging_enabled() {
      println!(
        "Verified gamedata particles usage in {} sec, {}/{} valid references, {}/{} spawn files inspected, {} unparsed custom data sections",
        result.duration.as_secs_f64(),
        result.checked_references_count - result.invalid_references_count,
        result.checked_references_count,
        result.checked_spawn_files_count - result.unreadable_spawn_files_count,
        result.checked_spawn_files_count,
        result.unparsed_custom_data_count
      );
    }

    Ok(result)
  }

  /// Collect known particle effect and group names from all particle files in gamedata roots.
  fn read_particle_names(&self) -> XRayResult<HashSet<String>> {
    let mut names: HashSet<String> = HashSet::new();

    for path in self.get_all_asset_absolute_paths_by_ends_with("particles.xr") {
      let particles_file: ParticlesFile = ParticlesFile::read_from_path::<XRayByteOrder, _>(&path)?;

      for effect in &particles_file.effects.effects {
        names.insert(Self::normalize_particle_name(&effect.name));
      }

      for group in &particles_file.groups.groups {
        names.insert(Self::normalize_particle_name(&group.name));
      }
    }

    Ok(names)
  }

  fn verify_particles_usage_in_configs(
    &self,
    options: &GamedataProjectVerifyOptions,
    particle_names: &HashSet<String>,
    result: &mut GamedataParticlesUsageVerificationResult,
  ) {
    for path in &self.ltx_project.ltx_file_entries {
      if LtxProject::is_ltx_scheme_path(path) {
        continue;
      }

      match Ltx::read_from_file_full(path) {
        Ok(ltx) => {
          self.verify_particles_usage_in_ltx(options, particle_names, &ltx, path, result);
        }
        Err(error) => {
          // Malformed ltx files are reported by the generic ltx check, not this one.
          if options.is_verbose_logging_enabled() {
            eprintln!(
              "Skipping ltx entry in particles usage check: {} - {}",
              path.display(),
              error
            );
          }
        }
      }
    }
  }

  fn verify_particles_usage_in_spawns(
    &self,
    options: &GamedataProjectVerifyOptions,
    particle_names: &HashSet<String>,
    result: &mut GamedataParticlesUsageVerificationResult,
  ) {
    let spawn_files: Vec<String> = self
      .assets
      .iter()
      .filter(|(relative_path, descriptor)| {
        descriptor.asset_type == AssetType::Spawn && relative_path.starts_with("spawns")
      })
      .map(|(key, _)| key.clone())
      .collect::<Vec<_>>();

    for relative_path in &spawn_files {
      result.checked_spawn_files_count += 1;

      let Some(spawn_path) = self.get_absolute_asset_path(relative_path) else {
        if options.is_logging_enabled() {
          eprintln!("Spawn path not found for particle usage check: {relative_path}");
        }

        result.findings.push(GamedataFindingFactory::for_asset(
          GamedataVerificationRule::ParticlesUsageSpawn,
          Path::new(relative_path),
          "Spawn path was not found in gamedata roots",
        ));
        result.unreadable_spawn_files_count += 1;
        continue;
      };

      let spawn_file: SpawnFile =
        match SpawnFile::read_from_path::<XRayByteOrder, PathBuf>(&spawn_path) {
          Ok(spawn_file) => spawn_file,
          Err(error) => {
            if options.is_logging_enabled() {
              eprintln!(
                "Could not inspect spawn file for particle usage: {} - {}",
                spawn_path.display(),
                error
              );
            }

            result.findings.push(GamedataFindingFactory::for_asset(
              GamedataVerificationRule::ParticlesUsageSpawn,
              &spawn_path,
              format!("Could not inspect spawn file for particle usage: {error}"),
            ));
            result.unreadable_spawn_files_count += 1;
            continue;
          }
        };

      for object in &spawn_file.alife_spawn.objects {
        let Some(custom_data) = object.inherited.get_custom_data() else {
          continue;
        };

        if custom_data.trim().is_empty() {
          continue;
        }

        match Ltx::read_from_str(custom_data) {
          Ok(ltx) => {
            self.verify_particles_usage_in_ltx(options, particle_names, &ltx, &spawn_path, result);
          }
          Err(error) => {
            if options.is_logging_enabled() {
              eprintln!(
                "Could not parse spawn custom data for particle usage: {} - {}",
                spawn_path.display(),
                error
              );
            }

            result.findings.push(GamedataFindingFactory::for_asset(
              GamedataVerificationRule::ParticlesUsageSpawnCustomData,
              &spawn_path,
              format!("Could not parse spawn custom data for particle usage: {error}"),
            ));
            result.unparsed_custom_data_count += 1;
          }
        }
      }
    }
  }

  fn verify_particles_usage_in_ltx(
    &self,
    options: &GamedataProjectVerifyOptions,
    particle_names: &HashSet<String>,
    ltx: &Ltx,
    path: &Path,
    result: &mut GamedataParticlesUsageVerificationResult,
  ) {
    for (section_name, section) in &ltx.sections {
      for (key, value) in section.iter() {
        if !Self::is_particle_reference_key(section_name, key) {
          continue;
        }

        for reference in value.split(',') {
          let reference: &str = reference.trim();

          if reference.is_empty() || SKIPPED_REFERENCE_VALUES.contains(&reference) {
            continue;
          }

          result.checked_references_count += 1;

          if !particle_names.contains(&Self::normalize_particle_name(reference)) {
            if options.is_logging_enabled() {
              eprintln!(
                "Unknown particle reference: [{section_name}] {key} = {reference} ({})",
                path.display()
              );
            }

            result.findings.push(GamedataFindingFactory::for_asset(
              GamedataVerificationRule::ParticlesUsageReference,
              path,
              format!("Unknown particle reference: [{section_name}] {key} = {reference}"),
            ));
            result.invalid_references_count += 1;
          }
        }
      }
    }
  }

  /// Whether ltx key is expected to contain particle effect or group name.
  fn is_particle_reference_key(section_name: &str, key: &str) -> bool {
    if key.starts_with('$') {
      return false;
    }

    if key == "particles" || key.ends_with("_particles") {
      return true;
    }

    key == "name" && (section_name == "sr_particle" || section_name.starts_with("sr_particle@"))
  }

  fn normalize_particle_name(name: &str) -> String {
    name.trim().to_lowercase().replace('/', "\\")
  }
}
