use std::path::Path;

use xray_error::XRayResult;

use super::{
  ExportContractDescriptor, ExportDescriptor, ExportParameterDescriptor, ExportReturnDescriptor,
  ExportSourceDescriptor, ExportsProject,
};
use crate::extern_manifest::{ExternExport, ExternParameter, ParsedExtern, ParsedExternManifest};
use crate::extern_parser::ExternManifestParser;

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
          end_line: entry.location.end_line,
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
          }
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
