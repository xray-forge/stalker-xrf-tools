use crate::error::texture_error::TextureError;

pub type TextureResult<T = ()> = Result<T, TextureError>;
