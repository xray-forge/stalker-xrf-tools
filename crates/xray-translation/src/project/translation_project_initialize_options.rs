use std::path::PathBuf;

pub struct ProjectInitializeOptions {
  pub output: xray_output::OutputOptions,
  pub path: PathBuf,
}
