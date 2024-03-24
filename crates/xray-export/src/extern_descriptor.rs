use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternDescriptor {
  pub name: String,
  pub parameters: Vec<ExternParameterDescriptor>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternParameterDescriptor {
  pub name: String,
  pub typing: String,
  pub comment: Option<String>,
}
