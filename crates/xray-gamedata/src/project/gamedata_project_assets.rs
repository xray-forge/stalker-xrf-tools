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

      let relative_path: &Path = entry_path.strip_prefix(root).map_err(|_| {
        XRayError::new_unexpected_error("Failed to strip prefix from gamedata root path")
      })?;
      let Some(relative): Option<&str> = relative_path.to_str() else {
        log::warn!("Could not strip prefix: {}", entry_path.display());
        continue;
      };
      let logical_path: String = Self::logical_asset_path("", relative);

      if options
        .ignored
        .iter()
        .any(|ignored| logical_path.starts_with(&Self::logical_asset_path("", ignored)))
      {
        continue;
      }

      assets.insert(
        logical_path,
        AssetDescriptor::from_relative_path(relative_path),
      );
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
    let filter: String = Self::logical_asset_path("", filter);

    self
      .assets
      .iter()
      .filter_map(|(path, _)| {
        if path.ends_with(&filter) {
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
    let asset_path: String = Self::logical_asset_path(prefix, relative_path);

    self
      .assets
      .get(&asset_path)
      .map(|descriptor| (self.root.join(&descriptor.relative_path), descriptor))
      .or(None)
  }

  fn logical_asset_path(prefix: &str, relative_path: &str) -> String {
    let prefix: String = prefix.replace('/', "\\").to_lowercase();
    let relative_path: String = relative_path.replace('/', "\\").to_lowercase();
    let prefix: &str = prefix.trim_matches('\\');
    let relative_path: &str = relative_path.trim_start_matches('\\');

    if prefix.is_empty() {
      relative_path.to_string()
    } else if relative_path.is_empty() {
      prefix.to_string()
    } else {
      format!("{prefix}\\{relative_path}")
    }
  }

  pub fn get_prefixed_masked_assets(
    &self,
    prefix: &str,
    mask: &str,
  ) -> Vec<(PathBuf, &AssetDescriptor)> {
    let asset_mask: String = Self::logical_asset_path(prefix, mask);
    let split: Vec<&str> = asset_mask.split('*').collect::<Vec<_>>();

    if split.len() != 2 {
      return Vec::new();
    }

    self
      .assets
      .iter()
      .filter_map(|(path, descriptor)| {
        if path.starts_with(split.first().unwrap()) && path.ends_with(split.last().unwrap()) {
          Some((self.root.join(&descriptor.relative_path), descriptor))
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
  use crate::asset::asset_descriptor::AssetDescriptor;
  use crate::{GamedataProject, GamedataProjectReadOptions};
  use std::collections::HashMap;
  use std::fs;
  use std::path::PathBuf;
  use std::sync::atomic::{AtomicU64, Ordering};
  use xray_error::XRayResult;

  static NEXT_TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

  #[test]
  fn normalizes_host_separators_to_xray_logical_paths() {
    assert_eq!(
      GamedataProject::logical_asset_path("textures", "sky\\clouds.dds"),
      "textures\\sky\\clouds.dds"
    );
    assert_eq!(
      GamedataProject::logical_asset_path("textures", "sky/clouds.dds"),
      "textures\\sky\\clouds.dds"
    );
    assert_eq!(
      GamedataProject::logical_asset_path("meshes", "actors\\stalker_*.omf"),
      "meshes\\actors\\stalker_*.omf"
    );
  }

  #[test]
  fn preserves_physical_path_casing_separately_from_logical_identity() -> XRayResult {
    let unique: u64 = NEXT_TEST_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed);
    let root: PathBuf = std::env::temp_dir().join(format!(
      "xray-gamedata-asset-path-test-{}-{unique}",
      std::process::id()
    ));
    let relative_path: PathBuf = PathBuf::from("Textures").join("Sky").join("Clouds.DDS");
    fs::create_dir_all(root.join("Textures").join("Sky"))?;
    fs::write(root.join(&relative_path), [])?;

    let assets: HashMap<String, AssetDescriptor> =
      GamedataProject::read_project_assets(&GamedataProjectReadOptions {
        root: root.clone(),
        is_silent: true,
        ..Default::default()
      })?;
    let descriptor: &AssetDescriptor = assets
      .get("textures\\sky\\clouds.dds")
      .expect("Expected canonical logical asset identity");

    assert_eq!(descriptor.relative_path, relative_path);
    assert!(root.join(&descriptor.relative_path).is_file());

    fs::remove_dir_all(root)?;

    Ok(())
  }

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
