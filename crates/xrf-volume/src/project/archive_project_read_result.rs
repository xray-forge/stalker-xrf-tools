use serde::{Deserialize, Serialize};

/// One archived text file read for display: its name, decoded content, and unpacked size.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectReadResult {
  /// Entry name the content was read under.
  pub name: String,
  /// Entry text decoded from Windows-1251, like every engine text format.
  pub content: String,
  /// Entry bytes once unpacked, before decoding.
  pub size: u32,
}

impl ProjectReadResult {
  /// Wraps a decoded read for IPC.
  pub fn new(name: &str, content: &str, size: u32) -> Self {
    Self {
      name: name.into(),
      content: content.into(),
      size,
    }
  }
}
