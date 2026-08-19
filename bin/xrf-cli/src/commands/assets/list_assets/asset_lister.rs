use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use xrf_archive::mount_plan;
use xrf_assets::{XrayAssetLocation, XrayMountKind, XrayMountPlan, XrayScope, XrayVfs, implied_install_root};
use xrf_error::XrfResult;

/// Resolved assets and source metadata for one listing.
pub struct AssetListing {
  /// Description of whether the path was treated as an installation or a bare root.
  pub origin: String,
  /// One line per mount, in search order.
  pub mounts: Vec<String>,
  /// Winning entries, one per logical path.
  pub entries: Vec<XrayAssetLocation>,
  /// Entries shadowed by a higher-priority mount, absent unless asked for.
  pub shadowed: Vec<XrayAssetLocation>,
  /// Time spent planning, mounting, and enumerating.
  pub duration: Duration,
}

/// Builds a VFS from an installation or bare root and lists its resolved assets.
///
/// Entries identify their physical containers. Optional shadowed entries expose lower-priority copies of winning paths.
pub struct AssetLister {
  path: PathBuf,
  prefix: Option<String>,
  is_loose_only: bool,
  is_shadowed_included: bool,
}

impl AssetLister {
  pub fn new(path: &Path) -> Self {
    Self {
      is_loose_only: false,
      is_shadowed_included: false,
      path: path.to_path_buf(),
      prefix: None,
    }
  }

  /// Narrows the listing to one logical subtree, such as `configs` or `textures\wpn`.
  pub fn with_prefix(mut self, prefix: Option<&str>) -> Self {
    self.prefix = prefix.map(ToString::to_string);

    self
  }

  /// Restricts the listing to directory mounts, excluding archives.
  pub fn with_loose_only(mut self, is_loose_only: bool) -> Self {
    self.is_loose_only = is_loose_only;

    self
  }

  /// Includes entries hidden by higher-priority mounts.
  pub fn with_shadowed(mut self, is_shadowed_included: bool) -> Self {
    self.is_shadowed_included = is_shadowed_included;

    self
  }

  /// Plans and enumerates the path's asset sources.
  ///
  /// # Errors
  ///
  /// Returns an error when installation metadata cannot be read, decoded, or parsed, or when the requested prefix is not
  /// a valid X-Ray logical path.
  pub fn run(&self) -> XrfResult<AssetListing> {
    let started: Instant = Instant::now();
    let (origin, plan) = self.plan()?;

    let mut vfs: XrayVfs = XrayVfs::new();

    mount_plan(&mut vfs, &plan)?;

    let scope: XrayScope = self.scope()?;
    let entries: Vec<XrayAssetLocation> = vfs.entries(&scope);
    let shadowed: Vec<XrayAssetLocation> = if self.is_shadowed_included {
      Self::shadowed(&vfs, &scope, &entries)
    } else {
      Vec::new()
    };

    Ok(AssetListing {
      duration: started.elapsed(),
      entries,
      mounts: vfs
        .scoped(&scope)
        .map(|mount| {
          format!(
            "{:<9} {} ({})",
            format!("{:?}", mount.kind()),
            mount.source().root_path().display(),
            mount.label()
          )
        })
        .collect(),
      origin,
      shadowed,
    })
  }

  /// Builds a mount plan and a description of its origin.
  ///
  /// A containing installation takes precedence over a bare root because its archives are not visible to a directory walk.
  fn plan(&self) -> XrfResult<(String, XrayMountPlan)> {
    if self.path.join(xrf_assets::FSGAME_FILE_NAME).is_file() {
      return Ok((
        format!("installation {}", self.path.display()),
        XrayMountPlan::from_fsgame(&self.path)?,
      ));
    }

    if let Some(install) = implied_install_root(&self.path) {
      return Ok((
        format!("installation {} around {}", install.display(), self.path.display()),
        XrayMountPlan::from_fsgame(&install)?,
      ));
    }

    Ok((
      format!("root {}", self.path.display()),
      XrayMountPlan::root(&self.path)?,
    ))
  }

  fn scope(&self) -> XrfResult<XrayScope> {
    let scope: XrayScope = if self.is_loose_only {
      XrayScope::of_kind(XrayMountKind::Directory)
    } else {
      XrayScope::all()
    };

    match self.prefix.as_deref() {
      Some(prefix) => scope.with_prefix(prefix),
      None => Ok(scope),
    }
  }

  /// Returns entries hidden by a higher-priority mount.
  ///
  /// The result removes winning path and container pairs from the complete enumeration.
  fn shadowed(vfs: &XrayVfs, scope: &XrayScope, winners: &[XrayAssetLocation]) -> Vec<XrayAssetLocation> {
    let mut shadowed: Vec<XrayAssetLocation> = vfs.entries_all(scope);

    shadowed.retain(|entry| {
      !winners
        .iter()
        .any(|winner| winner.logical_path() == entry.logical_path() && winner.container() == entry.container())
    });

    shadowed
  }
}
