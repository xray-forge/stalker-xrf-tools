use crate::xray_asset_utils::{is_component_prefix, join, logical_path, normalize};
use crate::{DirectoryAssetIndex, XrayAsset, XrayAssetType};
use std::collections::BTreeMap;
use std::path::Path;
use xray_error::{XRayError, XRayResult};

#[derive(Debug)]
pub struct XrayAssetIndex {
  directory: DirectoryAssetIndex,
  assets: BTreeMap<String, usize>,
}

impl XrayAssetIndex {
  pub fn new(directory: DirectoryAssetIndex, ignored: &[String]) -> XRayResult<Self> {
    let ignored: Vec<String> = ignored
      .iter()
      .map(|path| normalize(path))
      .collect::<XRayResult<_>>()?;

    let mut assets: BTreeMap<String, usize> = BTreeMap::new();

    for (index, asset) in directory.assets().enumerate() {
      let logical_path = logical_path(asset.relative_path())?;

      if ignored
        .iter()
        .any(|prefix| is_component_prefix(&logical_path, prefix))
      {
        continue;
      }

      if let Some(previous) = assets.insert(logical_path.clone(), index) {
        return Err(XRayError::new_asset_error(format!(
          "directory assets '{}' and '{}' have the same logical path '{logical_path}'",
          directory.asset(previous).relative_path().display(),
          asset.relative_path().display()
        )));
      }
    }

    Ok(Self { directory, assets })
  }
  pub fn directory(&self) -> &DirectoryAssetIndex {
    &self.directory
  }

  pub fn root(&self) -> &Path {
    self.directory.root()
  }

  pub fn assets(&self) -> impl Iterator<Item = XrayAsset<'_>> {
    self
      .assets
      .iter()
      .map(|(path, index)| self.asset(path, *index))
  }

  pub fn find(&self, path: &str) -> XRayResult<Option<XrayAsset<'_>>> {
    let path = normalize(path)?;

    Ok(
      self
        .assets
        .get_key_value(&path)
        .map(|(path, index)| self.asset(path, *index)),
    )
  }

  pub fn find_in(&self, prefix: &str, path: &str) -> XRayResult<Option<XrayAsset<'_>>> {
    self.find(&join(prefix, path)?)
  }

  pub fn with_prefix(&self, prefix: &str) -> XRayResult<impl Iterator<Item = XrayAsset<'_>>> {
    let prefix = normalize(prefix)?;

    Ok(
      self
        .assets
        .iter()
        .filter(move |(path, _)| is_component_prefix(path, &prefix))
        .map(|(path, index)| self.asset(path, *index)),
    )
  }

  pub fn with_type(&self, asset_type: XrayAssetType) -> impl Iterator<Item = XrayAsset<'_>> {
    self
      .assets
      .iter()
      .filter(move |(path, _)| XrayAssetType::from_logical_path(path) == Some(asset_type))
      .map(|(path, index)| self.asset(path, *index))
  }

  pub fn with_suffix(&self, suffix: &str) -> XRayResult<impl Iterator<Item = XrayAsset<'_>>> {
    let suffix = normalize(suffix)?;

    Ok(
      self
        .assets
        .iter()
        .filter(move |(path, _)| path.ends_with(&suffix))
        .map(|(path, index)| self.asset(path, *index)),
    )
  }

  pub fn with_mask(&self, mask: &str) -> XRayResult<impl Iterator<Item = XrayAsset<'_>>> {
    let mask = normalize(mask)?;

    let Some((start, end)) = mask.split_once('*') else {
      return Err(XRayError::new_asset_error(
        "X-Ray asset mask must contain exactly one '*'",
      ));
    };

    if end.contains('*') {
      return Err(XRayError::new_asset_error(
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

  pub fn with_mask_in(
    &self,
    prefix: &str,
    mask: &str,
  ) -> XRayResult<impl Iterator<Item = XrayAsset<'_>>> {
    let mask: String = join(prefix, mask)?;

    let Some((start, end)) = mask.split_once('*') else {
      return Err(XRayError::new_asset_error(
        "X-Ray asset mask must contain exactly one '*'",
      ));
    };

    if end.contains('*') {
      return Err(XRayError::new_asset_error(
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
    }
  }
}
