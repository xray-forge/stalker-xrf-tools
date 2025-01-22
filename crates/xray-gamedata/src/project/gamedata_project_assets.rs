use crate::asset::asset_descriptor::AssetDescriptor;
use crate::error::GamedataError;
use crate::{GamedataProject, GamedataProjectReadOptions, GamedataResult};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

impl GamedataProject {
  pub fn read_project_assets(
    options: &GamedataProjectReadOptions,
    roots: &[&Path],
    ignored: &[&str],
  ) -> GamedataResult<HashMap<String, AssetDescriptor>> {
    if options.is_logging_enabled() {
      println!("Reading project assets map in roots: {:?}", roots);
    }

    let mut assets: HashMap<String, AssetDescriptor> = HashMap::new();

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
              AssetDescriptor::new_with_extension(index, relative),
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
}

impl GamedataProject {
  pub fn get_absolute_asset_path(&self, relative_path: &str) -> Option<PathBuf> {
    self.get_prefixed_absolute_asset_path("", relative_path)
  }

  pub fn get_absolute_asset_path_hit(&mut self, relative_path: &str) -> Option<PathBuf> {
    self.get_prefixed_absolute_asset_path_hit("", relative_path)
  }

  pub fn get_prefixed_absolute_asset_path(
    &self,
    prefix: &str,
    relative_path: &str,
  ) -> Option<PathBuf> {
    self
      .get_prefixed_asset(prefix, relative_path)
      .map(|(path, _)| path)
  }

  pub fn get_prefixed_absolute_asset_path_hit(
    &mut self,
    prefix: &str,
    relative_path: &str,
  ) -> Option<PathBuf> {
    self
      .get_prefixed_asset_mut(prefix, relative_path)
      .map(|(path, descriptor)| {
        descriptor.add_hit();

        path
      })
  }

  pub fn get_prefixed_asset(
    &self,
    prefix: &str,
    relative_path: &str,
  ) -> Option<(PathBuf, &AssetDescriptor)> {
    let asset_path: PathBuf =
      PathBuf::from(prefix.to_lowercase()).join(relative_path.to_lowercase());

    self
      .assets
      .get(asset_path.to_str().unwrap())
      .map(|descriptor| {
        (
          self
            .roots
            .get(descriptor.root_index)
            .expect("Correct root setup")
            .join(&asset_path),
          descriptor,
        )
      })
      .or_else(|| {
        log::warn!(
          "Trying to get not existing asset: {}",
          asset_path.to_str().unwrap()
        );

        None
      })
  }

  pub fn get_prefixed_asset_mut(
    &mut self,
    prefix: &str,
    relative_path: &str,
  ) -> Option<(PathBuf, &mut AssetDescriptor)> {
    let asset_path: PathBuf =
      PathBuf::from(prefix.to_lowercase()).join(relative_path.to_lowercase());

    self
      .assets
      .get_mut(asset_path.to_str().unwrap())
      .map(|descriptor| {
        (
          self
            .roots
            .get(descriptor.root_index)
            .expect("Correct root setup")
            .join(&asset_path),
          descriptor,
        )
      })
      .or_else(|| {
        log::warn!(
          "Trying to get not existing asset: {}",
          asset_path.to_str().unwrap()
        );

        None
      })
  }

  pub fn get_ogf_visual_path_hit(&mut self, visual_path: &str) -> Option<PathBuf> {
    self.get_mesh_path_hit(visual_path, ".ogf")
  }

  pub fn get_omf_visual_path_hit(&mut self, visual_path: &str) -> Option<PathBuf> {
    self.get_mesh_path_hit(visual_path, ".omf")
  }

  pub fn get_mesh_path_hit(&mut self, visual_path: &str, extension: &str) -> Option<PathBuf> {
    let mut visual_path: String = String::from(visual_path);

    if !visual_path.ends_with(extension) {
      visual_path.push_str(extension);
    }

    self.get_prefixed_absolute_asset_path_hit("meshes", &visual_path)
  }

  pub fn add_asset_hit_by_relative_path(&mut self, relative_path: &str) -> GamedataResult {
    match self.assets.get_mut(&relative_path.to_lowercase()) {
      Some(descriptor) => {
        descriptor.hits += 1;

        Ok(())
      }
      None => Err(GamedataError::new_check_error(format!(
        "Failed to get asset descriptor '{relative_path}' in gamedata roots info"
      ))),
    }
  }
}
