use image_dds::ImageFormat;
use std::path::PathBuf;

pub struct PackDescriptionOptions {
  pub description: PathBuf,
  pub base: PathBuf,
  pub output: PathBuf,
  pub dds_compression_format: ImageFormat,
  pub is_verbose: bool,
  pub is_strict: bool,
}
