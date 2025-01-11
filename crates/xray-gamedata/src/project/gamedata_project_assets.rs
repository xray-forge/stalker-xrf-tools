use crate::error::GamedataError;
use crate::project::gamedata_asset_descriptor::GamedataAssetDescriptor;
use crate::{GamedataProject, GamedataProjectOpenOptions, GamedataResult};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

impl GamedataProject {
  pub fn read_project_assets(
    options: &GamedataProjectOpenOptions,
    roots: &[&Path],
    ignored: &[&str],
  ) -> GamedataResult<HashMap<String, GamedataAssetDescriptor>> {
    log::info!("Reading project assets map in roots: {:?}", roots);

    if options.is_logging_enabled() {
      println!("Reading project assets map in roots: {:?}", roots);
    }

    let mut assets: HashMap<String, GamedataAssetDescriptor> = HashMap::new();

    for (index, root) in roots.iter().enumerate() {
      for entry in WalkDir::new(root) {
        let entry: DirEntry =
          entry.map_err(|error| GamedataError::Io(error.into_io_error().unwrap()))?;
        let entry_path: &Path = entry.path();

        // Dirs are skipped.
        if entry_path.is_dir() {
          continue;
        }

        if let Some(relative) = entry_path.strip_prefix(root)?.to_str() {
          if ignored.iter().any(|it| relative.starts_with(it)) {
            continue;
          }

          if !assets.contains_key(relative) {
            assets.insert(
              relative.to_lowercase(),
              GamedataAssetDescriptor::new_with_extension(index, relative),
            );
          }
        } else {
          log::warn!("Could not strip prefix: {:?}", entry_path);
        }
      }
    }

    if options.is_logging_enabled() {
      println!("Read project assets map: {} files", assets.len());
    }

    Ok(assets)
  }

  pub fn get_relative_asset_path(&mut self, relative_path: &str) -> Option<PathBuf> {
    self.get_prefixed_relative_asset_path("", relative_path)
  }

  pub fn get_prefixed_relative_asset_path(
    &mut self,
    prefix: &str,
    relative_path: &str,
  ) -> Option<PathBuf> {
    let asset_path: PathBuf =
      PathBuf::from(prefix.to_lowercase()).join(relative_path.to_lowercase());

    if let Some(descriptor) = self.assets.get_mut(asset_path.to_str().unwrap()) {
      descriptor.add_hit();

      return Some(
        self
          .roots
          .get(descriptor.root_index)
          .expect("Correct root setup")
          .join(asset_path),
      );
    } else {
      log::warn!(
        "Trying to get not existing asset path: {}",
        asset_path.to_str().unwrap()
      );
    }

    None
  }

  pub fn add_asset_hit_by_relative_path(&mut self, relative_path: &str) -> GamedataResult {
    if let Some(descriptor) = self.assets.get_mut(&relative_path.to_lowercase()) {
      descriptor.hits += 1;
      Ok(())
    } else {
      Err(GamedataError::new_check_error(format!(
        "Failed to get asset descriptor '{relative_path}' in gamedata roots info"
      )))
    }
  }
}
