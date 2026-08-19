use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;

use xrf_error::{XrfError, XrfResult};

use crate::xray_path::{is_component_prefix, join, normalize, normalize_host_relative};
use crate::{DirectoryAssetIndex, XrayAsset, XrayAssetType};

#[derive(Debug)]
pub struct XrayAssetIndex {
  directory: DirectoryAssetIndex,
  assets: BTreeMap<String, usize>,
}

impl XrayAssetIndex {
  /// Builds a strict logical-path index over a directory index.
  ///
  /// `ignored` contains logical prefixes to omit. Paths are normalized before comparison, and two
  /// remaining files that normalize to one X-Ray path are rejected.
  ///
  /// # Errors
  ///
  /// Returns an error when an ignored prefix or asset path is invalid, or when normalized paths collide.
  pub fn new(directory: DirectoryAssetIndex, ignored: &[String]) -> XrfResult<Self> {
    let ignored: Vec<String> = ignored.iter().map(|path| normalize(path)).collect::<XrfResult<_>>()?;

    let mut assets: BTreeMap<String, usize> = BTreeMap::new();

    for (index, asset) in directory.assets().enumerate() {
      let logical_path = normalize_host_relative(asset.relative_path())?;

      if ignored.iter().any(|prefix| is_component_prefix(&logical_path, prefix)) {
        continue;
      }

      if let Some(previous) = assets.insert(logical_path.clone(), index) {
        return Err(XrfError::new_asset_error(format!(
          "directory assets '{}' and '{}' have the same logical path '{logical_path}'",
          directory.asset(previous).relative_path().display(),
          asset.relative_path().display()
        )));
      }
    }

    Ok(Self { directory, assets })
  }

  /// Returns the physical directory index used as the source of this logical index.
  pub fn directory(&self) -> &DirectoryAssetIndex {
    &self.directory
  }

  /// Returns the root containing the indexed files.
  pub fn root(&self) -> &Path {
    self.directory.root()
  }

  /// Iterates over indexed assets in normalized logical-path order.
  pub fn assets(&self) -> impl Iterator<Item = XrayAsset<'_>> {
    self.assets.iter().map(|(path, index)| self.asset(path, *index))
  }

  /// Finds an asset by a path normalized to the engine's lower-case backslash form.
  ///
  /// # Errors
  ///
  /// Returns an error when `path` contains invalid or ambiguous components.
  pub fn find(&self, path: &str) -> XrfResult<Option<XrayAsset<'_>>> {
    let path = normalize(path)?;

    Ok(
      self
        .assets
        .get_key_value(&path)
        .map(|(path, index)| self.asset(path, *index)),
    )
  }

  /// Finds an asset below a logical prefix, joining and normalizing both components.
  pub fn find_in(&self, prefix: &str, path: &str) -> XrfResult<Option<XrayAsset<'_>>> {
    self.find(&join(prefix, path)?)
  }

  /// Returns the physical path for a logical asset, if it exists.
  pub fn absolute_path(&self, path: &str) -> XrfResult<Option<PathBuf>> {
    Ok(self.find(path)?.map(|asset| asset.absolute_path()))
  }

  /// Returns the physical path for an asset below a logical prefix, if it exists.
  pub fn absolute_path_in(&self, prefix: &str, path: &str) -> XrfResult<Option<PathBuf>> {
    Ok(self.find_in(prefix, path)?.map(|asset| asset.absolute_path()))
  }

  /// Returns the expected physical location for a valid X-Ray logical path, even when absent.
  pub fn expected_absolute_path(&self, path: &str) -> XrfResult<PathBuf> {
    Ok(self.root().join(normalize(path)?))
  }

  /// Finds an OGF reference below `meshes`.
  pub fn ogf(&self, reference: &str) -> XrfResult<Option<XrayAsset<'_>>> {
    self.find_in("meshes", &crate::xray_path::with_extension(reference, ".ogf"))
  }

  /// Finds an OMF reference below `meshes`.
  pub fn omf(&self, reference: &str) -> XrfResult<Option<XrayAsset<'_>>> {
    self.find_in("meshes", &crate::xray_path::with_extension(reference, ".omf"))
  }

  /// Finds one OMF or all OMF files matching a trailing `*.omf` mask.
  pub fn omfs(&self, reference: &str) -> XrfResult<Vec<XrayAsset<'_>>> {
    if reference.ends_with("*.omf") {
      Ok(self.with_mask_in("meshes", reference)?.collect())
    } else {
      Ok(self.omf(reference)?.into_iter().collect())
    }
  }

  /// Finds a texture reference below `textures`, resolving its authoring extension to `.dds`.
  pub fn dds_texture(&self, reference: &str) -> XrfResult<Option<XrayAsset<'_>>> {
    self.find_in("textures", &crate::texture::dds_logical_path(reference))
  }

  /// Iterates over assets in a normalized logical subtree.
  pub fn with_prefix(&self, prefix: &str) -> XrfResult<impl Iterator<Item = XrayAsset<'_>>> {
    let prefix = normalize(prefix)?;

    Ok(
      self
        .assets
        .iter()
        .filter(move |(path, _)| is_component_prefix(path, &prefix))
        .map(|(path, index)| self.asset(path, *index)),
    )
  }

  /// Iterates over assets with the requested extension-derived type.
  pub fn with_type(&self, asset_type: XrayAssetType) -> impl Iterator<Item = XrayAsset<'_>> {
    self
      .assets
      .iter()
      .filter(move |(path, _)| XrayAssetType::from_logical_path(path) == Some(asset_type))
      .map(|(path, index)| self.asset(path, *index))
  }

  /// Iterates over normalized paths ending with `suffix`.
  pub fn with_suffix(&self, suffix: &str) -> XrfResult<impl Iterator<Item = XrayAsset<'_>>> {
    let suffix = normalize(suffix)?;

    Ok(
      self
        .assets
        .iter()
        .filter(move |(path, _)| path.ends_with(&suffix))
        .map(|(path, index)| self.asset(path, *index)),
    )
  }

  /// Iterates over paths matching one normalized `prefix*suffix` mask.
  ///
  /// # Errors
  ///
  /// Returns an error unless `mask` contains exactly one `*` and has valid path components.
  pub fn with_mask(&self, mask: &str) -> XrfResult<impl Iterator<Item = XrayAsset<'_>>> {
    let mask = normalize(mask)?;

    let Some((start, end)) = mask.split_once('*') else {
      return Err(XrfError::new_asset_error(
        "X-Ray asset mask must contain exactly one '*'",
      ));
    };

    if end.contains('*') {
      return Err(XrfError::new_asset_error(
        "X-Ray asset mask must contain exactly one '*'",
      ));
    }

    let start: String = start.to_string();
    let end: String = end.to_string();

    Ok(
      self
        .assets
        .iter()
        .filter(move |(path, _)| path.starts_with(&start) && path.ends_with(&end))
        .map(|(path, index)| self.asset(path, *index)),
    )
  }

  /// Iterates over assets below `prefix` matching one normalized `prefix*suffix` mask.
  ///
  /// # Errors
  ///
  /// Returns an error unless the joined mask contains exactly one `*` and has valid path components.
  pub fn with_mask_in(&self, prefix: &str, mask: &str) -> XrfResult<impl Iterator<Item = XrayAsset<'_>>> {
    let mask: String = join(prefix, mask)?;

    let Some((start, end)) = mask.split_once('*') else {
      return Err(XrfError::new_asset_error(
        "X-Ray asset mask must contain exactly one '*'",
      ));
    };

    if end.contains('*') {
      return Err(XrfError::new_asset_error(
        "X-Ray asset mask must contain exactly one '*'",
      ));
    }

    let start: String = start.to_string();
    let end: String = end.to_string();

    Ok(
      self
        .assets
        .iter()
        .filter(move |(path, _)| path.starts_with(&start) && path.ends_with(&end))
        .map(|(path, index)| self.asset(path, *index)),
    )
  }

  fn asset<'a>(&'a self, logical_path: &'a str, index: usize) -> XrayAsset<'a> {
    XrayAsset {
      logical_path,
      asset_type: XrayAssetType::from_logical_path(logical_path),
      directory_asset: self.directory.asset(index),
      root: self.directory.root(),
    }
  }
}
