use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportDescriptor {
  pub filename: String,
  pub name: String,
  pub comment: Option<String>,
  pub parameters: Vec<ExportParameterDescriptor>,
  pub line: usize,
  pub col: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportParameterDescriptor {
  pub name: String,
  pub typing: String,
  pub comment: Option<String>,
}
