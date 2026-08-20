use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use xrf_chunk::{ChunkReader, InMemoryChunkDataSource};
use xrf_error::{XrfError, XrfResult};
use xrf_ltx::{LtxProject, LtxProjectOptions};
use xrf_vfs::{XrayLookupScope, XrayMountMode, XrayPathCollision, XrayVfs, open_plan};

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

    let vfs: XrayVfs = open_plan(&mode.plan(&options.root)?.ignoring(&options.ignored)?)?;
    let scope: XrayLookupScope = XrayLookupScope::all();

    // The gate is what the project resolves, not what sits on disk: an installation keeps its configs inside `db\configs`.
    if vfs.find(&scope, SYSTEM_LTX_LOGICAL_PATH)?.is_none() {
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
    self.vfs().read(&self.scope, logical_path)
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
    ChunkReader::from_bytes(&self.read_asset(logical_path)?)
  }

  /// Files any mount holds but cannot reach, because another file in the same mount claims their engine identity.
  ///
  /// Reported rather than refused at open time: a tool has to be able to load a project and say what is wrong with it.
  pub fn collisions(&self) -> Vec<XrayPathCollision> {
    self.vfs().collisions(&self.scope)
  }
}

impl GamedataProject {
  pub fn is_valid_gamedata_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists() && path.as_ref().is_dir()
  }
}
