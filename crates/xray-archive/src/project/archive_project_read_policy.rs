use std::path::Path;

use serde::Serialize;

use crate::project::archive_project_constants::{
  ALLOWED_PROJECT_AUDIO_EXTENSIONS, ALLOWED_PROJECT_AUDIO_SIZE, ALLOWED_PROJECT_IMAGE_EXTENSIONS,
  ALLOWED_PROJECT_IMAGE_SIZE, ALLOWED_PROJECT_READ_EXTENSIONS, ALLOWED_PROJECT_READ_SIZE,
};

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveProjectReadPolicy {
  pub extensions: &'static [&'static str],
  pub maximum_size: u32,
  /// Extensions decoded into a picture. Compression does not apply: it is undone before decoding.
  pub image_extensions: &'static [&'static str],
  pub maximum_image_size: u32,
  /// Extensions played by the webview itself, so the backend only has to hand over the bytes.
  pub audio_extensions: &'static [&'static str],
  pub maximum_audio_size: u32,
}

impl ArchiveProjectReadPolicy {
  /// Whether this file is one the backend decodes into a picture rather than reading as text.
  pub fn supports_image(&self, filename: &str) -> bool {
    Self::has_extension(filename, self.image_extensions)
  }

  /// Whether this file is one the webview can play back directly.
  pub fn supports_audio(&self, filename: &str) -> bool {
    Self::has_extension(filename, self.audio_extensions)
  }

  pub fn supports_file(&self, filename: &str) -> bool {
    Self::has_extension(filename, self.extensions)
  }

  fn has_extension(filename: &str, extensions: &[&str]) -> bool {
    Path::new(filename)
      .extension()
      .and_then(|extension| extension.to_str())
      .is_some_and(|extension| extensions.iter().any(|allowed| extension.eq_ignore_ascii_case(allowed)))
  }
}

impl Default for ArchiveProjectReadPolicy {
  fn default() -> Self {
    Self {
      extensions: ALLOWED_PROJECT_READ_EXTENSIONS,
      maximum_size: ALLOWED_PROJECT_READ_SIZE,
      image_extensions: ALLOWED_PROJECT_IMAGE_EXTENSIONS,
      maximum_image_size: ALLOWED_PROJECT_IMAGE_SIZE,
      audio_extensions: ALLOWED_PROJECT_AUDIO_EXTENSIONS,
      maximum_audio_size: ALLOWED_PROJECT_AUDIO_SIZE,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::ArchiveProjectReadPolicy;

  #[test]
  fn default_policy_recognizes_supported_extensions_case_insensitively() {
    let policy: ArchiveProjectReadPolicy = ArchiveProjectReadPolicy::default();

    for extension in policy.extensions {
      assert!(policy.supports_file(&format!("preview.{}", extension)));
      assert!(policy.supports_file(&format!("preview.{}", extension.to_uppercase())));
    }

    // Textures are read as images, never as text, which is the whole reason for the second list.
    assert!(!policy.supports_file("preview.dds"));
    assert!(!policy.supports_file("preview"));
    assert!(policy.supports_image("preview.dds"));
    assert!(policy.supports_image("preview.DDS"));
    assert!(!policy.supports_image("preview.ltx"));
    assert!(policy.supports_audio("ambient.ogg"));
    assert!(policy.supports_audio("ambient.OGG"));
    assert!(!policy.supports_audio("preview.dds"));
  }
}
