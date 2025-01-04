use roxmltree::Node;

/// Description of single texture sprite.
pub struct SpriteDescription {
  pub id: String,
  pub x: u32,
  pub y: u32,
  pub w: u32,
  pub h: u32,
}

impl SpriteDescription {
  pub fn new<T>(id: T, x: u32, y: u32, w: u32, h: u32) -> Self
  where
    T: Into<String>,
  {
    assert!(w > 0, "Expected valid sprite width, got {w} instead");
    assert!(h > 0, "Expected valid sprite height, {h} instead");

    Self {
      id: id.into(),
      x,
      y,
      w,
      h,
    }
  }

  pub fn new_optional_from_node(node: Node) -> Option<Self> {
    let id: Option<&str> = node.attribute("id");
    let x: Option<&str> = node.attribute("x");
    let y: Option<&str> = node.attribute("y");
    let w: Option<&str> = node.attribute("width");
    let h: Option<&str> = node.attribute("height");

    if id.is_none() || x.is_none() || y.is_none() || w.is_none() || h.is_none() {
      None
    } else {
      let id: &str = id?;
      let x: u32 = x?.trim().parse::<u32>().unwrap_or(0);
      let y: u32 = y?.trim().parse::<u32>().unwrap_or(0);
      let w: u32 = w?.trim().parse::<u32>().unwrap_or(0);
      let h: u32 = h?.trim().parse::<u32>().unwrap_or(0);

      if Self::is_valid_size(w, h) {
        Some(Self::new(id, x, y, w, h))
      } else {
        None
      }
    }
  }

  pub fn is_valid_size(w: u32, h: u32) -> bool {
    w > 0 && h > 0
  }

  pub fn get_dimension_boundaries(&self) -> (u32, u32) {
    (self.x + self.w, self.y + self.h)
  }
}
