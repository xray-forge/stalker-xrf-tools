use crate::XRayError;
use image::ImageError;
use std::error::Error;
use std::io::Error as IoError;

impl From<IoError> for XRayError {
  fn from(value: IoError) -> Self {
    Self::Io {
      message: value.to_string(),
      kind: value.kind(),
    }
  }
}

impl From<serde_json::error::Error> for XRayError {
  fn from(value: serde_json::error::Error) -> Self {
    Self::Serde {
      message: value.to_string(),
    }
  }
}

impl From<ImageError> for XRayError {
  fn from(value: ImageError) -> Self {
    Self::TextureProcessing {
      message: value.to_string(),
    }
  }
}

impl From<Box<dyn Error + Send + Sync>> for XRayError {
  fn from(value: Box<dyn Error + Send + Sync>) -> Self {
    Self::Generic {
      message: value.to_string(),
    }
  }
}
