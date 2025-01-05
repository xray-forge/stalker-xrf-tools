use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInitializeResult {
  pub duration: u128,
}

impl ProjectInitializeResult {
  pub fn new() -> Self {
    Self { duration: 0 }
  }
}
