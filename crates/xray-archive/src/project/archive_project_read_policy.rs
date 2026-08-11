use std::path::Path;

use serde::Serialize;

use crate::project::archive_project_constants::{ALLOWED_PROJECT_READ_EXTENSIONS, ALLOWED_PROJECT_READ_SIZE};

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveProjectReadPolicy {
  pub extensions: &'static [&'static str],
  pub maximum_size: u32,
  pub supports_compressed_files: bool,
}

impl ArchiveProjectReadPolicy {
  pub fn supports_file(&self, filename: &str) -> bool {
    Path::new(filename)
      .extension()
      .and_then(|extension| extension.to_str())
      .is_some_and(|extension| {
        self
          .extensions
          .iter()
          .any(|allowed| extension.eq_ignore_ascii_case(allowed))
      })
  }
}

impl Default for ArchiveProjectReadPolicy {
  fn default() -> Self {
    Self {
      extensions: ALLOWED_PROJECT_READ_EXTENSIONS,
      maximum_size: ALLOWED_PROJECT_READ_SIZE,
      supports_compressed_files: false,
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

    assert!(!policy.supports_file("preview.dds"));
    assert!(!policy.supports_file("preview"));
  }
}
