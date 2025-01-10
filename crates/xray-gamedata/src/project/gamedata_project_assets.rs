use crate::error::GamedataError;
use crate::{GamedataProject, GamedataResult};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

impl GamedataProject {
  pub fn read_project_assets(roots: &[&Path]) -> GamedataResult<HashSet<PathBuf>> {
    log::info!("Reading project assets in roots: {:?}", roots);

    let mut assets: HashSet<PathBuf> = HashSet::new();

    for root in roots {
      for entry in WalkDir::new(root) {
        let entry: DirEntry =
          entry.map_err(|error| GamedataError::Io(error.into_io_error().unwrap()))?;
        let entry_path: &Path = entry.path();

        // let relative = entry_path.strip_prefix(root).map_err(|error| error)?;

        assets.insert(entry_path.to_path_buf());
      }
    }

    log::info!("Read project assets: {} files", assets.len());

    Ok(assets)
  }

  pub fn get_relative_asset_path(&self, relative_path: &str) -> Option<PathBuf> {
    self.get_prefixed_relative_asset_path("", relative_path)
  }

  pub fn get_prefixed_relative_asset_path(
    &self,
    prefix: &str,
    relative_path: &str,
  ) -> Option<PathBuf> {
    for root in &self.roots {
      let relative_from_root: PathBuf = root.join(prefix).join(relative_path);

      if relative_from_root.exists() && relative_from_root.is_file() {
        return Some(relative_from_root);
      }
    }

    None
  }
}
