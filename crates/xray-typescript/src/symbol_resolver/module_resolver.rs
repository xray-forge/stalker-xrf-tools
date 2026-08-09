use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use xray_error::XRayResult;

use super::TypeScriptSymbol;
use super::declaration_parser::{exported_symbol, local_symbol};
use super::typescript_project::TypeScriptProject;
use crate::parse_typescript_file;
use crate::swc_common::SourceMap;
use crate::swc_ecma_ast::{ImportSpecifier, ModuleDecl, ModuleExportName, ModuleItem, Program};

/// Resolves TypeScript function references within one project.
///
/// The resolver follows local declarations, named imports, configured aliases,
/// and named or wildcard re-exports.
pub struct TypeScriptSymbolResolver {
  project: TypeScriptProject,
}

impl TypeScriptSymbolResolver {
  /// Create a resolver by discovering the nearest TypeScript project configuration.
  pub fn discover(source_root: &Path) -> XRayResult<Self> {
    Ok(Self {
      project: TypeScriptProject::discover(source_root)?,
    })
  }

  /// Resolve a local or imported source symbol used by `local_name`.
  ///
  /// Returns `None` when the identifier cannot be resolved to a declared
  /// callable or explicitly typed value.
  pub fn resolve_symbol(
    &self,
    source_file: &Path,
    source_map: &SourceMap,
    program: &Program,
    local_name: &str,
  ) -> XRayResult<Option<TypeScriptSymbol>> {
    if let Some(symbol) = local_symbol(program, local_name, source_map) {
      return Ok(Some(symbol));
    }
    let Some((module_specifier, export_name)) = imported_function_reference(program, local_name)
    else {
      return Ok(None);
    };
    let Some(module_path) = self
      .project
      .resolve_module_path(source_file, &module_specifier)
    else {
      return Ok(None);
    };
    let mut visited: BTreeSet<PathBuf> = BTreeSet::new();

    self.resolve_exported_symbol(&module_path, &export_name, &mut visited)
  }

  /// Resolve the type of a named property on a referenced object literal.
  pub fn resolve_member_type(
    &self,
    source_file: &Path,
    source_map: &SourceMap,
    program: &Program,
    object_name: &str,
    property_name: &str,
  ) -> XRayResult<Option<String>> {
    let Some(symbol) = self.resolve_symbol(source_file, source_map, program, object_name)? else {
      return Ok(None);
    };

    Ok(symbol.property_type(property_name))
  }

  /// Resolve one exported symbol through declarations and re-exports.
  fn resolve_exported_symbol(
    &self,
    module_path: &Path,
    export_name: &str,
    visited: &mut BTreeSet<PathBuf>,
  ) -> XRayResult<Option<TypeScriptSymbol>> {
    let normalized: PathBuf = module_path.to_path_buf();

    if !visited.insert(normalized.clone()) {
      return Ok(None);
    }

    let source = parse_typescript_file(&normalized)?;
    let Program::Module(module) = &source.program else {
      return Ok(None);
    };

    for item in &module.body {
      let ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(declaration)) = item else {
        continue;
      };
      if let Some(symbol) =
        exported_symbol(&declaration.decl, export_name, source.source_map.as_ref())
      {
        return Ok(Some(symbol));
      }
    }

    for item in &module.body {
      let ModuleItem::ModuleDecl(declaration) = item else {
        continue;
      };

      match declaration {
        ModuleDecl::ExportAll(export_all) => {
          let specifier: String = export_all.src.value.to_string_lossy().to_string();
          let Some(path) = self.project.resolve_module_path(&normalized, &specifier) else {
            continue;
          };
          if let Some(symbol) = self.resolve_exported_symbol(&path, export_name, visited)? {
            return Ok(Some(symbol));
          }
        }
        ModuleDecl::ExportNamed(named) => {
          let Some(source_path) = named.src.as_ref() else {
            continue;
          };
          let Some(original_name) = reexported_name(named, export_name) else {
            continue;
          };
          let specifier: String = source_path.value.to_string_lossy().to_string();
          let Some(path) = self.project.resolve_module_path(&normalized, &specifier) else {
            continue;
          };
          if let Some(symbol) = self.resolve_exported_symbol(&path, &original_name, visited)? {
            return Ok(Some(symbol));
          }
        }
        _ => {}
      }
    }

    Ok(None)
  }
}

/// Return the imported module and exported name for one local named import.
fn imported_function_reference(program: &Program, local_name: &str) -> Option<(String, String)> {
  let Program::Module(module) = program else {
    return None;
  };

  for item in &module.body {
    let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = item else {
      continue;
    };

    for specifier in &import.specifiers {
      let ImportSpecifier::Named(named) = specifier else {
        continue;
      };
      if named.is_type_only || named.local.sym != *local_name {
        continue;
      }
      let export_name: String = named
        .imported
        .as_ref()
        .map(module_export_name)
        .unwrap_or_else(|| local_name.into());

      return Some((import.src.value.to_string_lossy().to_string(), export_name));
    }
  }

  None
}

/// Return the original export name when a named re-export exposes `export_name`.
fn reexported_name(named: &crate::swc_ecma_ast::NamedExport, export_name: &str) -> Option<String> {
  named.specifiers.iter().find_map(|specifier| {
    let crate::swc_ecma_ast::ExportSpecifier::Named(specifier) = specifier else {
      return None;
    };
    if specifier.is_type_only {
      return None;
    }
    let exposed_name: String = specifier
      .exported
      .as_ref()
      .map(module_export_name)
      .unwrap_or_else(|| module_export_name(&specifier.orig));

    (exposed_name == export_name).then(|| module_export_name(&specifier.orig))
  })
}

/// Return the text form of an AST module export name.
fn module_export_name(name: &ModuleExportName) -> String {
  name.atom().to_string()
}
