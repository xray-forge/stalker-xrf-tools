use crate::data::sprite_description::SpriteDescription;
use std::cmp::max;

pub struct FileDescription {
  pub name: String,
  pub sprites: Vec<SpriteDescription>,
}

impl FileDescription {
  pub fn new<T>(name: T) -> Self
  where
    T: Into<String>,
  {
    Self {
      name: name.into(),
      sprites: Vec::new(),
    }
  }

  pub fn add_sprite(&mut self, texture: SpriteDescription) {
    self.sprites.push(texture);
  }

  pub fn get_dimension_boundaries(&self) -> (u32, u32) {
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

    assert_eq!(
      max_width % 4,
      0,
      "DirectX compression requires texture width to be multiple of 4"
    );
    assert_eq!(
      max_height % 4,
      0,
      "DirectX compression requires texture height to be multiple of 4"
    );

    (max_width, max_height)
  }
}
