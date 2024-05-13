use ddsfile::Dds;
use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat, RgbaImage};
use image_dds::{dds_from_image, ImageFormat as DDSImageFormat};
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::path::Path;

pub fn read_dds_by_path(path: &Path) -> io::Result<Dds> {
  Dds::read(&mut File::open(path)?).map_err(|error| io::Error::new(io::ErrorKind::NotFound, error))
}

pub fn dds_to_image(dds: &Dds) -> io::Result<RgbaImage> {
  Ok(image_dds::image_from_dds(dds, 0).unwrap())
}

pub fn save_image_as_ui_dds(path: &Path, image: &RgbaImage, format: DDSImageFormat) {
  dds_from_image(
    image,
    format,
    image_dds::Quality::Slow,
    image_dds::Mipmaps::Disabled,
  )
  .unwrap()
  .write(&mut BufWriter::new(File::create(path).unwrap()))
  .unwrap();
}

pub fn save_image_as_ui_png(path: &Path, image: &RgbaImage) {
  image.save_with_format(path, ImageFormat::Png).unwrap()
}

pub fn rescale_image_to_bounds(image: DynamicImage, width: u32, _: u32) -> DynamicImage {
  // todo: Also rescale on height?

  if image.width() > width {
    image.resize(
      width,
      (image.height() as f32 * (width as f32 / image.width() as f32)) as u32,
      FilterType::Lanczos3,
    )
  } else {
    image
  }
}
