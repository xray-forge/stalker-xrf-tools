use crate::asset::asset_descriptor::AssetDescriptor;
use crate::asset::asset_type::AssetType;
use crate::{GamedataProject, GamedataProjectReadOptions};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use xray_error::{XRayError, XRayResult};

impl GamedataProject {
  pub fn read_project_assets(
    options: &GamedataProjectReadOptions,
  ) -> XRayResult<HashMap<String, AssetDescriptor>> {
    let root: &Path = &options.root;

    if options.is_logging_enabled() {
      println!("Reading project assets map in root: {}", root.display());
    }

    let mut assets: HashMap<String, AssetDescriptor> = HashMap::new();

    for entry in WalkDir::new(root) {
      let entry: DirEntry = entry.map_err(|error| error.into_io_error().unwrap())?;
      let entry_path: &Path = entry.path();

      // Dirs are skipped.
      if entry_path.is_dir() {
        continue;
      }

      if let Some(relative) = entry_path
        .strip_prefix(root)
        .map_err(|_| {
          XRayError::new_unexpected_error("Failed to strip prefix from gamedata root path")
        })?
        .to_str()
      {
        if options.ignored.iter().any(|it| relative.starts_with(it)) {
          continue;
        }

        // todo: Descriptor with lowercase?
        assets.insert(
          relative.to_lowercase(),
          AssetDescriptor::new_with_extension(relative),
        );
      } else {
        log::warn!("Could not strip prefix: {}", entry_path.display());
      }
    }

    if options.is_logging_enabled() {
      println!("Read project assets map: {} files", assets.len());
    }

    Ok(assets)
  }
}

impl GamedataProject {
  /// Get list of all asset relative paths by provided type.
  pub fn get_all_asset_paths_by_type(&self, asset_type: AssetType) -> Vec<String> {
    self
      .assets
      .iter()
      .filter_map(|(path, descriptor)| {
        if descriptor.asset_type == asset_type {
          Some(path.clone())
        } else {
          None
        }
      })
      .collect::<Vec<_>>()
  }

  /// Get list of all asset relative paths by provided ending part.
  pub fn get_all_asset_absolute_paths_by_ends_with(&self, filter: &str) -> Vec<PathBuf> {
    self
      .assets
      .iter()
      .filter_map(|(path, _)| {
        if path.ends_with(filter) {
          self.get_absolute_asset_path(path)
        } else {
          None
        }
      })
      .collect::<Vec<_>>()
  }

  pub fn get_absolute_asset_path(&self, relative_path: &str) -> Option<PathBuf> {
    self.get_prefixed_absolute_asset_path("", relative_path)
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
  ) -> Option<(PathBuf, &AssetDescriptor)> {
    let asset_path: PathBuf =
      PathBuf::from(prefix.to_lowercase()).join(relative_path.to_lowercase());

    self
      .assets
      .get(asset_path.to_str().unwrap())
      .map(|descriptor| (self.root.join(&asset_path), descriptor))
      .or(None)
  }

  pub fn get_prefixed_masked_assets(
    &self,
    prefix: &str,
    mask: &str,
  ) -> Vec<(PathBuf, &AssetDescriptor)> {
    let asset_mask: PathBuf = PathBuf::from(prefix.to_lowercase()).join(mask.to_lowercase());
    let split: Vec<&str> = asset_mask.to_str().unwrap().split('*').collect::<Vec<_>>();

    if split.len() != 2 {
      return Vec::new();
    }

    self
      .assets
      .iter()
      .filter_map(|(path, descriptor)| {
        if path.starts_with(split.first().unwrap()) && path.ends_with(split.last().unwrap()) {
          Some((self.root.join(path), descriptor))
        } else {
          None
        }
      })
      .collect::<Vec<_>>()
  }

  pub fn get_ogf_path(&self, visual_path: &str) -> Option<PathBuf> {
    self.get_mesh_path(visual_path, ".ogf")
  }

  pub fn get_omf_path(&self, visual_path: &str) -> Option<PathBuf> {
    self.get_mesh_path(visual_path, ".omf")
  }

  pub fn get_omf_paths(&self, visual_path: &str) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();

    if visual_path.ends_with("*.omf") {
      for (path, _) in self.get_prefixed_masked_assets("meshes", visual_path) {
        paths.push(path);
      }
    } else if let Some(path) = self.get_mesh_path(visual_path, ".omf") {
      paths.push(path);
    }

    paths
  }

  pub fn get_mesh_path(&self, visual_path: &str, extension: &str) -> Option<PathBuf> {
    let mut visual_path: String = String::from(visual_path);

    if !visual_path.ends_with(extension) {
      visual_path.push_str(extension);
    }

    self.get_prefixed_absolute_asset_path("meshes", &visual_path)
  }

  pub fn resolve_dds_texture_path(&self, texture_reference: &str) -> Option<PathBuf> {
    let texture_path: String = Self::dds_texture_asset_path_from_reference(texture_reference);

    self.get_prefixed_absolute_asset_path("textures", &texture_path)
  }

  /// Resolve a renderer texture reference to its assembled DDS asset path.
  ///
  /// X-Ray strips known authoring extensions before loading the corresponding
  /// DDS. Preserve every other byte so validation does not accept references
  /// that the renderer would fail to resolve, including surrounding whitespace.
  fn dds_texture_asset_path_from_reference(texture_reference: &str) -> String {
    if let Some((stem, extension)) = texture_reference.rsplit_once('.') {
      for renderer_extension in ["tga", "dds", "bmp", "ogm"] {
        if extension.eq_ignore_ascii_case(renderer_extension) {
          return format!("{stem}.dds");
        }
      }
    }

    format!("{texture_reference}.dds")
  }
}

#[cfg(test)]
mod tests {
  use crate::GamedataProject;

  #[test]
  fn resolves_renderer_authoring_extensions_to_dds() {
    assert_eq!(
      GamedataProject::dds_texture_asset_path_from_reference("pfx\\pfx_ani-fire01.bmp"),
      "pfx\\pfx_ani-fire01.dds"
    );
    assert_eq!(
      GamedataProject::dds_texture_asset_path_from_reference("pfx\\pfx_smoke_b.tga"),
      "pfx\\pfx_smoke_b.dds"
    );
    assert_eq!(
      GamedataProject::dds_texture_asset_path_from_reference("pfx\\pfx_smoke_b.DDS"),
      "pfx\\pfx_smoke_b.dds"
    );
    assert_eq!(
      GamedataProject::dds_texture_asset_path_from_reference("video\\intro.ogm"),
      "video\\intro.dds"
    );
  }

  #[test]
  fn appends_dds_to_extensionless_and_unknown_extension_references() {
    assert_eq!(
      GamedataProject::dds_texture_asset_path_from_reference("pfx\\pfx_dist5"),
      "pfx\\pfx_dist5.dds"
    );
    assert_eq!(
      GamedataProject::dds_texture_asset_path_from_reference("pfx\\pfx_dist5.png"),
      "pfx\\pfx_dist5.png.dds"
    );
  }

  #[test]
  fn preserves_whitespace_that_would_make_a_renderer_reference_invalid() {
    assert_eq!(
      GamedataProject::dds_texture_asset_path_from_reference(" pfx\\pfx_dist5.bmp"),
      " pfx\\pfx_dist5.dds"
    );
    assert_eq!(
      GamedataProject::dds_texture_asset_path_from_reference("pfx\\pfx_dist5.bmp "),
      "pfx\\pfx_dist5.bmp .dds"
    );
  }
}
