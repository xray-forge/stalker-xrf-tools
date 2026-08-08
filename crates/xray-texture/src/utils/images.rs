use ddsfile::Dds;
use image::codecs::png::PngEncoder;
use image::imageops::FilterType;
use image::{
  DynamicImage, ExtendedColorType, GenericImage, ImageBuffer, ImageEncoder, ImageFormat, Rgba,
  RgbaImage,
};
use image_dds::{ImageFormat as DDSImageFormat, dds_from_image};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use xray_error::{XRayError, XRayResult};
use xray_utils::assert;

/// Scale an image to the given bounds and centre it on a transparent canvas of exactly that size.
///
/// Scaling preserves the aspect ratio, so an image whose proportions differ from the bounds ends up
/// letterboxed rather than distorted. An image that already matches the bounds is returned untouched.
pub fn fit_image_into_bounds(
  image: DynamicImage,
  width: u32,
  height: u32,
  source: &Path,
) -> XRayResult<DynamicImage> {
  let image_width: u32 = image.width();
  let image_height: u32 = image.height();

  if image_width == width && image_height == height {
    return Ok(image);
  }

  log::info!(
    "Rescaling image to bounds: {}x{} from {}x{} {}",
    width,
    height,
    image_width,
    image_height,
    source.display()
  );

  let rescaled_image: DynamicImage = image.resize(width, height, FilterType::Lanczos3);
  let rescaled_width: u32 = rescaled_image.width();
  let rescaled_height: u32 = rescaled_image.height();

  if rescaled_width == width && rescaled_height == height {
    return Ok(rescaled_image);
  }

  log::info!(
    "Re-center rescaled image to bounds: {}x{} from {}x{} {}",
    width,
    height,
    rescaled_width,
    rescaled_height,
    source.display()
  );

  let mut centered: ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(width, height);

  assert(
    rescaled_width <= width,
    "Unexpected width {rescaled_width} > {width} when rescaling",
  )?;
  assert(
    rescaled_height <= height,
    "Unexpected height {rescaled_height} > {height} when rescaling",
  )?;

  centered.copy_from(
    &rescaled_image,
    (width - rescaled_width) / 2,
    (height - rescaled_height) / 2,
  )?;

  Ok(centered.into())
}

pub fn read_dds_by_path<P: AsRef<Path>>(path: P) -> XRayResult<Dds> {
  Dds::read(&mut File::open(path.as_ref())?).map_err(|error| {
    XRayError::new_texture_processing_error(format!(
      "Failed to read texture by path {}, error: {}",
      path.as_ref().display(),
      error,
    ))
  })
}

pub fn dds_to_image(dds: &Dds) -> XRayResult<RgbaImage> {
  image_dds::image_from_dds(dds, 0).map_err(|error| {
    XRayError::new_texture_processing_error(format!(
      "Failed to convert DDS to RGBA image: {}'",
      error,
    ))
  })
}

pub fn save_image_as_ui_dds(path: &Path, image: &RgbaImage, format: DDSImageFormat) -> XRayResult {
  dds_from_image(
    image,
    format,
    image_dds::Quality::Slow,
    image_dds::Mipmaps::Disabled,
  )
  .map_err(|it| XRayError::new_texture_processing_error(it.to_string()))?
  .write(&mut BufWriter::new(File::create(path)?))
  .map_err(|it| XRayError::new_texture_processing_error(it.to_string()))?;

  Ok(())
}

pub fn save_image_as_ui_png(path: &Path, image: &RgbaImage) -> XRayResult {
  Ok(image.save_with_format(path, ImageFormat::Png)?)
}

pub fn open_dds_as_png<P: AsRef<Path>>(path: P) -> XRayResult<(RgbaImage, Vec<u8>)> {
  let image: RgbaImage = read_dds_by_path(path).and_then(|dds| dds_to_image(&dds))?;

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
