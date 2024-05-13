use image_dds::ImageFormat;
use std::path::PathBuf;
use xray_ltx::Ltx;

pub struct PackEquipmentOptions {
  pub ltx: Ltx,
  pub source: PathBuf,
  pub output: PathBuf,
  pub gamedata: Option<PathBuf>,
  pub dds_compression_format: ImageFormat,
  pub is_verbose: bool,
  pub is_strict: bool,
}
