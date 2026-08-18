use std::collections::HashSet;
use std::path::Path;

use xrf_error::{XrfError, XrfResult};

use crate::xray_asset_utils::normalize;
use crate::{
  XrayAssetContainer, XrayAssetLocation, XrayAssetSource, XrayDirectorySource, XrayMount, XrayMountId, XrayMountKind,
  XrayScope,
};

/// The engine's view of assets: several mounted sources, searched in order, first hit wins.
///
/// Mount higher-priority sources first. This produces the same winner as `CLocatorAPI` when callers reverse the engine's
/// last-registration-wins order, while retaining shadowed entries for inspection.
///
/// Mounting indexes sources eagerly. Duplicate logical paths remain errors within one source and become ordinary
/// shadowing across mounts.
#[derive(Debug, Default)]
pub struct XrayVfs {
  mounts: Vec<XrayMount>,
}

impl XrayVfs {
  /// Creates an empty VFS with no searchable mounts.
  pub fn new() -> Self {
    Self::default()
  }

  /// Appends a source at a logical base with lower priority than existing mounts.
  pub fn mount(&mut self, base: &str, source: Box<dyn XrayAssetSource>) -> XrfResult<XrayMountId> {
    let id: XrayMountId = XrayMountId(self.mounts.len());

    log::info!("Mounting {} at base '{base}' as {id:?}", source.label());

    self.mounts.push(XrayMount::new(id, base, source)?);

    Ok(id)
  }

  /// Mounts a directory once, reusing the existing mount for the same root.
  ///
  /// The first mount's base and priority are retained when a root is reused.
  ///
  /// # Errors
  ///
  /// Returns an error when the base is invalid or the directory cannot be indexed.
  pub fn mount_directory(&mut self, base: &str, root: impl AsRef<Path>) -> XrfResult<XrayMountId> {
    let root: &Path = root.as_ref();

    if let Some(mount) = self.directory_mount_at(root) {
      return Ok(mount);
    }

    self.mount(base, Box::new(XrayDirectorySource::read(root)?))
  }

  /// Returns the mount already covering a directory root.
  pub fn directory_mount_at(&self, root: &Path) -> Option<XrayMountId> {
    self
      .mounts
      .iter()
      .find(|mount| mount.kind() == XrayMountKind::Directory && mount.source().root_path() == root)
      .map(XrayMount::id)
  }

  /// Returns mounts in search priority order.
  pub fn mounts(&self) -> &[XrayMount] {
    &self.mounts
  }

  /// Returns the number of mounts, including mounts that contain no matching entry for a scope.
  pub fn mount_count(&self) -> usize {
    self.mounts.len()
  }

  /// Returns whether no source has been mounted.
  pub fn is_empty(&self) -> bool {
    self.mounts.is_empty()
  }

  /// Iterates over mounts selected by a scope, preserving priority order.
  pub fn scoped(&self, scope: &XrayScope) -> impl Iterator<Item = &XrayMount> {
    self.mounts.iter().filter(move |mount| scope.includes(mount))
  }

  /// The winning location for a logical path, or `None` when no mount in scope holds it.
  pub fn find(&self, scope: &XrayScope, logical_path: &str) -> XrfResult<Option<XrayAssetLocation>> {
    let logical_path: String = normalize(logical_path)?;

    if !Self::within_prefix(scope, &logical_path) {
      return Ok(None);
    }

    Ok(self.locate(scope, &logical_path))
  }

  /// Every mount in scope holding a logical path, winner first.
  ///
  /// Includes shadowed copies for override auditing.
  pub fn find_all(&self, scope: &XrayScope, logical_path: &str) -> XrfResult<Vec<XrayAssetLocation>> {
    let logical_path: String = normalize(logical_path)?;

    if !Self::within_prefix(scope, &logical_path) {
      return Ok(Vec::new());
    }

    Ok(
      self
        .scoped(scope)
        .filter_map(|mount| Self::locate_in(mount, &logical_path))
        .collect(),
    )
  }

  /// Reads bytes from the winning entry for a logical path.
  pub fn read(&self, scope: &XrayScope, logical_path: &str) -> XrfResult<Vec<u8>> {
    let logical_path: String = normalize(logical_path)?;

    if Self::within_prefix(scope, &logical_path) {
      for mount in self.scoped(scope) {
        if let Some(source_path) = mount.to_source_path(&logical_path)
          && mount.source().contains(&source_path)
        {
          return mount.source().read(&source_path);
        }
      }
    }

    Err(XrfError::new_asset_error(format!(
      "no asset '{logical_path}' in scope across {} mount(s)",
      self.scoped(scope).count()
    )))
  }

  /// Returns winning entries in scope, one per logical path.
  pub fn entries(&self, scope: &XrayScope) -> Vec<XrayAssetLocation> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut located: Vec<XrayAssetLocation> = Vec::new();

    for mount in self.scoped(scope) {
      let Some(source_prefix) = mount.to_source_prefix(scope.prefix()) else {
        continue;
      };

      for source_path in mount.source().entries(source_prefix.as_deref()) {
        let Ok(logical_path) = mount.to_logical_path(&source_path) else {
          continue;
        };

        if seen.insert(logical_path.clone())
          && let Some(location) = Self::locate_in(mount, &logical_path)
        {
          located.push(location);
        }
      }
    }

    located
  }

  /// Returns every entry in scope, including shadowed copies.
  pub fn entries_all(&self, scope: &XrayScope) -> Vec<XrayAssetLocation> {
    let mut located: Vec<XrayAssetLocation> = Vec::new();

    for mount in self.scoped(scope) {
      let Some(source_prefix) = mount.to_source_prefix(scope.prefix()) else {
        continue;
      };

      for source_path in mount.source().entries(source_prefix.as_deref()) {
        if let Ok(logical_path) = mount.to_logical_path(&source_path)
          && let Some(location) = Self::locate_in(mount, &logical_path)
        {
          located.push(location);
        }
      }
    }

    located
  }

  /// Writes bytes to the winning entry.
  ///
  /// The operation refuses read-only winners and absent paths instead of creating a loose override.
  pub fn write(&self, scope: &XrayScope, logical_path: &str, bytes: &[u8]) -> XrfResult<()> {
    let logical_path: String = normalize(logical_path)?;

    if Self::within_prefix(scope, &logical_path) {
      for mount in self.scoped(scope) {
        let Some(source_path) = mount.to_source_path(&logical_path) else {
          continue;
        };

        if !mount.source().contains(&source_path) {
          continue;
        }

        if !mount.is_writable() {
          return Err(XrfError::new_asset_error(format!(
            "cannot write '{logical_path}': it is held by {} '{}', which is read only",
            match mount.kind() {
              XrayMountKind::Archive => "archive",
              XrayMountKind::Directory => "directory",
            },
            mount.label()
          )));
        }

        return mount.source().write(&source_path, bytes);
      }
    }

    Err(XrfError::new_asset_error(format!(
      "cannot write '{logical_path}': no mount in scope holds it"
    )))
  }

  /// Resolves a texture reference under the `textures` namespace after appending `.dds` or replacing its authoring extension.
  ///
  /// # Errors
  ///
  /// Returns an error when the reference cannot be normalized as an X-Ray path.
  pub fn dds_texture(&self, scope: &XrayScope, reference: &str) -> XrfResult<Option<XrayAssetLocation>> {
    self.find_in(scope, "textures", &crate::texture::dds_logical_path(reference))
  }

  /// Resolves an OGF reference under the `meshes` namespace.
  ///
  /// # Errors
  ///
  /// Returns an error when the reference cannot be normalized as an X-Ray path.
  pub fn ogf(&self, scope: &XrayScope, reference: &str) -> XrfResult<Option<XrayAssetLocation>> {
    self.find_in(scope, "meshes", &crate::xray_path::with_extension(reference, ".ogf"))
  }

  /// Resolves an OMF reference under the `meshes` namespace.
  ///
  /// # Errors
  ///
  /// Returns an error when the reference cannot be normalized as an X-Ray path.
  pub fn omf(&self, scope: &XrayScope, reference: &str) -> XrfResult<Option<XrayAssetLocation>> {
    self.find_in(scope, "meshes", &crate::xray_path::with_extension(reference, ".omf"))
  }

  fn find_in(&self, scope: &XrayScope, prefix: &str, path: &str) -> XrfResult<Option<XrayAssetLocation>> {
    self.find(scope, &crate::xray_asset_utils::join(prefix, path)?)
  }

  /// Checks whether a logical path falls inside the scope's subtree.
  fn within_prefix(scope: &XrayScope, logical_path: &str) -> bool {
    scope
      .prefix()
      .is_none_or(|prefix| crate::xray_asset_utils::is_component_prefix(logical_path, prefix))
  }

  fn locate(&self, scope: &XrayScope, logical_path: &str) -> Option<XrayAssetLocation> {
    self
      .scoped(scope)
      .find_map(|mount| Self::locate_in(mount, logical_path))
  }

  /// Pairs a logical path with the physical container reported by the mount's source.
  fn locate_in(mount: &XrayMount, logical_path: &str) -> Option<XrayAssetLocation> {
    let source_path: String = mount.to_source_path(logical_path)?;
    let container: XrayAssetContainer = mount.source().locate(&source_path)?;

    Some(XrayAssetLocation::new(logical_path.to_string(), container))
  }
}
