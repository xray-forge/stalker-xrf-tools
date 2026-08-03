use super::ExternManifestParser;
use crate::extern_manifest::{ExternCallable, ExternExport, ExternParameter, ParsedExtern};
use std::path::Path;
use xray_error::XRayResult;

/// Compatibility projection used by the desktop exports editor.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportDescriptor {
  pub filename: String,
  pub name: String,
  pub comment: Option<String>,
  pub parameters: Vec<ExportParameterDescriptor>,
  pub line: usize,
  pub col: usize,
}

/// Compatibility projection of a callable parameter used by the desktop editor.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportParameterDescriptor {
  pub name: String,
  pub typing: String,
  pub comment: Option<String>,
}

/// Projects the canonical parser result for the existing desktop exports editor.
#[derive(Default)]
pub struct ExportsEditorParser;

impl ExportsEditorParser {
  /// Create a parser that projects canonical externs for the desktop editor.
  pub fn new() -> Self {
    Self
  }

  /// Parse conditions externs and remove the `xr_conditions.` namespace.
  pub fn parse_conditions_from_path<P: AsRef<Path>>(
    &self,
    path: P,
  ) -> XRayResult<Vec<ExportDescriptor>> {
    self.parse_projected(path.as_ref(), |name: &str| {
      name.strip_prefix("xr_conditions.")
    })
  }

  /// Parse all callable externs as dialog declarations without renaming them.
  pub fn parse_dialogs_from_path<P: AsRef<Path>>(
    &self,
    path: P,
  ) -> XRayResult<Vec<ExportDescriptor>> {
    self.parse_projected(path.as_ref(), |name: &str| Some(name))
  }

  /// Parse effects externs and remove the `xr_effects.` namespace.
  pub fn parse_effects_from_path<P: AsRef<Path>>(
    &self,
    path: P,
  ) -> XRayResult<Vec<ExportDescriptor>> {
    self.parse_projected(path.as_ref(), |name: &str| name.strip_prefix("xr_effects."))
  }

  fn parse_projected(
    &self,
    path: &Path,
    filter: impl Fn(&str) -> Option<&str>,
  ) -> XRayResult<Vec<ExportDescriptor>> {
    let parsed = ExternManifestParser::new().parse_directory(path)?;

    let mut result: Vec<ExportDescriptor> = parsed
      .parsed
      .iter()
      .filter_map(|entry: &ParsedExtern| {
        let callable: &ExternCallable = match &entry.export {
          ExternExport::Callable(value) => value,
          ExternExport::Value(_) => return None,
        };

        let name: &str = filter(&entry.name)?;

        Some(ExportDescriptor {
          filename: entry.location.path.clone(),
          name: name.into(),
          comment: callable
            .doc
            .as_ref()
            .and_then(|documentation| documentation.description.clone()),
          parameters: callable
            .params
            .iter()
            .map(|parameter: &ExternParameter| ExportParameterDescriptor {
              name: parameter.name.clone(),
              typing: parameter.type_name.clone(),
              comment: parameter.doc.clone(),
            })
            .collect(),
          line: entry.location.line,
          col: entry.location.column,
        })
      })
      .collect();

    result.sort_by(|left: &ExportDescriptor, right: &ExportDescriptor| left.name.cmp(&right.name));

    Ok(result)
  }
}
