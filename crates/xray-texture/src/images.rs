use ddsfile::Dds;
use image::RgbaImage;
use image_dds::{dds_from_image, ImageFormat};
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

pub fn save_image_as_ui_dds(path: &Path, image: &RgbaImage, format: ImageFormat) {
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
