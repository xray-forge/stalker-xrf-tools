use serde::{Deserialize, Serialize};

#[cfg_attr(
  feature = "typescript-bindings",
  derive(ts_rs::TS),
  ts(export, export_to = "xray-archive.ts")
)]
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
