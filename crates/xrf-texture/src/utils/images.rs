use std::fs::File;
use std::io::{BufWriter, Cursor, Write};
use std::path::Path;

use ddsfile::Dds;
use image::codecs::png::PngEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, ExtendedColorType, GenericImage, ImageBuffer, ImageEncoder, ImageFormat, Rgba, RgbaImage};
use image_dds::{ImageFormat as DDSImageFormat, Mipmaps, dds_from_image};
use xrf_error::{XRayError, XRayResult};
use xrf_output::OutputOptions;
use xrf_utils::assert;

/// Scale an image to the given bounds and centre it on a transparent canvas of exactly that size.
///
/// Scaling preserves the aspect ratio, so an image whose proportions differ from the bounds ends up
/// letterboxed rather than distorted. An image that already matches the bounds is returned untouched.
pub fn fit_image_into_bounds(image: DynamicImage, width: u32, height: u32, source: &Path) -> XRayResult<DynamicImage> {
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
    XRayError::new_texture_processing_error(format!("Failed to convert DDS to RGBA image: {}'", error,))
  })
}

/// Write an image as a dds file with the given format and mip chain.
///
/// Dimensions do not have to be multiples of 4. The block compressor pads every mip level out to whole
/// 4x4 blocks itself and records the unpadded size in the header, so the file keeps the exact
/// dimensions the image was built with.
pub fn save_image_as_ui_dds(path: &Path, image: &RgbaImage, format: DDSImageFormat, mipmaps: Mipmaps) -> XRayResult {
  dds_from_image(image, format, image_dds::Quality::Slow, mipmaps)
    .map_err(|it| XRayError::new_texture_processing_error(it.to_string()))?
    .write(&mut BufWriter::new(File::create(path)?))
    .map_err(|it| XRayError::new_texture_processing_error(it.to_string()))?;

  Ok(())
}

/// Warn when the sheet about to be written at `path` is shaped differently from the one it replaces.
///
/// Packing is meant to replace a sheet's pixels, not its geometry. Canvas size and mip chain length are
/// resource state that the packed sprite rectangles do not fully describe, so a sheet that quietly
/// changes shape diverges from its pristine form and from the other resource repositories with nothing
/// in the log to say so. One sheet can also be described by several description files, and packing only
/// some of them would otherwise shrink it without a word.
pub fn warn_on_reshaped_ui_dds(output: &OutputOptions, path: &Path, width: u32, height: u32, mipmap_levels: u32) {
  if !path.is_file() {
    return;
  }

  let existing: Dds = match read_dds_by_path(path) {
    Ok(existing) => existing,
    Err(error) => {
      xrf_output::warning!(
        output,
        "Cannot compare shape against replaced file {}: {}",
        path.display(),
        error
      );

      return;
    }
  };

  if existing.header.width != width
    || existing.header.height != height
    || existing.get_num_mipmap_levels() != mipmap_levels
  {
    xrf_output::warning!(
      output,
      "Replacing {} of {}x{} with {} mipmap levels by {}x{} with {} mipmap levels",
      path.display(),
      existing.header.width,
      existing.header.height,
      existing.get_num_mipmap_levels(),
      width,
      height,
      mipmap_levels
    );
  }
}

pub fn save_image_as_ui_png(path: &Path, image: &RgbaImage) -> XRayResult {
  Ok(image.save_with_format(path, ImageFormat::Png)?)
}

/// Decode a DDS held in memory and re-encode it as PNG.
///
/// The path based variant cannot serve callers whose bytes live inside an archive, and writing them to
/// a temporary file first only to read it back would be doing the same work twice.
pub fn dds_bytes_as_png(bytes: &[u8]) -> XRayResult<(u32, u32, Vec<u8>)> {
  let dds: Dds = Dds::read(&mut Cursor::new(bytes))
    .map_err(|error| XRayError::new_texture_processing_error(format!("Failed to read DDS from memory: {error}.")))?;

  let image: RgbaImage = dds_to_image(&dds)?;

  let mut buffer: Vec<u8> = Vec::new();

  PngEncoder::new(buffer.by_ref())
    .write_image(image.as_raw(), image.width(), image.height(), ExtendedColorType::Rgba8)
    .map_err(|error| XRayError::new_texture_processing_error(format!("Failed to encode DDS as PNG: {error}.")))?;

  Ok((image.width(), image.height(), buffer))
}

pub fn open_dds_as_png<P: AsRef<Path>>(path: P) -> XRayResult<(RgbaImage, Vec<u8>)> {
  let image: RgbaImage = read_dds_by_path(path).and_then(|dds| dds_to_image(&dds))?;

  let mut buffer: Vec<u8> = Vec::new();

  PngEncoder::new(buffer.by_ref())
    .write_image(image.as_raw(), image.width(), image.height(), ExtendedColorType::Rgba8)
    .expect("Error encoding pixels as PNG");

  Ok((image, buffer))
}

#[cfg(test)]
mod tests {
  use std::fs;
  use std::path::PathBuf;

  use ddsfile::Dds;
  use image::RgbaImage;
  use image_dds::{ImageFormat as DDSImageFormat, Mipmaps};

  use super::{read_dds_by_path, save_image_as_ui_dds};

  fn temp_dds_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("xrf-texture-{name}-{}.dds", std::process::id()))
  }

  fn write_and_read(name: &str, width: u32, height: u32, mipmaps: Mipmaps) -> Dds {
    let path: PathBuf = temp_dds_path(name);

    save_image_as_ui_dds(
      &path,
      &RgbaImage::new(width, height),
      DDSImageFormat::BC3RgbaUnorm,
      mipmaps,
    )
    .expect("expect the sheet to be written");

    let dds: Dds = read_dds_by_path(&path).expect("expect the written sheet to be readable");

    fs::remove_file(&path).expect("expect the written sheet to be removable");

    dds
  }

  #[test]
  fn keeps_dimensions_that_are_not_multiples_of_four() {
    let dds: Dds = write_and_read("unaligned", 1023, 1020, Mipmaps::Disabled);

    assert_eq!(
      (dds.header.width, dds.header.height),
      (1023, 1020),
      "Expect block compression to pad internally rather than grow the stored size"
    );
  }

  #[test]
  fn writes_the_requested_mip_chain() {
    let generated: Dds = write_and_read("mipped", 1023, 1020, Mipmaps::GeneratedAutomatic);

    assert_eq!(generated.get_num_mipmap_levels(), 10);
    assert_eq!((generated.header.width, generated.header.height), (1023, 1020));

    let flat: Dds = write_and_read("flat", 1023, 1020, Mipmaps::Disabled);

    assert_eq!(flat.get_num_mipmap_levels(), 1);
  }
}
