use std::path::PathBuf;
use xray_assets::XrayAsset;

use crate::GamedataProject;

impl GamedataProject {
  pub fn get_all_asset_absolute_paths_by_ends_with(&self, filter: &str) -> Vec<PathBuf> {
    self
      .assets
      .with_suffix(filter)
      .into_iter()
      .flatten()
      .map(|asset| asset.absolute_path())
      .collect()
  }

  pub fn get_absolute_asset_path(&self, relative_path: &str) -> Option<PathBuf> {
    self.get_prefixed_absolute_asset_path("", relative_path)
  }

  pub fn get_shader_library_path(&self) -> PathBuf {
    self
      .assets
      .expected_absolute_path(xray_assets::shader::SHADER_LIBRARY_LOGICAL_PATH)
      .expect("fixed shader library path is valid")
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

  pub fn get_prefixed_asset(
    &self,
    prefix: &str,
    relative_path: &str,
  ) -> Option<(PathBuf, XrayAsset<'_>)> {
    self
      .assets
      .find_in(prefix, relative_path)
      .ok()
      .flatten()
      .map(|asset| (asset.absolute_path(), asset))
  }

  pub fn get_prefixed_masked_assets(
    &self,
    prefix: &str,
    mask: &str,
  ) -> Vec<(PathBuf, XrayAsset<'_>)> {
    let Ok(assets) = self.assets.with_mask_in(prefix, mask) else {
      return Vec::new();
    };
    assets.map(|asset| (asset.absolute_path(), asset)).collect()
  }

  pub fn get_ogf_path(&self, visual_path: &str) -> Option<PathBuf> {
    self
      .assets
      .ogf(visual_path)
      .ok()
      .flatten()
      .map(|asset| asset.absolute_path())
  }

  pub fn get_omf_path(&self, visual_path: &str) -> Option<PathBuf> {
    self
      .assets
      .omf(visual_path)
      .ok()
      .flatten()
      .map(|asset| asset.absolute_path())
  }

  pub fn get_omf_paths(&self, visual_path: &str) -> Vec<PathBuf> {
    self
      .assets
      .omfs(visual_path)
      .into_iter()
      .flatten()
      .map(|asset| asset.absolute_path())
      .collect()
  }

  pub fn get_mesh_path(&self, visual_path: &str, extension: &str) -> Option<PathBuf> {
    self
      .assets
      .find_in(
        "meshes",
        &xray_assets::xray_path::with_extension(visual_path, extension),
      )
      .ok()
      .flatten()
      .map(|asset| asset.absolute_path())
  }

  pub fn resolve_dds_texture_path(&self, texture_reference: &str) -> Option<PathBuf> {
    self
      .assets
      .dds_texture(texture_reference)
      .ok()
      .flatten()
      .map(|asset| asset.absolute_path())
  }
}

fn join_logical_path(prefix: &str, path: &str) -> Option<String> {
  let prefix = prefix.trim_matches(['/', '\\']);
  let path = path.trim_start_matches(['/', '\\']);
  match (prefix.is_empty(), path.is_empty()) {
    (true, true) => None,
    (true, false) => Some(path.to_string()),
    (false, true) => Some(prefix.to_string()),
    (false, false) => Some(format!("{prefix}\\{path}")),
  }
}

fn dds_texture_asset_path_from_reference(reference: &str) -> String {
  if let Some((stem, extension)) = reference.rsplit_once('.')
    && ["tga", "dds", "bmp", "ogm"]
      .iter()
      .any(|known| extension.eq_ignore_ascii_case(known))
  {
    format!("{stem}.dds")
  } else {
    format!("{reference}.dds")
  }
}
