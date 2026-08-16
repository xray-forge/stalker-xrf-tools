use std::path::PathBuf;

use xrf_dds::ImageFormat;
use xrf_ltx::Ltx;

pub struct PackEquipmentOptions {
  pub ltx: Ltx,
  pub source: PathBuf,
  pub output: xrf_output::OutputOptions,
  pub output_path: PathBuf,
  pub gamedata: Option<PathBuf>,
  pub dds_compression_format: ImageFormat,
  pub is_strict: bool,
}
