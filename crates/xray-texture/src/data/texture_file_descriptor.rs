use crate::data::texture_sprite_descriptor::TextureSpriteDescriptor;
use std::cmp::max;
use xray_error::XRayResult;
use xray_utils::assert_equal;

pub struct TextureFileDescriptor {
  pub name: String,
  pub sprites: Vec<TextureSpriteDescriptor>,
}

impl TextureFileDescriptor {
  pub fn new<T>(name: T) -> Self
  where
    T: Into<String>,
  {
    Self {
      name: name.into(),
      sprites: Vec::new(),
    }
  }

  pub fn add_sprite(&mut self, texture: TextureSpriteDescriptor) {
    self.sprites.push(texture);
  }

  pub fn get_dimension_boundaries(&self) -> XRayResult<(u32, u32)> {
    let mut max_width: u32 = 0;
    let mut max_height: u32 = 0;

    for texture in &self.sprites {
      let (width, height) = texture.get_dimension_boundaries();

      max_width = max(width, max_width);
      max_height = max(height, max_height);
    }

    // Make sure resulting sprites are multiples of 4 for width and height
    max_width = max_width + (4 - max_width % 4);
    max_height = max_height + (4 - max_height % 4);

    assert_equal(
      max_width % 4,
      0,
      "DirectX compression requires texture width to be multiple of 4",
    )?;
    assert_equal(
      max_height % 4,
      0,
      "DirectX compression requires texture height to be multiple of 4",
    )?;

    Ok((max_width, max_height))
  }
}
