/// One extern declaration projected for the desktop editor.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportDescriptor {
  pub name: String,
  pub description: Option<String>,
  pub source: ExportSourceDescriptor,
  #[serde(flatten)]
  pub contract: ExportContractDescriptor,
}

/// The mutually exclusive contracts an extern can expose.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ExportContractDescriptor {
  Callable {
    parameters: Vec<ExportParameterDescriptor>,
    returns: ExportReturnDescriptor,
  },
  Value {
    typing: String,
  },
}

/// One callable parameter projected for the desktop editor.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportParameterDescriptor {
  pub name: String,
  pub typing: String,
  pub description: Option<String>,
  pub is_optional: bool,
}

/// The return contract of a callable extern.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportReturnDescriptor {
  pub typing: String,
  pub description: Option<String>,
}

/// Project-relative source location of an extern declaration.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportSourceDescriptor {
  pub path: String,
  pub line: usize,
  pub column: usize,
}
