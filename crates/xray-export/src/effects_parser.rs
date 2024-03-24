use std::default::Default;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::ast::ast_utils::{
  get_expression_callee_name, get_expression_parameter_as_string_name,
  get_parameters_from_arrow_expression,
};
use crate::constants::{XR_EFFECT_PREFIX, XR_EXTERN_EXPRESSION};
use crate::error::export_error::ExportError;
use crate::extern_descriptor::ExternDescriptor;

use walkdir::WalkDir;
extern crate swc_common;
extern crate swc_ecma_parser;
use crate::error::parse_error::ExportParseError;
use swc_common::errors::DiagnosticBuilder;
use swc_common::sync::Lrc;
use swc_common::{
  errors::{ColorConfig, Handler},
  SourceFile, SourceMap,
};
use swc_ecma_ast::{Expr, ModuleItem, Program, Stmt};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

pub struct EffectsParser {
  pub files: Vec<PathBuf>,
}

impl EffectsParser {
  pub fn new(path: &Path) -> Result<EffectsParser, ExportError> {
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

    Ok(EffectsParser { files })
  }
}

impl EffectsParser {
  pub fn is_valid_ts_export_source_path(path: &Path) -> bool {
    if path.extension().is_some_and(|extension| extension == "ts") {
      !path.to_str().unwrap().ends_with(".test.ts")
    } else {
      false
    }
  }

  pub fn is_xr_effect_literal(name: &str) -> bool {
    name.starts_with(XR_EFFECT_PREFIX)
  }
}

impl EffectsParser {
  pub fn parse_effects(&self) -> Result<Vec<ExternDescriptor>, ExportError> {
    let mut expressions: Vec<ExternDescriptor> = Vec::new();

    for path in &self.files {
      log::info!("Parsing exports effects from: {:?}", path);

      let cm: Lrc<SourceMap> = Default::default();
      let handler: Handler =
        Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
      let fm: Rc<SourceFile> = cm.load_file(path).expect("Failed to load source file");

      let lexer = Lexer::new(
        Syntax::Typescript(Default::default()),
        Default::default(),
        StringInput::from(fm.as_ref()),
        None,
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

      expressions.append(&mut self.parse_program_extern_declarations(&program));
    }

    Ok(expressions)
  }

  fn parse_program_extern_declarations(&self, program: &Program) -> Vec<ExternDescriptor> {
    let mut expressions: Vec<ExternDescriptor> = Vec::new();

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

              if let Some(name) = name {
                if Self::is_xr_effect_literal(&name) {
                  expressions.push(ExternDescriptor {
                    name: name[XR_EFFECT_PREFIX.len()..].into(),
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
