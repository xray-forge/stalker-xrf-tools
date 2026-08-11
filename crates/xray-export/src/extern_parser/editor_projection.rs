use std::path::{Path, PathBuf};

use xray_error::XRayResult;

use super::ExternManifestParser;
use crate::extern_manifest::{ExternExport, ExternParameter, ParsedExtern, ParsedExternManifest};

/// Parsed externs and the project they came from.
#[derive(Clone, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportsProject {
  pub root: PathBuf,
  pub declarations: Vec<ExportDescriptor>,
}

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

/// Projects canonical externs for the desktop editor.
#[derive(Default)]
pub struct ExportsEditorParser;

impl ExportsEditorParser {
  /// Create a parser that projects canonical externs for the desktop editor.
  pub fn new() -> Self {
    Self
  }

  /// Scan one project and project every extern for the editor.
  pub fn parse_project_from_path<P: AsRef<Path>>(&self, path: P) -> XRayResult<ExportsProject> {
    let root: &Path = path.as_ref();
    let parsed: ParsedExternManifest = ExternManifestParser::new().parse_directory(root)?;

    Ok(ExportsProject {
      root: root.to_path_buf(),
      declarations: self.project(parsed),
    })
  }

  fn project(&self, parsed: ParsedExternManifest) -> Vec<ExportDescriptor> {
    let mut result: Vec<ExportDescriptor> = parsed
      .parsed
      .into_iter()
      .map(|entry: ParsedExtern| {
        let source = ExportSourceDescriptor {
          path: entry.location.path,
          line: entry.location.line,
          column: entry.location.column,
        };

        match entry.export {
          ExternExport::Callable(callable) => {
            let (description, return_description) = callable
              .doc
              .map(|documentation| (documentation.description, documentation.returns))
              .unwrap_or_default();
            let parameters: Vec<ExportParameterDescriptor> = callable
              .params
              .into_iter()
              .map(|parameter: ExternParameter| ExportParameterDescriptor {
                name: parameter.name,
                typing: parameter.type_name,
                description: parameter.doc,
                is_optional: parameter.optional.unwrap_or(false),
              })
              .collect();

            ExportDescriptor {
              name: entry.name,
              description,
              source,
              contract: ExportContractDescriptor::Callable {
                parameters,
                returns: ExportReturnDescriptor {
                  typing: callable.returns,
                  description: return_description,
                },
              },
            }
          },
          ExternExport::Value(value) => ExportDescriptor {
            name: entry.name,
            description: value.doc.and_then(|documentation| documentation.description),
            source,
            contract: ExportContractDescriptor::Value {
              typing: value.type_name,
            },
          },
        }
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

  use serde_json::{Value, json};

  use super::{ExportContractDescriptor, ExportsEditorParser};

  fn create_test_root(name: &str) -> PathBuf {
    let root: PathBuf = std::env::temp_dir().join(format!("xray-export-editor-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    root
  }

  fn write_source(root: &Path, name: &str, source: &str) {
    let path: PathBuf = root.join(name);

    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, source).unwrap();
  }

  #[test]
  fn parses_all_editor_exports_from_the_project_root() {
    let root: PathBuf = create_test_root("project");

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

    let project = ExportsEditorParser::new().parse_project_from_path(&root).unwrap();

    assert_eq!(project.root, root);
    assert_eq!(project.declarations.len(), 5);
    assert_eq!(project.declarations[0].name, "dialogs_zaton.hello");
    assert_eq!(project.declarations[1].name, "settings");
    assert!(matches!(
      &project.declarations[1].contract,
      ExportContractDescriptor::Value { typing } if typing == "Record<string, boolean>"
    ));
    assert_eq!(project.declarations[2].name, "start");
    assert_eq!(project.declarations[3].name, "xr_conditions.check");
    assert_eq!(project.declarations[4].name, "xr_effects.run");

    fs::remove_dir_all(root).unwrap();
  }

  #[test]
  fn projects_complete_callable_and_value_contracts() {
    let root: PathBuf = create_test_root("contracts");

    write_source(
      &root,
      "declarations.ts",
      r#"
        export {};
        /**
         * Runs one task.
         * @param count - Optional repetition count.
         * @returns Whether the task ran.
         */
        extern("tasks.run", (count?: number): boolean => true);
        /** Current task settings. */
        extern("settings", rawValue as Record<string, boolean>);
      "#,
    );

    let project = ExportsEditorParser::new().parse_project_from_path(&root).unwrap();
    let json: Value = json!(project);

    assert_eq!(json["root"], root.to_string_lossy().as_ref());
    assert_eq!(json["declarations"][0]["kind"], "value");
    assert_eq!(json["declarations"][0]["typing"], "Record<string, boolean>");
    assert_eq!(json["declarations"][0]["description"], "Current task settings.");
    assert_eq!(json["declarations"][1]["kind"], "callable");
    assert_eq!(json["declarations"][1]["description"], "Runs one task.");
    assert_eq!(json["declarations"][1]["parameters"][0]["name"], "count");
    assert_eq!(json["declarations"][1]["parameters"][0]["typing"], "number");
    assert_eq!(json["declarations"][1]["parameters"][0]["isOptional"], true);
    assert_eq!(
      json["declarations"][1]["parameters"][0]["description"],
      "Optional repetition count."
    );
    assert_eq!(json["declarations"][1]["returns"]["typing"], "boolean");
    assert_eq!(json["declarations"][1]["returns"]["description"], "Whether the task ran.");
    assert_eq!(json["declarations"][1]["source"]["path"], "declarations.ts");
    assert!(json["declarations"][1]["source"]["line"].as_u64().unwrap() > 0);
    assert!(json["declarations"][1]["source"]["column"].as_u64().unwrap() > 0);

    fs::remove_dir_all(root).unwrap();
  }

  #[test]
  fn keeps_an_empty_project_open() {
    let root: PathBuf = create_test_root("empty");

    let project = ExportsEditorParser::new().parse_project_from_path(&root).unwrap();

    assert_eq!(project.root, root);
    assert!(project.declarations.is_empty());

    fs::remove_dir_all(root).unwrap();
  }
}
