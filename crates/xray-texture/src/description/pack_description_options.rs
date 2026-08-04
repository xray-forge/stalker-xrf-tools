use image_dds::ImageFormat;
use std::path::PathBuf;

pub struct PackDescriptionOptions {
  pub description: PathBuf,
  pub base: PathBuf,
  pub output: xray_output::OutputOptions,
  pub output_path: PathBuf,
  pub dds_compression_format: ImageFormat,
  pub is_strict: bool,
  pub is_parallel: bool,
}
