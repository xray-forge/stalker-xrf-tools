use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectReadResult {
  pub name: String,
  pub content: String,
  pub size: u32,
}

impl ProjectReadResult {
  pub fn new(name: &str, content: &str, size: u32) -> Self {
    Self {
      name: name.into(),
      content: content.into(),
      size,
    }
  }
}
