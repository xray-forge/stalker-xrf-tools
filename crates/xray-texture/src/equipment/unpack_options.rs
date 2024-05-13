use image::RgbaImage;
use image_dds::ImageFormat;
use std::path::PathBuf;
use xray_ltx::Ltx;

pub struct UnpackEquipmentOptions {
  pub ltx: Ltx,
  pub source: RgbaImage,
  pub output: PathBuf,
  pub dds_compression_format: ImageFormat,
  pub is_verbose: bool,
}
