use std::error::Error;
use std::io::Error as IoError;

use image::ImageError;

use crate::XrfError;

impl From<IoError> for XrfError {
  fn from(value: IoError) -> Self {
    Self::Io {
      message: value.to_string(),
      kind: value.kind(),
    }
  }
}

impl From<serde_json::error::Error> for XrfError {
  fn from(value: serde_json::error::Error) -> Self {
    Self::Serde {
      message: value.to_string(),
    }
  }
}

impl From<ImageError> for XrfError {
  fn from(value: ImageError) -> Self {
    Self::TextureProcessing {
      message: value.to_string(),
    }
  }
}

impl From<Box<dyn Error + Send + Sync>> for XrfError {
  fn from(value: Box<dyn Error + Send + Sync>) -> Self {
    Self::Generic {
      message: value.to_string(),
    }
  }
}
