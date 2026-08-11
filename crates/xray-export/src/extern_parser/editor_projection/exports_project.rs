use std::path::PathBuf;

use super::ExportDescriptor;

/// Parsed externs and the project they came from.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportsProject {
  pub root: PathBuf,
  pub declarations: Vec<ExportDescriptor>,
}
