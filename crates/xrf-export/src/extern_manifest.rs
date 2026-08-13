use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// The public, stable manifest of names exported to X-Ray configuration data.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ExternManifest {
  pub exports: BTreeMap<String, ExternExport>,
}

/// One exported value. The JSON representation remains compatible with the
/// existing tracked `extern.json` artifact.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExternExport {
  Callable(ExternCallable),
  Value(ExternValue),
}

impl ExternExport {
  pub fn source(&self) -> &str {
    match self {
      Self::Callable(value) => &value.source,
      Self::Value(value) => &value.source,
    }
  }

  pub fn documentation(&self) -> Option<&ExternDocumentation> {
    match self {
      Self::Callable(value) => value.doc.as_ref(),
      Self::Value(value) => value.doc.as_ref(),
    }
  }
}

/// A callable extern with an explicitly declared function contract.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ExternCallable {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub doc: Option<ExternDocumentation>,
  pub params: Vec<ExternParameter>,
  pub returns: String,
  pub source: String,
}

/// A non-callable extern with an explicitly asserted value type.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ExternValue {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub doc: Option<ExternDocumentation>,
  pub source: String,
  #[serde(rename = "type")]
  pub type_name: String,
}

/// Optional human-facing documentation attached to an extern declaration.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ExternDocumentation {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub returns: Option<String>,
}

/// One explicitly typed callable argument.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ExternParameter {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub doc: Option<String>,
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub optional: Option<bool>,
  #[serde(rename = "type")]
  pub type_name: String,
}

/// A parsed declaration together with the location used by project projections.
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedExtern {
  pub export: ExternExport,
  pub location: ExternSourceLocation,
  pub name: String,
}

/// Location in the TypeScript source that declares an extern.
#[derive(Clone, Debug, PartialEq)]
pub struct ExternSourceLocation {
  pub column: usize,
  pub line: usize,
  /// Last line of the declaration, inclusive. Equal to `line` for a single line declaration.
  pub end_line: usize,
  pub path: String,
}

/// A parsed manifest and its non-serialized declaration locations.
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedExternManifest {
  pub manifest: ExternManifest,
  pub parsed: Vec<ParsedExtern>,
}
