use ddsfile::Error as DdsError;
use image::ImageError;
use image_dds::error::CreateImageError;
use image_dds::CreateDdsError;
use std::error::Error;
use std::io::Error as IoError;
use thiserror::Error as ThisError;
use xray_ltx::LtxError;

#[derive(ThisError, Debug)]
pub enum TextureError {
  #[error("Texture processing error: {message:?}")]
  Processing { message: String },
  #[error("Texture create image error: {0}")]
  CreateImage(#[from] CreateImageError),
  #[error("Texture image error: {0}")]
  Image(#[from] ImageError),
  #[error("Texture DDS error: {0}")]
  Dds(#[from] DdsError),
  #[error("Texture generic error: {0}")]
  Io(#[from] IoError),
  #[error("Texture ltx error: {0}")]
  Ltx(#[from] LtxError),
  #[error("Texture create DDS error: {0}")]
  CreateDds(#[from] CreateDdsError),
  #[error("Texture generic error: {0}")]
  Generic(#[from] Box<dyn Error>),
}

impl TextureError {
  pub fn new_processing_error<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::Processing {
      message: message.into(),
    }
  }
}
