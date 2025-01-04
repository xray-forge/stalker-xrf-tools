use crate::error::texture_processing_error::TextureProcessingError;
use ddsfile::Error as DdsError;
use image::ImageError;
use image_dds::error::CreateImageError;
use image_dds::CreateDdsError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Error as IoError;
use xray_ltx::LtxError;

#[derive(Debug)]
pub enum TextureError {
  CreateDds(CreateDdsError),
  CreateImage(CreateImageError),
  Image(ImageError),
  Dds(DdsError),
  Generic(Box<dyn Error>),
  Processing(TextureProcessingError),
  Io(IoError),
  Ltx(LtxError),
}

impl Display for TextureError {
  fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
    match *self {
      Self::CreateDds(ref error) => error.fmt(formatter),
      Self::CreateImage(ref error) => error.fmt(formatter),
      Self::Dds(ref error) => error.fmt(formatter),
      Self::Image(ref error) => error.fmt(formatter),
      Self::Generic(ref error) => error.fmt(formatter),
      Self::Processing(ref error) => error.fmt(formatter),
      Self::Io(ref error) => error.fmt(formatter),
      Self::Ltx(ref error) => error.fmt(formatter),
    }
  }
}

impl Error for TextureError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match *self {
      Self::CreateDds(ref error) => error.source(),
      Self::CreateImage(ref error) => error.source(),
      Self::Dds(ref error) => error.source(),
      Self::Image(ref error) => error.source(),
      Self::Generic(ref error) => error.source(),
      Self::Processing(ref error) => error.source(),
      Self::Io(ref error) => error.source(),
      Self::Ltx(ref error) => error.source(),
    }
  }
}

impl From<IoError> for TextureError {
  fn from(error: IoError) -> Self {
    Self::Io(error)
  }
}

impl From<CreateImageError> for TextureError {
  fn from(error: CreateImageError) -> Self {
    Self::CreateImage(error)
  }
}

impl From<DdsError> for TextureError {
  fn from(error: DdsError) -> Self {
    Self::Dds(error)
  }
}

impl From<ImageError> for TextureError {
  fn from(error: ImageError) -> Self {
    Self::Image(error)
  }
}

impl From<CreateDdsError> for TextureError {
  fn from(error: CreateDdsError) -> Self {
    Self::CreateDds(error)
  }
}

impl From<LtxError> for TextureError {
  fn from(error: LtxError) -> Self {
    Self::Ltx(error)
  }
}

impl From<Box<dyn Error>> for TextureError {
  fn from(error: Box<dyn Error>) -> Self {
    Self::Generic(error)
  }
}
