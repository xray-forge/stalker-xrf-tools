mod declaration_parser;
mod editor_projection;
mod jsdoc_parser;

pub use editor_projection::{ExportDescriptor, ExportParameterDescriptor, ExportsEditorParser};

use crate::extern_manifest::{ExternExport, ExternManifest, ParsedExternManifest};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use xray_error::{XRayError, XRayResult};
use xray_typescript::parse_typescript_file;

/// Parses TypeScript extern declarations into the canonical manifest model.
///
/// Source paths in the manifest are always relative to the declarations root.
#[derive(Default)]
pub struct ExternManifestParser;

impl ExternManifestParser {
  /// Create an extern manifest parser.
  pub fn new() -> Self {
    Self
  }

  /// Scan `declarations_root` and parse every eligible TypeScript declaration.
  pub fn parse_directory(&self, declarations_root: &Path) -> XRayResult<ParsedExternManifest> {
    let files: Vec<PathBuf> = self.read_source_files(declarations_root);

    self.parse_files(&files, declarations_root)
  }

  fn parse_files(
    &self,
    files: &[PathBuf],
    declarations_root: &Path,
  ) -> XRayResult<ParsedExternManifest> {
    let mut parsed = Vec::new();

    for path in files {
      let source = parse_typescript_file(path)?;
      let source_path: String = self.normalize_declaration_path(path, declarations_root)?;
      let mut declarations = declaration_parser::ExternDeclarationParser::new(
        &source.source_map,
        &source.comments,
        &source_path,
      )
      .parse(&source.program)?;

      parsed.append(&mut declarations);
    }

    parsed.sort_by(|left, right| left.name.cmp(&right.name));

    let mut exports: BTreeMap<String, ExternExport> = BTreeMap::new();

    for declaration in &parsed {
      if let Some(existing) = exports.insert(declaration.name.clone(), declaration.export.clone()) {
        return Err(XRayError::new_invalid_error(format!(
          "Duplicate extern '{}' declared in '{}' and '{}'.",
          declaration.name,
          existing.source(),
          declaration.export.source(),
        )));
      }
    }

    Ok(ParsedExternManifest {
      manifest: ExternManifest { exports },
      parsed,
    })
  }

  /// Return whether a TypeScript source can contribute an extern declaration.
  pub fn is_source_path(path: &Path) -> bool {
    path.extension().is_some_and(|extension| extension == "ts")
      && !path.file_name().is_some_and(|name| {
        name.to_string_lossy().ends_with(".test.ts") || name.to_string_lossy().ends_with(".spec.ts")
      })
      && !path
        .components()
        .any(|component| component.as_os_str() == "__test__")
  }

  fn read_source_files(&self, declarations_root: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = WalkDir::new(declarations_root)
      .into_iter()
      .filter_map(Result::ok)
      .map(|entry| entry.into_path())
      .filter(|path| Self::is_source_path(path))
      .collect();

    files.sort();

    files
  }

  fn normalize_declaration_path(
    &self,
    path: &Path,
    declarations_root: &Path,
  ) -> XRayResult<String> {
    let relative: &Path = path.strip_prefix(declarations_root).map_err(|_| {
      XRayError::new_invalid_error(format!(
        "Declaration '{}' is outside declarations root '{}'.",
        path.display(),
        declarations_root.display(),
      ))
    })?;

    Ok(relative.to_string_lossy().replace('\\', "/"))
  }
}

#[cfg(test)]
mod tests {
  use super::ExternManifestParser;
  use crate::ExternExport;
  use std::fs;
  use std::path::{Path, PathBuf};

  fn create_test_root(name: &str) -> PathBuf {
    let root: PathBuf =
      std::env::temp_dir().join(format!("xray-export-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();
    root
  }

  fn write_source(root: &Path, name: &str, source: &str) {
    fs::write(root.join(name), source).unwrap();
  }

  #[test]
  fn parses_direct_object_and_asserted_externs_with_docs() {
    let root: PathBuf = create_test_root("manifest");

    write_source(
      &root,
      "externs.ts",
      r#"
        export {};
        /**
         * Shared callbacks.
         * @param id - Callback identifier.
         * @returns Callback result.
         */
        extern("callbacks", {
          run: (id: TId): boolean => true,
        });

        /** Numeric data. */
        extern("data.value", rawValue as { readonly id: string });
        extern("data.checkers", rawValue as Record<EAchievement, () => boolean>);
      "#,
    );

    let parsed = ExternManifestParser::new().parse_directory(&root).unwrap();
    let callback = parsed.manifest.exports.get("callbacks.run").unwrap();
    let ExternExport::Callable(callback) = callback else {
      panic!("Expected callable extern");
    };

    assert_eq!(callback.params[0].name, "id");
    assert_eq!(callback.params[0].type_name, "TId");
    assert_eq!(callback.source, "externs.ts");
    assert_eq!(
      callback.params[0].doc.as_deref(),
      Some("Callback identifier.")
    );
    assert_eq!(callback.returns, "boolean");
    assert_eq!(
      callback
        .doc
        .as_ref()
        .and_then(|documentation| documentation.returns.as_deref()),
      Some("Callback result.")
    );
    assert!(matches!(
      parsed.manifest.exports.get("data.value"),
      Some(ExternExport::Value(_))
    ));
    let ExternExport::Value(value) = parsed.manifest.exports.get("data.value").unwrap() else {
      panic!("Expected value extern");
    };
    assert_eq!(value.type_name, "{ readonly id: string }");

    let ExternExport::Value(checkers) = parsed.manifest.exports.get("data.checkers").unwrap()
    else {
      panic!("Expected value extern");
    };

    assert_eq!(checkers.type_name, "Record<EAchievement, () => boolean>");

    fs::remove_dir_all(root).unwrap();
  }

  #[test]
  fn rejects_missing_callable_contracts_and_duplicate_names() {
    let root: PathBuf = create_test_root("invalid");
    write_source(
      &root,
      "missing.ts",
      "export {}; extern(\"test\", () => true);",
    );

    let parser = ExternManifestParser::new();
    assert!(
      parser
        .parse_directory(&root)
        .unwrap_err()
        .to_string()
        .contains("explicit return type")
    );

    write_source(
      &root,
      "missing.ts",
      "export {}; extern(\"test\", (): boolean => true); extern(\"test\", (): boolean => false);",
    );

    assert!(
      parser
        .parse_directory(&root)
        .unwrap_err()
        .to_string()
        .contains("Duplicate extern")
    );

    fs::remove_dir_all(root).unwrap();
  }

  #[test]
  fn excludes_test_sources() {
    assert!(!ExternManifestParser::is_source_path(Path::new(
      "declarations/example.test.ts"
    )));
    assert!(!ExternManifestParser::is_source_path(Path::new(
      "declarations/example.spec.ts"
    )));
    assert!(!ExternManifestParser::is_source_path(Path::new(
      "declarations/__test__/example.ts"
    )));
    assert!(ExternManifestParser::is_source_path(Path::new(
      "declarations/example.ts"
    )));
  }
}
