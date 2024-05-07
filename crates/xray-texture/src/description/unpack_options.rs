use std::path::PathBuf;

pub struct UnpackDescriptionOptions {
  pub description: PathBuf,
  pub base: PathBuf,
  pub output: PathBuf,
  pub is_verbose: bool,
  pub is_strict: bool,
}
