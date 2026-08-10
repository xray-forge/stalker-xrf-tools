use std::path::Path;

use xray_error::XRayResult;

use super::ExternManifestParser;
use crate::extern_manifest::{ExternExport, ExternParameter, ParsedExtern, ParsedExternManifest};

/// Compatibility projection used by the desktop exports editor.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportDescriptor {
  pub filename: String,
  pub name: String,
  pub comment: Option<String>,
  pub parameters: Vec<ExportParameterDescriptor>,
  pub typing: Option<String>,
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

  /// Scan one project and project every extern for the editor.
  pub fn parse_project_from_path<P: AsRef<Path>>(&self, path: P) -> XRayResult<Vec<ExportDescriptor>> {
    let parsed: ParsedExternManifest = ExternManifestParser::new().parse_directory(path.as_ref())?;

    Ok(self.project(&parsed, |name: &str| Some(name)))
  }

  /// Parse conditions externs and remove the `xr_conditions.` namespace.
  pub fn parse_conditions_from_path<P: AsRef<Path>>(&self, path: P) -> XRayResult<Vec<ExportDescriptor>> {
    self.parse_projected(path.as_ref(), |name: &str| name.strip_prefix("xr_conditions."))
  }

  /// Parse all callable externs as dialog declarations without renaming them.
  pub fn parse_dialogs_from_path<P: AsRef<Path>>(&self, path: P) -> XRayResult<Vec<ExportDescriptor>> {
    self.parse_projected(path.as_ref(), |name: &str| Some(name))
  }

  /// Parse effects externs and remove the `xr_effects.` namespace.
  pub fn parse_effects_from_path<P: AsRef<Path>>(&self, path: P) -> XRayResult<Vec<ExportDescriptor>> {
    self.parse_projected(path.as_ref(), |name: &str| name.strip_prefix("xr_effects."))
  }

  fn parse_projected(&self, path: &Path, filter: impl Fn(&str) -> Option<&str>) -> XRayResult<Vec<ExportDescriptor>> {
    let parsed = ExternManifestParser::new().parse_directory(path)?;

    Ok(self.project(&parsed, filter))
  }

  fn project(&self, parsed: &ParsedExternManifest, filter: impl Fn(&str) -> Option<&str>) -> Vec<ExportDescriptor> {
    let mut result: Vec<ExportDescriptor> = parsed
      .parsed
      .iter()
      .filter_map(|entry: &ParsedExtern| {
        let name: &str = filter(&entry.name)?;
        let (comment, parameters, typing) = match &entry.export {
          ExternExport::Callable(callable) => (
            callable
              .doc
              .as_ref()
              .and_then(|documentation| documentation.description.clone()),
            callable
              .params
              .iter()
              .map(|parameter: &ExternParameter| ExportParameterDescriptor {
                name: parameter.name.clone(),
                typing: parameter.type_name.clone(),
                comment: parameter.doc.clone(),
              })
              .collect(),
            None,
          ),
          ExternExport::Value(value) => (
            value
              .doc
              .as_ref()
              .and_then(|documentation| documentation.description.clone()),
            Vec::new(),
            Some(value.type_name.clone()),
          ),
        };

        Some(ExportDescriptor {
          filename: entry.location.path.clone(),
          name: name.into(),
          comment,
          parameters,
          typing,
          line: entry.location.line,
          col: entry.location.column,
        })
      })
      .collect();

    result.sort_by(|left: &ExportDescriptor, right: &ExportDescriptor| left.name.cmp(&right.name));

    result
  }
}

#[cfg(test)]
mod tests {
  use std::fs;
  use std::path::{Path, PathBuf};

  use super::ExportsEditorParser;

  fn write_source(root: &Path, name: &str, source: &str) {
    let path: PathBuf = root.join(name);

    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, source).unwrap();
  }

  #[test]
  fn parses_all_editor_exports_from_the_project_root() {
    let root: PathBuf = std::env::temp_dir().join(format!("xray-export-editor-project-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);

    write_source(
      &root,
      "src/tsconfig.json",
      r#"{
        "compilerOptions": {
          "baseUrl": "./engine",
          "paths": { "@/*": ["../*"] }
        }
      }"#,
    );
    write_source(
      &root,
      "src/engine/core/check.ts",
      "export function check(value: string): boolean { return value.length > 0; }",
    );
    write_source(
      &root,
      "src/engine/declarations/conditions/check.ts",
      r#"
        import { check } from "@/engine/core/check";
        extern("xr_conditions.check", check);
      "#,
    );
    write_source(
      &root,
      "src/engine/declarations/effects/run.ts",
      "export {}; extern(\"xr_effects.run\", (): void => {});",
    );
    write_source(
      &root,
      "src/engine/declarations/dialogs/hello.ts",
      "export {}; extern(\"dialogs_zaton.hello\", (): void => {});",
    );
    write_source(
      &root,
      "src/engine/scripts/start.ts",
      r#"
        export {};
        extern("settings", rawValue as Record<string, boolean>);
        extern("start", (): void => {});
      "#,
    );
    write_source(
      &root,
      "src/engine/core/decorated.ts",
      "@UnsupportedDecorator() class Decorated {}",
    );
    write_source(&root, "node_modules/invalid.ts", "this is not TypeScript {{{");
    write_source(&root, "target/invalid.ts", "this is not TypeScript {{{");

    let declarations = ExportsEditorParser::new().parse_project_from_path(&root).unwrap();

    assert_eq!(declarations.len(), 5);
    assert_eq!(declarations[0].name, "dialogs_zaton.hello");
    assert_eq!(declarations[1].name, "settings");
    assert_eq!(declarations[1].typing.as_deref(), Some("Record<string, boolean>"));
    assert_eq!(declarations[2].name, "start");
    assert_eq!(declarations[3].name, "xr_conditions.check");
    assert_eq!(declarations[3].parameters[0].typing, "string");
    assert_eq!(declarations[4].name, "xr_effects.run");

    fs::remove_dir_all(root).unwrap();
  }
}
