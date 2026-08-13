use std::path::PathBuf;

pub struct ProjectInitializeOptions {
  pub output: xrf_output::OutputOptions,
  pub path: PathBuf,
}
