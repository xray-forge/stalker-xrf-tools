use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use xrf_error::{XrfError, XrfResult};

use crate::path::{XrayPath, normalize};
use crate::source::XrayDirectorySource;
use crate::vfs::XrayDirectoryListing;
use crate::{
  XrayAsset, XrayAssetContainer, XrayAssetRules, XrayAssetSource, XrayAssetType, XrayLookupScope, XrayMount,
  XrayMountId, XrayMountKind, XrayPathCollision, XraySkippedMount,
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
  skipped: Vec<XraySkippedMount>,
  /// Paths already mounted from a plan, so a later plan naming the same source reuses it.
  ///
  /// Keyed by the planned path rather than the source's root, because a volume set's root is the common parent of its
  /// volumes: two single-volume plans in one directory share a root while naming different sources.
  planned: HashMap<PathBuf, XrayMountId>,
}

impl XrayVfs {
  /// Creates an empty VFS with no searchable mounts.
  pub fn new() -> Self {
    Self::default()
  }

  /// Sources a plan named that could not be opened.
  ///
  /// Empty for a VFS assembled by hand. Populated by [`crate::mount_plan`], which is tolerant of a source that fails to
  /// open — so a caller that reports on what this VFS holds must report these too, or a mount that silently vanished
  /// looks like content that is silently missing.
  pub fn skipped_mounts(&self) -> &[XraySkippedMount] {
    &self.skipped
  }

  /// Records a source that a plan named but could not open.
  pub(crate) fn record_skipped(&mut self, skipped: XraySkippedMount) {
    self.skipped.push(skipped);
  }

  /// The mount already opened from a planned path, when its kind still matches.
  ///
  /// Opening a source indexes it, so a caller that keeps one VFS across requests — a viewer resolving one model after
  /// another — would otherwise re-walk the same tree and append a duplicate mount every time.
  pub(crate) fn planned_mount(&self, path: &Path, kind: XrayMountKind) -> Option<XrayMountId> {
    self
      .planned
      .get(path)
      .copied()
      .filter(|id| self.mounts.get(id.0).is_some_and(|mount| mount.kind() == kind))
  }

  /// Remembers which mount a planned path produced.
  pub(crate) fn record_planned(&mut self, path: PathBuf, id: XrayMountId) {
    self.planned.insert(path, id);
  }

  /// Appends a source at a logical base with lower priority than existing mounts.
  ///
  /// # Errors
  ///
  /// Returns an error when a non-empty base is not a valid X-Ray logical path.
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
  pub fn scoped(&self, scope: &XrayLookupScope) -> impl Iterator<Item = &XrayMount> {
    self.mounts.iter().filter(move |mount| scope.includes(mount))
  }

  /// The winning location for a logical path, or `None` when no mount in scope holds it.
  ///
  /// # Errors
  ///
  /// Returns an error when the path is not a valid X-Ray logical path. Absence is `Ok(None)`, not an error.
  pub fn find(&self, scope: &XrayLookupScope, logical_path: &str) -> XrfResult<Option<XrayAsset>> {
    let logical_path: String = normalize(logical_path)?;

    if !Self::within_prefix(scope, &logical_path) {
      return Ok(None);
    }

    Ok(self.locate(scope, &logical_path))
  }

  /// Every mount in scope holding a logical path, winner first.
  ///
  /// Includes shadowed copies for override auditing.
  ///
  /// # Errors
  ///
  /// Returns an error when the path is not a valid X-Ray logical path.
  pub fn find_all(&self, scope: &XrayLookupScope, logical_path: &str) -> XrfResult<Vec<XrayAsset>> {
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
  ///
  /// Prefer [`Self::read_asset`] when the asset has already been resolved.
  ///
  /// # Errors
  ///
  /// Returns a not-found error when nothing in scope holds the path, an invalid-path error when it is not a valid X-Ray
  /// logical path, or the source's own error when the bytes cannot be read.
  pub fn read(&self, scope: &XrayLookupScope, logical_path: &str) -> XrfResult<Vec<u8>> {
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

    // Absence is `NotFound` throughout this crate, so a consumer can tell "the asset is not here" from "the source
    // holding it failed" without reading the message.
    Err(XrfError::new_not_found_error(format!(
      "no asset '{logical_path}' in scope across {} mount(s)",
      self.scoped(scope).count()
    )))
  }

  /// Reads the bytes of an asset this VFS already resolved.
  ///
  /// Prefer this over [`Self::read`] whenever a lookup or an enumeration already produced the asset. It reads from the
  /// source that *answered* rather than searching the mounts again, which is both cheaper and more truthful: between a
  /// resolve and a path-keyed read, a remount or a new override can change which mount wins, so the bytes need not be
  /// the ones described by the asset in hand.
  ///
  /// # Errors
  ///
  /// Returns a not-found error when no mount in this VFS holds the asset's container — most often because the asset came
  /// from a different VFS, or its mount has since been replaced.
  pub fn read_asset(&self, asset: &XrayAsset) -> XrfResult<Vec<u8>> {
    let container_root: &Path = match asset.container() {
      XrayAssetContainer::Directory { root, .. } => root,
      XrayAssetContainer::Archive { path } => path,
    };

    let Some(mount) = self
      .mounts
      .iter()
      .find(|mount| mount.source().root_path() == container_root)
    else {
      return Err(XrfError::new_not_found_error(format!(
        "cannot read '{}': no mount in this VFS holds {}",
        asset.logical_path(),
        container_root.display()
      )));
    };

    let Some(source_path) = mount.to_source_path(asset.logical_path().as_str()) else {
      return Err(XrfError::new_not_found_error(format!(
        "cannot read '{}': it falls outside the base of the mount holding it",
        asset.logical_path()
      )));
    };

    mount.source().read(&source_path)
  }

  /// Size in bytes of the winning entry, without reading it.
  ///
  /// For a size gate that exists to avoid parsing a truncated asset: reading the bytes to measure them would defeat it, and
  /// for an archived entry would decompress the whole thing.
  ///
  /// Answers `None` both for an absent asset and for a path that is not a valid logical path — a size gate has nothing
  /// useful to do with the difference, and every caller would discard it.
  pub fn size(&self, scope: &XrayLookupScope, logical_path: &str) -> Option<u64> {
    let logical_path: String = normalize(logical_path).ok()?;

    if !Self::within_prefix(scope, &logical_path) {
      return None;
    }

    self.scoped(scope).find_map(|mount| {
      mount
        .to_source_path(&logical_path)
        .filter(|source_path| mount.source().contains(source_path))
        .and_then(|source_path| mount.source().size(&source_path))
    })
  }

  /// Returns winning entries in scope, one per logical path, ordered by that path.
  ///
  /// Sorted here rather than left to callers: an archive source keys its name table by hash, so enumeration order is
  /// otherwise arbitrary and unstable between runs. Every consumer that shows or diffs a listing needs a deterministic
  /// order, and two shipped defects came from a caller forgetting to impose one. Sorting ~47,000 entries costs
  /// milliseconds against the enumeration itself.
  pub fn entries(&self, scope: &XrayLookupScope) -> Vec<XrayAsset> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut located: Vec<XrayAsset> = Vec::new();

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

    located.sort_by(|first, second| first.logical_path().cmp(second.logical_path()));

    located
  }

  /// Returns winning entries in scope whose extension identifies one kind.
  ///
  /// Not narrowed to the kind's own directory. That directory is where a *reference* resolves, not where
  /// every instance lives: a level ships its own `.dds` files under `levels\<name>\`, and narrowing would drop them from any
  /// enumeration that means "every texture in this project".
  ///
  /// Narrow with a scope prefix when a caller wants one subtree.
  pub fn entries_of_type(&self, scope: &XrayLookupScope, asset_type: XrayAssetType) -> Vec<XrayAsset> {
    self
      .entries(scope)
      .into_iter()
      .filter(|entry| entry.is_type(asset_type))
      .collect()
  }

  /// Returns winning entries in scope whose logical path ends with `suffix` on a component boundary.
  ///
  /// For assets named by convention rather than by extension alone — `particles.xr` libraries, a level's `level.spawn` — where
  /// the tail of the path is the identity and no kind describes it. The boundary matters: a suffix of `particles.xr`
  /// names that file anywhere in the tree, and must not also match a neighbour named `old_particles.xr`.
  ///
  /// # Errors
  ///
  /// Returns an error when `suffix` is not a valid X-Ray logical path fragment.
  pub fn entries_with_suffix(&self, scope: &XrayLookupScope, suffix: &str) -> XrfResult<Vec<XrayAsset>> {
    let suffix: String = normalize(suffix)?;

    Ok(
      self
        .entries(scope)
        .into_iter()
        .filter(|entry| {
          entry
            .logical_path()
            .as_str()
            .strip_suffix(&suffix)
            .is_some_and(|rest| rest.is_empty() || rest.ends_with('\\'))
        })
        .collect(),
    )
  }

  /// Files any mount in scope holds but cannot reach, because another file in the same mount claims their identity.
  ///
  /// An authoring problem to report rather than a reason to refuse the VFS: nothing here affects what resolves, only what a
  /// person should be told is unreachable.
  pub fn collisions(&self, scope: &XrayLookupScope) -> Vec<XrayPathCollision> {
    self
      .scoped(scope)
      .flat_map(|mount| mount.source().collisions().iter().cloned())
      .collect()
  }

  /// Returns what sits directly inside one logical directory, as a browser or a tree view needs it.
  ///
  /// Separate from [`Self::entries`], which answers everything *below* a prefix: listing `textures` with a prefix scope
  /// yields every texture in the tree, while this yields its handful of folders and files. That is the difference between
  /// expanding one node and loading the whole tree.
  ///
  /// Directories are not entries — a volume records them, and treating them as assets inflates every count — so folder
  /// names are derived from the path segments of entries. Cost is therefore proportional to the entries under `directory`,
  /// not to the number of children returned.
  ///
  /// # Errors
  ///
  /// Returns an error when `directory` is not a valid X-Ray logical path. An empty `directory` lists the logical root.
  pub fn children(&self, scope: &XrayLookupScope, directory: &str) -> XrfResult<XrayDirectoryListing> {
    let directory: String = if directory.is_empty() {
      String::new()
    } else {
      normalize(directory)?
    };

    let scope: XrayLookupScope = if directory.is_empty() {
      scope.clone()
    } else {
      scope.clone().with_prefix(&directory)?
    };

    let mut listing: XrayDirectoryListing = Default::default();
    let mut directories: HashSet<String> = HashSet::new();

    for entry in self.entries(&scope) {
      let Some(remainder) = Self::remainder_under(entry.logical_path().as_str(), &directory) else {
        continue;
      };

      match remainder.split_once('\\') {
        Some((child, _)) => {
          if directories.insert(child.to_string()) {
            listing.directories.push(child.to_string());
          }
        }
        None => listing.files.push(entry),
      }
    }

    listing.directories.sort();
    listing.files.sort_by(|a, b| a.logical_path().cmp(b.logical_path()));

    Ok(listing)
  }

  /// The part of a logical path below `directory`, or `None` when it does not sit under it.
  fn remainder_under<'a>(logical_path: &'a str, directory: &str) -> Option<&'a str> {
    if directory.is_empty() {
      return Some(logical_path);
    }

    logical_path
      .strip_prefix(directory)
      .and_then(|rest| rest.strip_prefix('\\'))
  }

  /// Returns every entry in scope, including shadowed copies, ordered by logical path.
  ///
  /// Copies of one path stay in mount priority order, so the winner precedes the entries it shadows.
  pub fn entries_all(&self, scope: &XrayLookupScope) -> Vec<XrayAsset> {
    let mut located: Vec<XrayAsset> = Vec::new();

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

    // Stable, so copies of one logical path keep the mount order they were enumerated in.
    located.sort_by(|first, second| first.logical_path().cmp(second.logical_path()));

    located
  }

  /// Writes bytes to the winning entry.
  ///
  /// The operation refuses read-only winners and absent paths instead of creating a loose override.
  pub fn write(&self, scope: &XrayLookupScope, logical_path: &str, bytes: &[u8]) -> XrfResult<()> {
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

  /// Creates a loose override in the highest-priority writable mount in scope.
  ///
  /// Unlike [`Self::write`], this creates a new entry instead of modifying the current winner. The mount is rebuilt so the
  /// override resolves immediately.
  ///
  /// # Errors
  ///
  /// Returns an error when the path is invalid or out of scope, no writable mount can contain it, the target is already
  /// indexed there, creation or remounting fails, or the new entry does not resolve.
  pub fn write_override(&mut self, scope: &XrayLookupScope, logical_path: &str, bytes: &[u8]) -> XrfResult<XrayAsset> {
    let logical_path: String = normalize(logical_path)?;

    if !Self::within_prefix(scope, &logical_path) {
      return Err(XrfError::new_asset_error(format!(
        "cannot override '{logical_path}': it falls outside the scope's subtree"
      )));
    }

    let Some((id, source_path)) = self
      .scoped(scope)
      .find(|mount| mount.is_writable())
      .map(|mount| (mount.id(), mount.to_source_path(&logical_path)))
    else {
      return Err(XrfError::new_asset_error(format!(
        "cannot override '{logical_path}': no writable mount is in scope"
      )));
    };

    let Some(source_path) = source_path else {
      return Err(XrfError::new_asset_error(format!(
        "cannot override '{logical_path}': it falls outside the writable mount's base"
      )));
    };

    self.mounts[id.0].source().create(&source_path, bytes)?;
    self.remount(id)?;

    self.find(scope, &logical_path)?.ok_or_else(|| {
      XrfError::new_asset_error(format!("override '{logical_path}' did not resolve after being written"))
    })
  }

  /// Reindexes a directory mount so newly created files resolve.
  ///
  /// Non-directory mounts are left unchanged.
  ///
  /// # Errors
  ///
  /// Returns an error when the mount does not exist or its root can no longer be indexed.
  pub fn remount(&mut self, id: XrayMountId) -> XrfResult<()> {
    let Some(mount) = self.mounts.get(id.0) else {
      return Err(XrfError::new_asset_error(format!("no mount {id:?} to remount")));
    };

    if mount.kind() != XrayMountKind::Directory {
      return Ok(());
    }

    let base: String = mount.base().to_string();
    let root: PathBuf = mount.source().root_path().to_path_buf();

    self.mounts[id.0] = XrayMount::new(id, &base, Box::new(XrayDirectorySource::read(&root)?))?;

    Ok(())
  }

  /// Resolves a raw engine reference of one kind, under that kind's directory and extension.
  ///
  /// This is how an editor resolves any kind the table knows without the VFS growing a method per kind. `reference` is
  /// untrusted engine text — from a config field or a mesh header — so normalizing it is this call's job, which is why it
  /// takes `&str` rather than an [`crate::XrayPath`].
  ///
  /// # Errors
  ///
  /// Returns an error when `asset_type` has no canonical home, or when the reference cannot be normalized as an X-Ray path.
  pub fn resolve(
    &self,
    scope: &XrayLookupScope,
    asset_type: XrayAssetType,
    reference: &str,
  ) -> XrfResult<Option<XrayAsset>> {
    let rules: XrayAssetRules = asset_type.rules().ok_or_else(|| {
      XrfError::new_asset_error(format!(
        "asset kind {asset_type:?} has no single directory to resolve under"
      ))
    })?;

    self.find_in(scope, rules.directory, &rules.logical_path(reference))
  }

  /// Resolves every asset of one kind a reference names, which may be a `*` mask.
  ///
  /// A motion reference is allowed to name a set — `wpn\wpn_ak74_*.omf` means every matching animation file — so this
  /// answers a list where [`Self::resolve`] answers at most one. A reference without `*` resolves to a single asset or none,
  /// which is why this is not two separate calls at the consumer.
  ///
  /// # Errors
  ///
  /// Returns an error when the kind has no canonical home, the reference is not a valid X-Ray path, or a mask carries more
  /// than one `*`.
  pub fn resolve_all(
    &self,
    scope: &XrayLookupScope,
    asset_type: XrayAssetType,
    reference: &str,
  ) -> XrfResult<Vec<XrayAsset>> {
    if !reference.contains('*') {
      return Ok(self.resolve(scope, asset_type, reference)?.into_iter().collect());
    }

    let rules: XrayAssetRules = asset_type.rules().ok_or_else(|| {
      XrfError::new_asset_error(format!(
        "asset kind {asset_type:?} has no single directory to resolve under"
      ))
    })?;

    let mask: String = crate::path::join(rules.directory, &rules.logical_path(reference))?;
    let Some((start, end)) = mask.split_once('*') else {
      return Ok(Vec::new());
    };

    if end.contains('*') {
      return Err(XrfError::new_asset_error(
        "X-Ray asset mask must contain exactly one '*'",
      ));
    }

    Ok(
      self
        .entries(&scope.clone().with_prefix(rules.directory)?)
        .into_iter()
        .filter(|entry| {
          let path: &str = entry.logical_path().as_str();

          path.starts_with(start) && path.ends_with(end)
        })
        .collect(),
    )
  }

  /// Resolves a texture reference, appending `.dds` or replacing its authoring extension.
  ///
  /// # Errors
  ///
  /// Returns an error when the reference cannot be normalized as an X-Ray path.
  pub fn dds_texture(&self, scope: &XrayLookupScope, reference: &str) -> XrfResult<Option<XrayAsset>> {
    self.resolve(scope, XrayAssetType::Dds, reference)
  }

  /// Resolves an OGF reference under the `meshes` namespace.
  ///
  /// # Errors
  ///
  /// Returns an error when the reference cannot be normalized as an X-Ray path.
  pub fn ogf(&self, scope: &XrayLookupScope, reference: &str) -> XrfResult<Option<XrayAsset>> {
    self.resolve(scope, XrayAssetType::Ogf, reference)
  }

  /// Resolves an OMF reference under the `meshes` namespace.
  ///
  /// # Errors
  ///
  /// Returns an error when the reference cannot be normalized as an X-Ray path.
  pub fn omf(&self, scope: &XrayLookupScope, reference: &str) -> XrfResult<Option<XrayAsset>> {
    self.resolve(scope, XrayAssetType::Omf, reference)
  }

  fn find_in(&self, scope: &XrayLookupScope, prefix: &str, path: &str) -> XrfResult<Option<XrayAsset>> {
    self.find(scope, &crate::path::join(prefix, path)?)
  }

  /// Checks whether a logical path falls inside the scope's subtree.
  fn within_prefix(scope: &XrayLookupScope, logical_path: &str) -> bool {
    scope
      .prefix()
      .is_none_or(|prefix| crate::path::is_component_prefix(logical_path, prefix))
  }

  fn locate(&self, scope: &XrayLookupScope, logical_path: &str) -> Option<XrayAsset> {
    self
      .scoped(scope)
      .find_map(|mount| Self::locate_in(mount, logical_path))
  }

  /// Pairs a logical path with the physical container reported by the mount's source.
  fn locate_in(mount: &XrayMount, logical_path: &str) -> Option<XrayAsset> {
    let source_path: Cow<'_, str> = mount.to_source_path(logical_path)?;
    let container: XrayAssetContainer = mount.source().locate(&source_path)?;

    Some(XrayAsset::new(
      XrayPath::from_normalized(logical_path.to_string()),
      container,
    ))
  }
}
