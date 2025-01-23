use crate::{TextureError, TextureResult};
use ddsfile::Dds;
use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder, ImageFormat, RgbaImage};
use image_dds::{dds_from_image, ImageFormat as DDSImageFormat};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

pub fn read_dds_by_path(path: &Path) -> TextureResult<Dds> {
  Dds::read(&mut File::open(path)?).map_err(|error| {
    TextureError::new_processing_error(format!(
      "Failed to read texture by path {:?}, error: {:?}",
      path, error,
    ))
  })
}

pub fn dds_to_image(dds: &Dds) -> TextureResult<RgbaImage> {
  image_dds::image_from_dds(dds, 0).map_err(|error| {
    TextureError::new_processing_error(format!(
      "Failed to convert DDS to RGBA image : {:?}'",
      error,
    ))
  })
}

pub fn save_image_as_ui_dds(
  path: &Path,
  image: &RgbaImage,
  format: DDSImageFormat,
) -> TextureResult {
  dds_from_image(
    image,
    format,
    image_dds::Quality::Slow,
    image_dds::Mipmaps::Disabled,
  )?
  .write(&mut BufWriter::new(File::create(path)?))?;

  Ok(())
}

pub fn save_image_as_ui_png(path: &Path, image: &RgbaImage) -> TextureResult {
  Ok(image.save_with_format(path, ImageFormat::Png)?)
}

pub fn open_dds_as_png(path: &Path) -> TextureResult<(RgbaImage, Vec<u8>)> {
  let image: RgbaImage =
    read_dds_by_path(&PathBuf::from(path)).and_then(|dds| dds_to_image(&dds))?;

  let mut buffer: Vec<u8> = Vec::new();

  PngEncoder::new(buffer.by_ref())
    .write_image(
      image.as_raw(),
      image.width(),
      image.height(),
      ExtendedColorType::Rgba8,
    )
    .expect("Error encoding pixels as PNG");

  Ok((image, buffer))
}
