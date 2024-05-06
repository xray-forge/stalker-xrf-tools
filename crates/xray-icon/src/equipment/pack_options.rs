use std::path::PathBuf;
use xray_ltx::Ltx;

pub struct PackOptions {
  pub ltx: Ltx,
  pub source: PathBuf,
  pub output: PathBuf,
  pub is_verbose: bool,
}
