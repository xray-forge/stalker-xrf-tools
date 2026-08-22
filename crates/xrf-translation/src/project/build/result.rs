use std::time::Duration;

use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectBuildResult {
  #[serde(with = "xrf_utils::duration_ms")]
  pub duration: Duration,
}

impl ProjectBuildResult {
  pub fn new() -> Self {
    Self {
      duration: Duration::ZERO,
    }
  }
}
