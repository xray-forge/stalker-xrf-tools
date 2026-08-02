use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
/// A named X-Ray export declaration and its source location.
pub struct ExportDescriptor {
  pub filename: String,
  pub name: String,
  pub comment: Option<String>,
  pub parameters: Vec<ExportParameterDescriptor>,
  pub line: usize,
  pub col: usize,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
/// A callback parameter exposed by an X-Ray export declaration.
pub struct ExportParameterDescriptor {
  pub name: String,
  pub typing: String,
  pub comment: Option<String>,
}
