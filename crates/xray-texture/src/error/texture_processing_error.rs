use crate::TextureError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct TextureProcessingError {
  pub message: String,
}

impl TextureProcessingError {
  pub fn new<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self {
      message: message.into(),
    }
  }

  pub fn new_texture_error<T>(message: T) -> TextureError
  where
    T: Into<String>,
  {
    TextureError::Processing(Self {
      message: message.into(),
    })
  }
}

impl Display for TextureProcessingError {
  fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
    write!(formatter, "TextureProcessingError: {}", self.message)
  }
}

impl Error for TextureProcessingError {}
