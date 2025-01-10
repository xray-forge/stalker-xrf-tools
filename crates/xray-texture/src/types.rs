use crate::error::TextureError;

pub type TextureResult<T = ()> = Result<T, TextureError>;
