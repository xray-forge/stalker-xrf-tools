use crate::project::gamedata_asset_descriptor::GamedataAssetDescriptor;
use crate::project::gamedata_project_options::GamedataProjectOpenOptions;
use crate::GamedataResult;
use std::collections::HashMap;
use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use xray_ltx::{LtxProject, LtxProjectOptions};

pub struct GamedataProject {
  pub assets: HashMap<String, GamedataAssetDescriptor>,
  pub roots: Vec<PathBuf>,
  pub configs: PathBuf,
  pub ltx_project: LtxProject,
}

impl GamedataProject {
  pub fn open(options: &GamedataProjectOpenOptions) -> GamedataResult<Self> {
    let roots: Vec<PathBuf> = options
      .roots
      .iter()
      .filter(|it| {
        if Self::is_valid_gamedata_dir(it) {
          true
        } else {
          println!("Skipping invalid gamedata dir: {:?}", it);

          false
        }
      })
      .cloned()
      .collect();

    if roots.is_empty() {
      return Err(
        io::Error::new(
          ErrorKind::NotFound,
          format!(
            "Invalid gamedata roots provided: {:?}, at least one valid resources root is required",
            options.roots
          ),
        )
        .into(),
      );
    }

    if !Self::is_valid_configs_dir(&options.configs) {
      return Err(
        io::Error::new(
          ErrorKind::NotFound,
          format!(
            "Invalid gamedata configs folder provided: {:?}, existing folder with system.ltx is required",
            options.configs
          ),
        ).into(),
      );
    }

    // todo: Make sure config is part of one of asset roots.
    // todo: Make sure config is part of one of asset roots.
    // todo: Make sure config is part of one of asset roots.

    Ok(Self {
      assets: Self::read_project_assets(
        options,
        &roots.iter().map(|it| it.as_path()).collect::<Vec<&Path>>(),
        &options
          .ignored
          .iter()
          .map(|it| it.as_str())
          .collect::<Vec<&str>>(),
      )?,
      roots,
      configs: options.configs.clone(),
      ltx_project: LtxProject::open_at_path_opt(
        &options.configs,
        LtxProjectOptions {
          is_with_schemes_check: true,
        },
      )?,
    })
  }
}

impl GamedataProject {
  pub fn is_valid_gamedata_dir(path: &Path) -> bool {
    path.exists() && path.is_dir()
  }

  pub fn is_valid_configs_dir(path: &Path) -> bool {
    path.exists() && path.is_dir() && path.join("system.ltx").exists()
  }
}
