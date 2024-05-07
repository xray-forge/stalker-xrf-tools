use image::RgbaImage;
use std::path::PathBuf;
use xray_ltx::Ltx;

pub struct UnpackEquipmentOptions {
  pub ltx: Ltx,
  pub source: RgbaImage,
  pub output: PathBuf,
  pub is_verbose: bool,
}
