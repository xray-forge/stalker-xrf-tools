use base64::engine::GeneralPurpose;
use base64::{alphabet, engine, Engine};
use ddsfile::Dds;
use image::RgbaImage;
use image_dds::{dds_from_image, ImageFormat};
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::path::Path;

pub fn read_dds_by_path(path: &Path) -> io::Result<RgbaImage> {
  Ok(
    image_dds::image_from_dds(
      &Dds::read(&mut File::open(path)?)
        .map_err(|error| io::Error::new(io::ErrorKind::NotFound, error))?,
      0,
    )
    .unwrap(),
  )
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

pub fn image_to_base64(image: &RgbaImage) -> String {
  GeneralPurpose::new(&alphabet::STANDARD, engine::general_purpose::NO_PAD)
    .encode(image.as_raw().as_slice())
}

pub fn image_to_web_base64(image: &RgbaImage) -> String {
  format!("data:image/png;base64,{}", image_to_base64(image))
}
