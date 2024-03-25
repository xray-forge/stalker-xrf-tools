use std::default::Default;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::ast::ast_utils::{
  get_expression_callee_name, get_expression_parameter_as_string_name,
  get_parameters_from_arrow_expression,
};
use crate::constants::{XR_CONDITIONS_PREFIX, XR_EFFECT_PREFIX, XR_EXTERN_EXPRESSION};
use crate::error::export_error::ExportError;
use crate::extern_descriptor::ExportDescriptor;

use walkdir::WalkDir;
extern crate swc_common;
extern crate swc_ecma_parser;
use crate::error::parse_error::ExportParseError;
use swc_common::comments::{Comments, SingleThreadedComments};
use swc_common::errors::DiagnosticBuilder;
use swc_common::sync::Lrc;
use swc_common::{
  errors::{ColorConfig, Handler},
  Loc, SourceFile, SourceMap,
};
use swc_ecma_ast::{Expr, ModuleItem, Program, Stmt};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

#[derive(Default)]
pub struct ExportsParser {}

impl ExportsParser {
  pub fn new() -> ExportsParser {
    ExportsParser {}
  }
}

impl ExportsParser {
  pub fn is_valid_ts_export_source_path(path: &Path) -> bool {
    if path.extension().is_some_and(|extension| extension == "ts") {
      !path.to_str().unwrap().ends_with(".test.ts")
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
  pub fn parse_conditions(
    &self,
    files: &Vec<PathBuf>,
  ) -> Result<Vec<ExportDescriptor>, ExportError> {
    self.parse_exports(files, Self::is_xr_conditions_literal)
  }

  pub fn parse_conditions_from_path(
    &self,
    path: &Path,
  ) -> Result<Vec<ExportDescriptor>, ExportError> {
    self.parse_conditions(&Self::read_exporting_sources_from_path(path)?)
  }

  pub fn parse_dialogs(&self, files: &Vec<PathBuf>) -> Result<Vec<ExportDescriptor>, ExportError> {
    self.parse_exports(files, |value| Some(value.into()))
  }

  pub fn parse_dialogs_from_path(&self, path: &Path) -> Result<Vec<ExportDescriptor>, ExportError> {
    self.parse_dialogs(&Self::read_exporting_sources_from_path(path)?)
  }

  pub fn parse_effects(&self, files: &Vec<PathBuf>) -> Result<Vec<ExportDescriptor>, ExportError> {
    self.parse_exports(files, Self::is_xr_effect_literal)
  }

  pub fn parse_effects_from_path(&self, path: &Path) -> Result<Vec<ExportDescriptor>, ExportError> {
    self.parse_effects(&Self::read_exporting_sources_from_path(path)?)
  }

  pub fn parse_exports(
    &self,
    files: &Vec<PathBuf>,
    filter: fn(&str) -> Option<String>,
  ) -> Result<Vec<ExportDescriptor>, ExportError> {
    let mut expressions: Vec<ExportDescriptor> = Vec::new();

    for path in files {
      log::info!("Parsing exports effects from: {:?}", path);

      let (program, source_map, comments) = self.open_ts_source_file(path)?;

      expressions.append(&mut self.parse_program_extern_declarations(
        &program,
        &source_map,
        &comments,
        filter,
      ));
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
  ) -> Vec<ExportDescriptor> {
    let mut expressions: Vec<ExportDescriptor> = Vec::new();

    if let Program::Module(module) = &program {
      for module_item in &module.body {
        if let ModuleItem::Stmt(Stmt::Expr(expression)) = module_item {
          // If it is call expression + extern:
          if let Expr::Call(call_expression) = expression.expr.as_ref() {
            if get_expression_callee_name(&call_expression.callee)
              .is_some_and(|x| x == XR_EXTERN_EXPRESSION)
              && call_expression.args.len() == 2
            {
              let name: Option<String> =
                get_expression_parameter_as_string_name(call_expression.args.first().unwrap());

              if let Some(effect_full_name) = name {
                if let Some(effect_name) = filter(&effect_full_name) {
                  let comment: Option<String> =
                    comments.get_leading(expression.span.lo).map(|it| {
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
                      call_expression.args.get(1).unwrap(),
                    ),
                  });
                }
              }
            }
          }
        }
      }
    }

    expressions
  }
}

impl ExportsParser {
  pub fn read_exporting_sources_from_path(path: &Path) -> Result<Vec<PathBuf>, ExportError> {
    let mut files: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(path)
      .into_iter()
      .filter_map(|entry| match entry {
        Ok(entry) => Some(entry),
        Err(_) => None,
      })
    {
      let path: &Path = entry.path();

      if Self::is_valid_ts_export_source_path(path) {
        files.push(path.into());
      }
    }

    Ok(files)
  }

  fn open_ts_source_file(
    &self,
    path: &Path,
  ) -> Result<(Program, Lrc<SourceMap>, Box<dyn Comments>), ExportError> {
    let source_map: Lrc<SourceMap> = Default::default();
    let handler: Handler =
      Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(source_map.clone()));
    let fm: Rc<SourceFile> = source_map
      .load_file(path)
      .expect("Failed to load source file");
    let comments: Box<SingleThreadedComments> = Box::default();

    let lexer: Lexer = Lexer::new(
      Syntax::Typescript(Default::default()),
      Default::default(),
      StringInput::from(fm.as_ref()),
      Some(&comments),
    );

    let mut parser: Parser<Lexer> = Parser::new_from(lexer);
    let mut diagnostics: Vec<DiagnosticBuilder> = parser
      .take_errors()
      .into_iter()
      .map(|it| it.into_diagnostic(&handler))
      .collect();

    for diagostic in diagnostics.iter_mut() {
      diagostic.emit();
    }

    if !diagnostics.is_empty() {
      return Err(ExportParseError::new_export_error(format!(
        "Failed to parse target files: {}",
        diagnostics
          .iter()
          .map(|builder| builder
            .message
            .iter()
            .map(|message| message.0.as_str())
            .collect::<Vec<_>>()
            .join(", "))
          .collect::<Vec<_>>()
          .join(", ")
      )));
    }

    let program: Program = parser
      .parse_program()
      .map_err(|error| error.into_diagnostic(&handler).emit())
      .expect("Failed to parse TS module");

    Ok((program, source_map, comments))
  }
}
