use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use xrf_error::XrfResult;
use xrf_vfs::{XrayAssetLocation, XrayMountKind, XrayMountMode, XrayPathCollision, XrayScope, XrayVfs, open_plan};

/// Resolved assets and source metadata for one listing.
pub struct AssetListing {
  /// How the path was interpreted when planning its mounts.
  pub origin: String,
  /// One line per mount, in search order.
  pub mounts: Vec<String>,
  /// Winning entries, one per logical path.
  pub entries: Vec<XrayAssetLocation>,
  /// Entries shadowed by a higher-priority mount, absent unless asked for.
  pub shadowed: Vec<XrayAssetLocation>,
  /// Files a mount holds but cannot reach, because another file in it claims their identity.
  pub collisions: Vec<XrayPathCollision>,
  /// Time spent planning, mounting, and enumerating.
  pub duration: Duration,
}

/// Builds a VFS from an installation or bare root and lists its resolved assets.
///
/// Entries identify their physical containers. Optional shadowed entries expose lower-priority copies of winning paths.
pub struct AssetLister {
  path: PathBuf,
  mode: XrayMountMode,
  prefix: Option<String>,
  ignored: Vec<String>,
  is_loose_only: bool,
  is_shadowed_included: bool,
}

impl AssetLister {
  pub fn new(path: &Path) -> Self {
    Self {
      is_loose_only: false,
      is_shadowed_included: false,
      // A listing exists to show what resolves, so it looks for a containing installation by default; its archives are
      // invisible to a directory walk and omitting them would answer with a fraction of the tree.
      ignored: Vec::new(),
      mode: XrayMountMode::ContainingInstallation,
      path: path.to_path_buf(),
      prefix: None,
    }
  }

  /// Logical prefixes the directory mounts omit, as `verify-gamedata --ignore` means them.
  pub fn with_ignored(mut self, ignored: &[String]) -> Self {
    self.ignored = ignored.to_vec();

    self
  }

  /// Sets how the path is interpreted when planning its mounts.
  pub fn with_mode(mut self, mode: XrayMountMode) -> Self {
    self.mode = mode;

    self
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
    let vfs: XrayVfs = open_plan(&self.mode.plan(&self.path)?.ignoring(&self.ignored)?)?;
    let scope: XrayScope = self.scope()?;
    let entries: Vec<XrayAssetLocation> = vfs.entries(&scope);
    let shadowed: Vec<XrayAssetLocation> = if self.is_shadowed_included {
      Self::shadowed(&vfs, &scope, &entries)
    } else {
      Vec::new()
    };

    Ok(AssetListing {
      collisions: vfs.collisions(&scope),
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
      origin: format!("{} {}", self.mode, self.path.display()),
      shadowed,
    })
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
