use ddsfile::Dds;
use image::RgbaImage;
use image_dds::{dds_from_image, ImageFormat};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub fn read_dds(path: &Path) -> RgbaImage {
  image_dds::image_from_dds(&Dds::read(&mut File::open(path).unwrap()).unwrap(), 0).unwrap()
}

pub fn save_image_as_dds(path: &Path, image: &RgbaImage, format: ImageFormat) {
  dds_from_image(
    image,
    format,
    image_dds::Quality::Fast,
    image_dds::Mipmaps::GeneratedAutomatic,
  )
  .unwrap()
  .write(&mut BufWriter::new(File::create(path).unwrap()))
  .unwrap();
}
