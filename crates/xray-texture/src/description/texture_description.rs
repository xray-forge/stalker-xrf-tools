pub struct TextureDescription {
  pub id: String,
  pub x: u32,
  pub y: u32,
  pub w: u32,
  pub h: u32,
}

impl TextureDescription {
  pub fn new(id: &str, x: u32, y: u32, w: u32, h: u32) -> TextureDescription {
    assert!(w > 0, "Expected valid texture width");
    assert!(h > 0, "Expected valid texture height");

    TextureDescription {
      id: id.into(),
      x,
      y,
      w,
      h,
    }
  }
}
