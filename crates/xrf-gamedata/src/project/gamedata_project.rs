use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use xrf_error::{XrfError, XrfResult};
use xrf_ltx::{LtxProject, LtxProjectOptions};
use xrf_vfs::{DirectoryAssetIndex, XrayAssetIndex, XrayMountPlan, XrayPathCollision, XrayScope, XrayVfs, open_plan};

use crate::project::gamedata_project_options::GamedataProjectReadOptions;

pub struct GamedataProject {
  pub(crate) assets: XrayAssetIndex,
  pub(crate) ltx_project: LtxProject,
  /// Mounted sources the project resolves through.
  ///
  /// Present alongside `assets` while checks migrate onto it one at a time. The index only ever sees one loose directory, so
  /// a check still reading through it cannot see an installation's archives; the VFS is what makes that possible.
  pub(crate) vfs: XrayVfs,
  pub(crate) scope: XrayScope,
}

impl GamedataProject {
  pub fn root(&self) -> &Path {
    self.assets.root()
  }

  pub fn open(options: &GamedataProjectReadOptions) -> XrfResult<Self> {
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

    let configs: PathBuf = options.root.join("configs");

    if !Self::is_valid_configs_dir(&configs) {
      return Err(
        io::Error::new(
          ErrorKind::NotFound,
          format!(
            "Invalid gamedata configs directory provided: {}, existing directory with system.ltx is required",
            configs.display()
          ),
        )
        .into(),
      );
    }

    Ok(Self {
      assets: XrayAssetIndex::new(DirectoryAssetIndex::read(&options.root)?, &options.ignored)?,
      ltx_project: LtxProject::open_at_path_opt(
        &configs,
        LtxProjectOptions {
          is_with_schemes_check: true,
          is_strict_check: false,
        },
      )
      .map_err(|error| XrfError::new_asset_error(format!("Failed to open gamedata project ltx configs: {}", error)))?,
      scope: XrayScope::all(),
      vfs: open_plan(&XrayMountPlan::root(&options.root)?.ignoring(&options.ignored)?)?,
    })
  }

  /// Files any mount holds but cannot reach, because another file in the same mount claims their engine identity.
  ///
  /// Reported rather than refused at open time: a tool has to be able to load a project and say what is wrong with it.
  pub fn collisions(&self) -> Vec<XrayPathCollision> {
    self.vfs.collisions(&self.scope)
  }
}

impl GamedataProject {
  pub fn is_valid_gamedata_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists() && path.as_ref().is_dir()
  }

  pub fn is_valid_configs_dir<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists() && path.as_ref().is_dir() && path.as_ref().join("system.ltx").exists()
  }
}
