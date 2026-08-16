use std::fs::File;
use std::io::{BufWriter, Cursor, Write};
use std::path::Path;

use ddsfile::{Dds, DxgiFormat};
use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder, RgbaImage};
use image_dds::{ImageFormat, Mipmaps, Quality, dds_from_image};
use xrf_error::{XrfError, XrfResult};

use crate::{DdsMetadata, DdsPng};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DdsEncodeOptions {
  pub format: ImageFormat,
  pub quality: Quality,
  pub mipmaps: Mipmaps,
}

impl DdsEncodeOptions {
  pub fn new(format: ImageFormat, quality: Quality, mipmaps: Mipmaps) -> Self {
    Self {
      format,
      quality,
      mipmaps,
    }
  }
}

/// Parsed DDS file with format behavior behind one interface.
pub struct DdsFile {
  dds: Dds,
  file_size: u64,
  metadata_size: u64,
}

impl DdsFile {
  pub fn read_from_path<P: AsRef<Path>>(path: P) -> XrfResult<Self> {
    let mut file: File = File::open(path.as_ref())?;
    let file_size: u64 = file.metadata()?.len();
    let dds: Dds = Dds::read(&mut file).map_err(|error| {
      XrfError::new_texture_processing_error(format!(
        "Failed to read texture by path {}, error: {}",
        path.as_ref().display(),
        error,
      ))
    })?;

    Self::from_parsed(dds, file_size)
  }

  pub fn read_from_bytes(bytes: &[u8]) -> XrfResult<Self> {
    let file_size: u64 = u64::try_from(bytes.len())
      .map_err(|_| XrfError::new_texture_processing_error("DDS input exceeds the supported size range"))?;
    let dds: Dds = Dds::read(&mut Cursor::new(bytes))
      .map_err(|error| XrfError::new_texture_processing_error(format!("Failed to read DDS from memory: {error}.")))?;

    Self::from_parsed(dds, file_size)
  }

  pub fn encode_rgba(image: &RgbaImage, options: DdsEncodeOptions) -> XrfResult<Self> {
    let dds: Dds = dds_from_image(image, options.format, options.quality, options.mipmaps)
      .map_err(|error| XrfError::new_texture_processing_error(error.to_string()))?;
    let data_size: u64 = u64::try_from(dds.data.len())
      .map_err(|_| XrfError::new_texture_processing_error("Encoded DDS exceeds the supported size range"))?;
    let metadata_size: u64 = if dds.header10.is_some() { 148 } else { 128 };
    let file_size: u64 = metadata_size
      .checked_add(data_size)
      .ok_or_else(|| XrfError::new_texture_processing_error("Encoded DDS size overflowed"))?;

    Ok(Self {
      dds,
      file_size,
      metadata_size,
    })
  }

  pub fn metadata(&self) -> DdsMetadata {
    DdsMetadata::from_dds(&self.dds, self.file_size, self.metadata_size)
  }

  pub fn decode_rgba(&self, mipmap_level: u32) -> XrfResult<RgbaImage> {
    image_dds::image_from_dds(&self.dds, mipmap_level).map_err(|error| {
      XrfError::new_texture_processing_error(format!("Failed to convert DDS to RGBA image: {}'", error,))
    })
  }

  pub fn to_png(&self) -> XrfResult<DdsPng> {
    let image: RgbaImage = self.decode_rgba(0)?;
    let mut bytes: Vec<u8> = Vec::new();

    PngEncoder::new(bytes.by_ref())
      .write_image(image.as_raw(), image.width(), image.height(), ExtendedColorType::Rgba8)
      .map_err(|error| XrfError::new_texture_processing_error(format!("Failed to encode DDS as PNG: {error}.")))?;

    Ok(DdsPng {
      width: image.width(),
      height: image.height(),
      bytes,
    })
  }

  pub fn write_to_path(&self, path: &Path) -> XrfResult {
    self
      .dds
      .write(&mut BufWriter::new(File::create(path)?))
      .map_err(|error| XrfError::new_texture_processing_error(error.to_string()))?;

    Ok(())
  }

  pub fn write_to_bytes(&self) -> XrfResult<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::new();

    self
      .dds
      .write(&mut bytes)
      .map_err(|error| XrfError::new_texture_processing_error(error.to_string()))?;

    Ok(bytes)
  }

  pub fn is_xray_compatible(&self) -> bool {
    if let Some(header10) = &self.dds.header10 {
      Self::is_xray_supported_format(header10.dxgi_format)
    } else if let Some(format) = DxgiFormat::try_from_pixel_format(&self.dds.header.spf) {
      Self::is_xray_supported_format(format)
    } else {
      true
    }
  }

  pub fn is_xray_supported_format(format: DxgiFormat) -> bool {
    matches!(
      format,
      DxgiFormat::BC1_UNorm
        | DxgiFormat::BC1_UNorm_sRGB
        | DxgiFormat::BC2_UNorm
        | DxgiFormat::BC2_UNorm_sRGB
        | DxgiFormat::BC3_UNorm
        | DxgiFormat::BC3_UNorm_sRGB
    )
  }

  fn from_parsed(dds: Dds, file_size: u64) -> XrfResult<Self> {
    let data_size: u64 = u64::try_from(dds.data.len())
      .map_err(|_| XrfError::new_texture_processing_error("DDS payload exceeds the supported size range"))?;
    let metadata_size: u64 = file_size
      .checked_sub(data_size)
      .ok_or_else(|| XrfError::new_texture_processing_error("DDS payload is larger than its source"))?;

    Ok(Self {
      dds,
      file_size,
      metadata_size,
    })
  }
}

#[cfg(test)]
mod tests {
  use ddsfile::{AlphaMode, D3D10ResourceDimension, Dds, DxgiFormat, NewDxgiParams};
  use image::RgbaImage;
  use image_dds::{ImageFormat, Mipmaps, Quality};
  use xrf_test_utils::utils::write_generated_test_resource;

  use super::{DdsEncodeOptions, DdsFile};

  fn encoded_file(width: u32, height: u32, mipmaps: Mipmaps) -> DdsFile {
    DdsFile::encode_rgba(
      &RgbaImage::new(width, height),
      DdsEncodeOptions::new(ImageFormat::BC3RgbaUnorm, Quality::Slow, mipmaps),
    )
    .expect("expect the DDS to encode")
  }

  fn dx10_file(format: DxgiFormat) -> DdsFile {
    let dds: Dds = Dds::new_dxgi(NewDxgiParams {
      height: 4,
      width: 4,
      depth: None,
      format,
      mipmap_levels: None,
      array_layers: None,
      caps2: None,
      is_cubemap: false,
      resource_dimension: D3D10ResourceDimension::Texture2D,
      alpha_mode: AlphaMode::Unknown,
    })
    .expect("expect the test DDS to be constructible");
    let mut bytes: Vec<u8> = Vec::new();

    dds.write(&mut bytes).expect("expect the test DDS to serialize");

    DdsFile::read_from_bytes(&bytes).expect("expect the test DDS to parse")
  }

  #[test]
  fn keeps_dimensions_that_are_not_multiples_of_four() {
    let metadata = encoded_file(1023, 1020, Mipmaps::Disabled).metadata();

    assert_eq!((metadata.width, metadata.height), (1023, 1020));
  }

  #[test]
  fn writes_the_requested_mip_chain() {
    let generated = encoded_file(1023, 1020, Mipmaps::GeneratedAutomatic).metadata();
    let flat = encoded_file(1023, 1020, Mipmaps::Disabled).metadata();

    assert_eq!(generated.mipmap_levels, 10);
    assert_eq!(flat.mipmap_levels, 1);
  }

  #[test]
  fn reads_the_same_metadata_from_bytes_and_path() {
    let encoded: DdsFile = encoded_file(64, 32, Mipmaps::Disabled);
    let bytes: Vec<u8> = encoded.write_to_bytes().expect("expect DDS bytes");
    let path = write_generated_test_resource("xrf-dds/read-path.dds", &bytes).expect("expect scratch DDS");

    assert_eq!(
      DdsFile::read_from_bytes(&bytes)
        .expect("expect bytes to parse")
        .metadata(),
      DdsFile::read_from_path(&path).expect("expect path to parse").metadata()
    );
  }

  #[test]
  fn transcodes_the_base_mip_to_png() {
    let png = encoded_file(16, 8, Mipmaps::Disabled)
      .to_png()
      .expect("expect the DDS to transcode");

    assert_eq!((png.width, png.height), (16, 8));
    assert_eq!(&png.bytes[..8], b"\x89PNG\r\n\x1a\n");
  }

  #[test]
  fn identifies_xray_compatible_formats() {
    assert!(dx10_file(DxgiFormat::BC1_UNorm_sRGB).is_xray_compatible());
    assert!(dx10_file(DxgiFormat::BC3_UNorm).is_xray_compatible());
    assert!(!dx10_file(DxgiFormat::BC4_UNorm).is_xray_compatible());
  }

  #[test]
  fn rejects_malformed_bytes() {
    assert!(DdsFile::read_from_bytes(b"not a DDS").is_err());
  }
}
