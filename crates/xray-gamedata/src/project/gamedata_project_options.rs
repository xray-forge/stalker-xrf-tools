use std::path::PathBuf;

pub struct GamedataProjectOpenOptions {
  pub roots: Vec<PathBuf>,
  pub configs: PathBuf,
}

impl GamedataProjectOpenOptions {
  pub fn new(roots: Vec<PathBuf>, configs: PathBuf) -> Self {
    Self { roots, configs }
  }
}
