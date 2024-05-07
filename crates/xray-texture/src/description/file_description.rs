use crate::description::texture_description::TextureDescription;

pub struct FileDescription {
  pub name: String,
  pub textures: Vec<TextureDescription>,
}

impl FileDescription {
  pub fn new(name: &str) -> FileDescription {
    FileDescription {
      name: name.into(),
      textures: Vec::new(),
    }
  }

  pub fn add_texture(&mut self, texture: TextureDescription) {
    self.textures.push(texture);
  }
}
