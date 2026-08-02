use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use xray_ltx::Ltx;

/// Definitions that weather sections may reference.
///
/// Read failures are retained for each definition family. This lets the weather validator mark
/// affected cycles invalid instead of treating an unavailable definition file as an empty set.
pub struct WeatherDefinitions {
  /// Ambient section names read from `environment/ambients.ltx` and its includes.
  pub ambient_sections: Result<HashSet<String>, String>,
  /// Sun section names read from `environment/suns.ltx` and the engine's `system.ltx` fallback.
  pub sun_sections: Result<HashSet<String>, String>,
  /// Thunderbolt collection names mapped to member definitions that could not be resolved.
  ///
  /// An empty member list means that every member of the collection has a definition.
  pub thunderbolt_collections: Result<HashMap<String, Vec<String>>, String>,
}

impl WeatherDefinitions {
  /// Reads weather definitions from an assembled `configs` directory.
  ///
  /// The returned value retains definition-family errors so validation can continue and report
  /// every affected weather cycle in one run.
  pub fn read(configs_root: &Path) -> Self {
    let system_sections: Result<HashSet<String>, String> =
      Self::read_sections(&configs_root.join("system.ltx"));

    Self {
      ambient_sections: Self::read_sections(&configs_root.join("environment").join("ambients.ltx")),
      // The engine falls back to system.ltx for legacy sun definitions.
      sun_sections: Self::read_sections(&configs_root.join("environment").join("suns.ltx"))
        .and_then(|mut sections| {
          sections.extend(
            system_sections
              .as_ref()
              .map_err(Clone::clone)?
              .iter()
              .cloned(),
          );
          Ok(sections)
        }),
      thunderbolt_collections: Self::read_thunderbolt_collections(configs_root),
    }
  }

  fn read_sections(path: &Path) -> Result<HashSet<String>, String> {
    Self::read_ltx(path).map(|ltx| {
      ltx
        .iter()
        .map(|(section_name, _)| section_name.to_string())
        .filter(|section_name| !section_name.is_empty())
        .collect()
    })
  }

  fn read_ltx(path: &Path) -> Result<Ltx, String> {
    Ltx::read_from_file_full(path).map_err(|error| {
      format!(
        "Could not read weather definitions from {}: {error}",
        path.display()
      )
    })
  }

  fn read_thunderbolt_collections(
    configs_root: &Path,
  ) -> Result<HashMap<String, Vec<String>>, String> {
    let environment_root: PathBuf = configs_root.join("environment");
    let collections: Ltx = Self::read_ltx(&environment_root.join("thunderbolt_collections.ltx"))?;
    let thunderbolts: Ltx = Self::read_ltx(&environment_root.join("thunderbolts.ltx"))?;
    let system: Ltx = Self::read_ltx(&configs_root.join("system.ltx"))?;

    let mut result: HashMap<String, Vec<String>> = HashMap::new();

    for (collection_name, collection) in &collections {
      if !collection_name.is_empty() {
        result.insert(
          collection_name.to_string(),
          collection
            .iter()
            .map(|(thunderbolt_name, _)| thunderbolt_name.to_string())
            .filter(|thunderbolt_name| !thunderbolts.has_section(thunderbolt_name))
            .collect(),
        );
      }
    }

    // The engine also permits legacy thunderbolt definitions in system.ltx.
    for (collection_name, collection) in &system {
      if !collection_name.is_empty() {
        result
          .entry(collection_name.to_string())
          .or_insert_with(|| {
            collection
              .iter()
              .map(|(thunderbolt_name, _)| thunderbolt_name.to_string())
              .filter(|thunderbolt_name| !system.has_section(thunderbolt_name))
              .collect()
          });
      }
    }

    Ok(result)
  }
}
