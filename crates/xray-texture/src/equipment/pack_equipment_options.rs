use std::path::PathBuf;

use image_dds::ImageFormat;
use xray_ltx::Ltx;

pub struct PackEquipmentOptions {
  pub ltx: Ltx,
  pub source: PathBuf,
  pub output: xray_output::OutputOptions,
  pub output_path: PathBuf,
  pub gamedata: Option<PathBuf>,
  pub dds_compression_format: ImageFormat,
  pub is_strict: bool,
}
