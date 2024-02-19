#[derive(Debug)]
pub struct PatrolLink {
  pub index: u32,
  pub links: Vec<(u32, f32)>,
}

impl PatrolLink {
  pub fn new(index: u32) -> PatrolLink {
    PatrolLink {
      index,
      links: Vec::new(),
    }
  }
}
