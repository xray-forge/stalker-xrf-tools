use std::collections::BTreeSet;
use std::path::PathBuf;

use xrf_error::XRayResult;
use xrf_ltx::Ltx;

use crate::GamedataFindingFactory;
use crate::project::levels::level_bundle::LevelBundle;
use crate::project::levels::level_engine_constants::{
  LEVELS_DIRECTORY, MULTIPLAYER_MAPS_FILE, MULTIPLAYER_MAPS_SECTION, SINGLE_PLAYER_MAPS_FILE,
  SINGLE_PLAYER_MAPS_SECTION,
};
use crate::project::levels::level_roster::LevelRoster;
use crate::{Finding, GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationRule};

/// Reconciles the game graph roster, the bundles on disk, and level map declarations.
pub(crate) struct LevelReconciliationVerifier<'a> {
  options: &'a GamedataProjectVerifyOptions,
  project: &'a GamedataProject,
}

impl<'a> LevelReconciliationVerifier<'a> {
  pub(crate) fn new(project: &'a GamedataProject, options: &'a GamedataProjectVerifyOptions) -> Self {
    Self { options, project }
  }

  /// Collect level bundle directory names from indexed assets.
  ///
  /// Files stored directly under the levels root, such as `root.ltx`, are not bundles.
  pub(crate) fn bundle_names(&self) -> XRayResult<BTreeSet<String>> {
    let prefix: String = format!("{LEVELS_DIRECTORY}\\");

    Ok(
      self
        .project
        .assets
        .with_prefix(LEVELS_DIRECTORY)?
        .filter_map(|asset| {
          asset
            .logical_path()
            .strip_prefix(&prefix)
            .and_then(|rest| rest.split_once('\\'))
            .map(|(directory, _)| directory.to_string())
        })
        .collect(),
    )
  }

  pub(crate) fn verify(&self, roster: &LevelRoster, bundles: &BTreeSet<String>) -> XRayResult<Vec<Finding>> {
    let mut findings: Vec<Finding> = Vec::new();
    let declared_maps: BTreeSet<String> = self.declared_map_levels()?;

    for level in &roster.levels {
      if !bundles.contains(&level.name) {
        findings.push(GamedataFindingFactory::for_asset(
          GamedataVerificationRule::LevelsMissingBundle,
          LevelBundle::path_of(&level.name),
          format!(
            "Game graph declares level [{}] with id {}, but no level bundle exists for it",
            level.name, level.id
          ),
        ));
      }

      if !declared_maps.contains(&level.name) {
        findings.push(GamedataFindingFactory::for_asset(
          GamedataVerificationRule::LevelsUndeclaredMap,
          LevelBundle::path_of(&level.name),
          format!(
            "Game graph declares level [{}], but no level map is declared for it in [{SINGLE_PLAYER_MAPS_SECTION}] or [{MULTIPLAYER_MAPS_SECTION}]",
            level.name
          ),
        ));
      }
    }

    let roster_names: BTreeSet<&str> = roster.names();

    for bundle in bundles {
      if !roster_names.contains(bundle.as_str()) {
        findings.push(GamedataFindingFactory::for_asset(
          GamedataVerificationRule::LevelsOrphanBundle,
          LevelBundle::path_of(bundle),
          format!("Level bundle [{bundle}] is not reachable from any game graph"),
        ));
      }
    }

    Ok(findings)
  }

  /// Levels declared in single player and multiplayer map configurations.
  ///
  /// A declaration without a bundle is legal and is deliberately not reported: shipped
  /// configurations declare maps for levels that were never built.
  fn declared_map_levels(&self) -> XRayResult<BTreeSet<String>> {
    let mut declared: BTreeSet<String> = BTreeSet::new();

    for (file, section_name) in [
      (SINGLE_PLAYER_MAPS_FILE, SINGLE_PLAYER_MAPS_SECTION),
      (MULTIPLAYER_MAPS_FILE, MULTIPLAYER_MAPS_SECTION),
    ] {
      for asset in self.project.assets.with_suffix(file)? {
        let path: PathBuf = asset.absolute_path();

        // Malformed configurations are reported by the ltx check, not this one.
        let Ok(ltx) = Ltx::read_from_file_full(&path) else {
          xrf_output::verbose!(
            self.options.output,
            "Skipping unreadable level maps configuration: {}",
            path.display()
          );

          continue;
        };

        if let Some(section) = ltx.section(section_name) {
          declared.extend(section.iter().map(|(key, _)| key.to_lowercase()));
        }
      }
    }

    Ok(declared)
  }
}
