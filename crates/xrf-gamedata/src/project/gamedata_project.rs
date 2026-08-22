use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use xrf_chunk::{ChunkReader, InMemoryChunkDataSource};
use xrf_error::{XrfError, XrfResult};
use xrf_ltx::{LtxProject, LtxProjectOptions};
use xrf_vfs::{
  XrayAsset, XrayAssetType, XrayLogicalPath, XrayLookupScope, XrayMountMode, XrayPathCollision, XraySkippedMount,
  XrayVfs,
};

use crate::project::gamedata_project_options::GamedataProjectReadOptions;

/// Logical directory holding a project's configs.
pub(crate) const CONFIGS_DIRECTORY: &str = "configs";

/// Root config the engine loads, and the one asset whose absence means this is not a usable project.
const SYSTEM_LTX_LOGICAL_PATH: &str = "configs\\system.ltx";

#[derive(Debug)]
pub struct GamedataProject {
  /// Owns the mounted sources, since a config project needs them for the same reasons a check does.
  ///
  /// The project resolves assets through the same mounts under a wider scope: `ltx_project` narrows to `configs`, while an
  /// asset lookup spans the whole tree. One VFS, two scopes, rather than mounting an installation twice.
  pub(crate) ltx_project: LtxProject,
  pub(crate) scope: XrayLookupScope,
  /// Location shown in output, which for an installation is the game directory rather than any one mount.
  pub(crate) root: PathBuf,
}

impl GamedataProject {
  pub fn root(&self) -> &Path {
    &self.root
  }

  /// Opens a project at a path, reading it the way `mode` says.
  ///
  /// A gamedata tree and a game installation are both accepted: an installation mounts its `fsgame.ltx` sources, so the
  /// checks see assets inside `db\` volumes. Every check resolves and reads through the VFS, which is what makes that
  /// honest — while any of them still read a single loose directory, an installation would have reported success over
  /// assets it never looked at.
  ///
  /// # Errors
  ///
  /// Returns an error when the path is not a directory, when the mode cannot plan it, or when the project holds no
  /// `configs\system.ltx`.
  pub fn open_with_mode(mode: XrayMountMode, options: &GamedataProjectReadOptions) -> XrfResult<Self> {
    if !Self::is_valid_gamedata_dir(&options.root) {
      return Err(
        io::Error::new(
          ErrorKind::NotFound,
          format!(
            "Invalid gamedata root provided: {}, an existing directory is required",
            options.root.display()
          ),
        )
        .into(),
      );
    }

    // An ignored prefix covering the root config would make the gate below fail for a perfectly good project, aborting
    // every check — including ones that never read configs. Named explicitly rather than left as a puzzling "nothing
    // resolves", since the cause is the caller's own filter.
    if let Some(ignored) = options
      .ignored
      .iter()
      .find(|prefix| Self::hides_system_ltx(prefix).unwrap_or(false))
    {
      return Err(XrfError::new_invalid_error(format!(
        "Ignored prefix '{ignored}' hides '{SYSTEM_LTX_LOGICAL_PATH}', which every check needs"
      )));
    }

    let vfs: XrayVfs = XrayVfs::from_plan(&mode.plan(&options.root)?.ignoring(&options.ignored)?)?;
    let scope: XrayLookupScope = XrayLookupScope::all();

    // The gate is what the project resolves, not what sits on disk: an installation keeps its configs inside `db\configs`.
    if vfs.scoped(&scope).find(SYSTEM_LTX_LOGICAL_PATH)?.is_none() {
      return Err(
        io::Error::new(
          ErrorKind::NotFound,
          format!(
            "Invalid gamedata provided: {}, nothing resolves '{SYSTEM_LTX_LOGICAL_PATH}'",
            options.root.display()
          ),
        )
        .into(),
      );
    }

    let ltx_project: LtxProject = LtxProject::open_at_scope_opt(
      // The configs directory, not the game root: this project *is* the config tree, and callers join onto its root.
      options.root.join(CONFIGS_DIRECTORY),
      vfs,
      scope.clone().with_prefix(CONFIGS_DIRECTORY)?,
      LtxProjectOptions {
        is_with_schemes_check: true,
        is_strict_check: false,
      },
    )
    .map_err(|error| XrfError::new_asset_error(format!("Failed to open gamedata project ltx configs: {}", error)))?;

    Ok(Self {
      ltx_project,
      root: options.root.clone(),
      scope,
    })
  }

  /// Opens a project, treating the path as an installation only when it declares one.
  pub fn open(options: &GamedataProjectReadOptions) -> XrfResult<Self> {
    Self::open_with_mode(XrayMountMode::Auto, options)
  }

  /// Mounted sources this project resolves through, owned by its config project.
  pub(crate) fn vfs(&self) -> &XrayVfs {
    self.ltx_project.vfs()
  }

  /// Mounts and subtree the project's operations apply to.
  pub(crate) fn scope(&self) -> &XrayLookupScope {
    &self.scope
  }

  /// Reads an asset's bytes through the VFS, whether it is loose or inside an archive volume.
  ///
  /// # Errors
  ///
  /// Returns an error when nothing in scope holds the path, or the source cannot read it.
  pub(crate) fn read_asset(&self, logical_path: &str) -> XrfResult<Vec<u8>> {
    self.vfs().scoped(&self.scope).read(logical_path)
  }

  /// Opens a chunk reader over an asset's bytes.
  ///
  /// The single way a check reads a chunked format, so none of them has to care whether the asset is loose or archived. An
  /// archived entry has no file to slice, which is why this goes through bytes rather than a path.
  ///
  /// # Errors
  ///
  /// Returns an error when the asset cannot be read or holds no chunk.
  pub(crate) fn read_asset_chunks(&self, logical_path: &str) -> XrfResult<ChunkReader<InMemoryChunkDataSource>> {
    ChunkReader::from_vec(self.read_asset(logical_path)?)
  }

  /// Files any mount holds but cannot reach, because another file in the same mount claims their engine identity.
  ///
  /// Reported rather than refused at open time: a tool has to be able to load a project and say what is wrong with it.
  pub fn collisions(&self) -> Vec<XrayPathCollision> {
    self.vfs().scoped(&self.scope).list_collisions()
  }

  /// Sources this project's installation declared that could not be opened.
  ///
  /// Every check runs over what mounted, so a skipped source silently shrinks what verification covers — its assets are
  /// reported missing, or simply never counted. Any report of this project's results has to state these alongside them.
  pub fn skipped_mounts(&self) -> &[XraySkippedMount] {
    self.vfs().get_skipped_mounts()
  }

  /// Whether an ignored prefix would hide the root config every check reads.
  ///
  /// # Errors
  ///
  /// Returns an error when the prefix is not a valid X-Ray logical path.
  fn hides_system_ltx(prefix: &str) -> XrfResult<bool> {
    XrayLogicalPath::new(SYSTEM_LTX_LOGICAL_PATH)?.is_under(prefix)
  }
}

/// Asset access bound to this project's scope.
///
/// A check should never name the scope: the project owns the `(vfs, scope)` pair, and threading both through every call
/// site is how the two drift apart. These delegate to [`XrayVfs`] with the project's own scope supplied, so a check reads
/// as what it wants rather than where to look for it.
impl GamedataProject {
  /// The winning asset for a logical path, or `None` when nothing in the project holds it.
  ///
  /// # Errors
  ///
  /// Returns an error when the path is not a valid X-Ray logical path.
  pub(crate) fn find(&self, logical_path: &str) -> XrfResult<Option<XrayAsset>> {
    self.vfs().scoped(&self.scope).find(logical_path)
  }

  /// Size of an asset without reading it, for a gate that exists to avoid parsing a truncated file.
  pub(crate) fn size(&self, logical_path: &str) -> Option<u64> {
    self.vfs().scoped(&self.scope).read_size(logical_path)
  }

  /// Every asset the project resolves, one per logical path, ordered by that path.
  pub(crate) fn entries(&self) -> Vec<XrayAsset> {
    self.vfs().scoped(&self.scope).list_entries()
  }

  /// Every asset whose extension identifies one kind, wherever in the tree it lives.
  pub(crate) fn entries_of_type(&self, asset_type: XrayAssetType) -> Vec<XrayAsset> {
    self.vfs().scoped(&self.scope).list_entries_of_type(asset_type)
  }

  /// Every asset whose logical path ends with `suffix` on a component boundary.
  ///
  /// # Errors
  ///
  /// Returns an error when `suffix` is not a valid X-Ray logical path fragment.
  pub(crate) fn entries_with_suffix(&self, suffix: &str) -> XrfResult<Vec<XrayAsset>> {
    self.vfs().scoped(&self.scope).list_entries_with_suffix(suffix)
  }

  /// Resolves a raw engine reference of one kind under that kind's directory and extension.
  ///
  /// # Errors
  ///
  /// Returns an error when the kind has no canonical home, or the reference is not a valid X-Ray path.
  pub(crate) fn resolve(&self, asset_type: XrayAssetType, reference: &str) -> XrfResult<Option<XrayAsset>> {
    self.vfs().scoped(&self.scope).resolve(asset_type, reference)
  }

  /// Resolves every asset of one kind a reference names, which may be a `*` mask.
  ///
  /// # Errors
  ///
  /// Returns an error when the kind has no canonical home, the reference is invalid, or a mask carries more than one `*`.
  pub(crate) fn resolve_all(&self, asset_type: XrayAssetType, reference: &str) -> XrfResult<Vec<XrayAsset>> {
    self.vfs().scoped(&self.scope).resolve_all(asset_type, reference)
  }

  /// Resolves an OGF reference under the `meshes` namespace.
  ///
  /// # Errors
  ///
  /// Returns an error when the reference is not a valid X-Ray path.
  pub(crate) fn ogf(&self, reference: &str) -> XrfResult<Option<XrayAsset>> {
    self.resolve(XrayAssetType::Ogf, reference)
  }

  /// Resolves an OMF reference under the `meshes` namespace.
  ///
  /// # Errors
  ///
  /// Returns an error when the reference is not a valid X-Ray path.
  pub(crate) fn omf(&self, reference: &str) -> XrfResult<Option<XrayAsset>> {
    self.resolve(XrayAssetType::Omf, reference)
  }

  /// Resolves a texture reference, appending `.dds` or replacing its authoring extension.
  ///
  /// # Errors
  ///
  /// Returns an error when the reference is not a valid X-Ray path.
  pub(crate) fn dds_texture(&self, reference: &str) -> XrfResult<Option<XrayAsset>> {
    self.resolve(XrayAssetType::Dds, reference)
  }

  /// Reads an asset the project already resolved.
  ///
  /// Preferred over [`Self::read_asset`] when a lookup or enumeration produced the asset: it reads from the source that
  /// answered instead of searching the mounts again by path.
  ///
  /// # Errors
  ///
  /// Returns an error when no mount holds the asset's container, or the source cannot read it.
  pub(crate) fn read_resolved(&self, asset: &XrayAsset) -> XrfResult<Vec<u8>> {
    self.vfs().read_asset(asset)
  }

  /// Opens a chunk reader over an asset the project already resolved.
  ///
  /// # Errors
  ///
  /// Returns an error when the asset cannot be read or holds no chunk.
  pub(crate) fn read_resolved_chunks(&self, asset: &XrayAsset) -> XrfResult<ChunkReader<InMemoryChunkDataSource>> {
    ChunkReader::from_vec(self.read_resolved(asset)?)
  }
}

impl GamedataProject {
  pub fn is_valid_gamedata_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists() && path.as_ref().is_dir()
  }
}
