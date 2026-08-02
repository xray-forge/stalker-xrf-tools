use crate::asset::asset_descriptor::AssetDescriptor;
use crate::project::gamedata_project_options::GamedataProjectReadOptions;
use std::collections::HashMap;
use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{LtxProject, LtxProjectOptions};

pub struct GamedataProject {
  pub assets: HashMap<String, AssetDescriptor>,
  pub ltx_project: LtxProject,
  pub root: PathBuf,
}

impl GamedataProject {
  pub fn open(options: &GamedataProjectReadOptions) -> XRayResult<Self> {
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
            "Invalid gamedata configs folder provided: {}, existing folder with system.ltx is required",
            configs.display()
          ),
        ).into(),
      );
    }

    Ok(Self {
      assets: Self::read_project_assets(options)?,
      ltx_project: LtxProject::open_at_path_opt(
        &configs,
        LtxProjectOptions {
          is_with_schemes_check: true,
          is_strict_check: false,
        },
      )
      .map_err(|error| {
        XRayError::new_asset_error(format!(
          "Failed to open gamedata project ltx configs: {}",
          error
        ))
      })?,
      root: options.root.clone(),
    })
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
