use crate::GamedataProject;
use std::path::PathBuf;

impl GamedataProject {
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
