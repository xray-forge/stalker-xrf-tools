use std::path::PathBuf;

use image::RgbaImage;
use image_dds::ImageFormat;
use xrf_ltx::Ltx;

pub struct UnpackEquipmentOptions {
  pub ltx: Ltx,
  pub source: RgbaImage,
  pub output: xrf_output::OutputOptions,
  pub output_path: PathBuf,
  pub dds_compression_format: ImageFormat,
}
