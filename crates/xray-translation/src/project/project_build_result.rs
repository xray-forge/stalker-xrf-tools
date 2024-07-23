use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectBuildResult {
  pub duration: u128,
}

impl ProjectBuildResult {
  pub fn new() -> ProjectBuildResult {
    ProjectBuildResult { duration: 0 }
  }
}
