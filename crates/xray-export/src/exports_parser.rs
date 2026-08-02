use crate::constants::{XR_CONDITIONS_PREFIX, XR_EFFECT_PREFIX, XR_EXTERN_EXPRESSION};
use crate::export_parameters::get_parameters_from_arrow_expression;
use crate::extern_descriptor::ExportDescriptor;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use xray_error::XRayResult;
use xray_typescript::ast::{expression_callee_name, expression_string_argument};
use xray_typescript::parse_typescript_file;
use xray_typescript::swc_common::{Loc, SourceMap, comments::Comments};
use xray_typescript::swc_ecma_ast::{Expr, ModuleItem, Program, Stmt};

#[derive(Default)]
/// Parses X-Ray `extern` declarations from TypeScript source files.
pub struct ExportsParser {}

impl ExportsParser {
  pub fn new() -> Self {
    Self {}
  }
}

impl ExportsParser {
  pub fn is_valid_ts_export_source_path<P: AsRef<Path>>(path: P) -> bool {
    if path
      .as_ref()
      .extension()
      .is_some_and(|extension| extension == "ts")
    {
      !path.as_ref().to_str().unwrap().ends_with(".test.ts")
    } else {
      false
    }
  }

  pub fn is_xr_effect_literal(name: &str) -> Option<String> {
    name.strip_prefix(XR_EFFECT_PREFIX).map(|it| it.into())
  }

  pub fn is_xr_conditions_literal(name: &str) -> Option<String> {
    name.strip_prefix(XR_CONDITIONS_PREFIX).map(|it| it.into())
  }
}

impl ExportsParser {
  pub fn parse_conditions(&self, paths: &[PathBuf]) -> XRayResult<Vec<ExportDescriptor>> {
    self.parse_exports(paths, Self::is_xr_conditions_literal)
  }

  pub fn parse_conditions_from_path<P: AsRef<Path>>(
    &self,
    path: P,
  ) -> XRayResult<Vec<ExportDescriptor>> {
    self.parse_conditions(&Self::read_exporting_sources_from_path(path)?)
  }

  pub fn parse_dialogs(&self, files: &[PathBuf]) -> XRayResult<Vec<ExportDescriptor>> {
    self.parse_exports(files, |value| Some(value.into()))
  }

  pub fn parse_dialogs_from_path<P: AsRef<Path>>(
    &self,
    path: P,
  ) -> XRayResult<Vec<ExportDescriptor>> {
    self.parse_dialogs(&Self::read_exporting_sources_from_path(path)?)
  }

  pub fn parse_effects(&self, files: &[PathBuf]) -> XRayResult<Vec<ExportDescriptor>> {
    self.parse_exports(files, Self::is_xr_effect_literal)
  }

  pub fn parse_effects_from_path<P: AsRef<Path>>(
    &self,
    path: P,
  ) -> XRayResult<Vec<ExportDescriptor>> {
    self.parse_effects(&Self::read_exporting_sources_from_path(path)?)
  }

  pub fn parse_exports(
    &self,
    files: &[PathBuf],
    filter: fn(&str) -> Option<String>,
  ) -> XRayResult<Vec<ExportDescriptor>> {
    let mut expressions: Vec<ExportDescriptor> = Vec::new();

    for path in files {
      log::info!("Parsing exports from: {}", path.display());

      let source = parse_typescript_file(path)?;

      expressions.append(&mut self.parse_program_extern_declarations(
        &source.program,
        &source.source_map,
        &source.comments,
        filter,
      )?);
    }

    expressions.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(expressions)
  }

  fn parse_program_extern_declarations(
    &self,
    program: &Program,
    source_map: &SourceMap,
    comments: &dyn Comments,
    filter: fn(&str) -> Option<String>,
  ) -> XRayResult<Vec<ExportDescriptor>> {
    let mut expressions: Vec<ExportDescriptor> = Vec::new();

    if let Program::Module(module) = &program {
      for module_item in &module.body {
        if let ModuleItem::Stmt(Stmt::Expr(expression)) = module_item {
          // If it is call expression + extern:
          if let Expr::Call(call_expression) = expression.expr.as_ref()
            && expression_callee_name(&call_expression.callee)
              .is_some_and(|x| x == XR_EXTERN_EXPRESSION)
            && call_expression.args.len() == 2
          {
            let name: Option<String> =
              expression_string_argument(call_expression.args.first().unwrap());

            if let Some(effect_full_name) = name
              && let Some(effect_name) = filter(&effect_full_name)
            {
              let comment: Option<String> = comments.get_leading(expression.span.lo).map(|it| {
                it.iter()
                  .map(|comment| comment.text.as_str())
                  .collect::<Vec<_>>()
                  .join("\n")
              });

              let loc: Loc = source_map.lookup_char_pos(expression.span.lo);

              expressions.push(ExportDescriptor {
                col: loc.col.0,
                comment,
                filename: loc.file.name.to_string(),
                line: loc.line,
                name: effect_name,
                parameters: get_parameters_from_arrow_expression(
                  call_expression
                    .args
                    .get(1)
                    .expect("Expect1 index argument declaration"),
                )?,
              });
            }
          }
        }
      }
    }

    Ok(expressions)
  }
}

impl ExportsParser {
  pub fn read_exporting_sources_from_path<P: AsRef<Path>>(path: P) -> XRayResult<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
      let path: &Path = entry.path();

      if Self::is_valid_ts_export_source_path(path) {
        files.push(path.into());
      }
    }

    Ok(files)
  }
}
