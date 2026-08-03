use super::jsdoc_parser::JsDocParser;
use crate::extern_manifest::{
  ExternCallable, ExternDocumentation, ExternExport, ExternParameter, ExternSourceLocation,
  ExternValue, ParsedExtern,
};
use std::collections::BTreeMap;
use xray_error::{XRayError, XRayResult};
use xray_typescript::ast::{
  canonical_ts_type_to_string, expression_callee_name, expression_string_argument,
};
use xray_typescript::swc_common::{BytePos, SourceMap, SourceMapper, Spanned, comments::Comments};
use xray_typescript::swc_ecma_ast::{
  Expr, ModuleItem, Pat, Program, Prop, PropName, Stmt, TsFnOrConstructorType, TsType,
};

const EXTERN_EXPRESSION: &str = "extern";

/// Extracts the canonical extern declarations from one parsed TypeScript module.
///
/// Only top-level `extern(name, value)` expressions are considered. Dynamic
/// names and unsupported declaration shapes return location-aware errors.
pub struct ExternDeclarationParser<'a> {
  source_map: &'a SourceMap,
  jsdoc_parser: JsDocParser<'a>,
  source_path: &'a str,
}

impl<'a> ExternDeclarationParser<'a> {
  /// Create a parser whose diagnostics and documentation refer to one source file.
  pub fn new(source_map: &'a SourceMap, comments: &'a dyn Comments, source_path: &'a str) -> Self {
    Self {
      source_map,
      jsdoc_parser: JsDocParser::new(comments),
      source_path,
    }
  }

  /// Extract all supported top-level extern declarations from `program`.
  pub fn parse(&self, program: &Program) -> XRayResult<Vec<ParsedExtern>> {
    let source_map: &SourceMap = self.source_map;
    let source_path: &str = self.source_path;
    let Program::Module(module) = program else {
      return Ok(Vec::new());
    };
    let mut declarations: Vec<ParsedExtern> = Vec::new();

    for item in &module.body {
      let ModuleItem::Stmt(Stmt::Expr(statement)) = item else {
        continue;
      };
      let Expr::Call(call) = statement.expr.as_ref() else {
        continue;
      };
      if expression_callee_name(&call.callee).as_deref() != Some(EXTERN_EXPRESSION) {
        continue;
      }

      if call.args.len() != 2 {
        return Err(invalid_at(
          source_map,
          statement.span.lo,
          source_path,
          "expected a literal name and exactly one exported value",
        ));
      }
      let name: String = expression_string_argument(&call.args[0]).ok_or_else(|| {
        invalid_at(
          source_map,
          call.args[0].expr.span().lo,
          source_path,
          "extern names must be string literals",
        )
      })?;

      let documentation: Option<ExternDocumentation> = self.jsdoc_parser.parse(statement.span.lo);
      let parameter_docs: BTreeMap<String, String> =
        self.jsdoc_parser.parameter_docs(statement.span.lo);
      let location: ExternSourceLocation =
        source_location(source_map, statement.span.lo, source_path);

      match call.args[1].expr.as_ref() {
        Expr::Arrow(arrow) => declarations.push(ParsedExtern {
          export: ExternExport::Callable(parse_arrow(
            arrow,
            documentation,
            &parameter_docs,
            source_map,
            source_path,
          )?),
          location,
          name,
        }),
        Expr::Object(object) => {
          for property in &object.props {
            let Prop::KeyValue(property) = property
              .as_prop()
              .ok_or_else(|| {
                invalid_at(
                  source_map,
                  property.span().lo,
                  source_path,
                  "object externs must contain property assignments",
                )
              })?
              .as_ref()
            else {
              return Err(invalid_at(
                source_map,
                property.span().lo,
                source_path,
                "object externs must contain property assignments",
              ));
            };
            let property_name: String = property_name(&property.key).ok_or_else(|| {
              invalid_at(
                source_map,
                property.key.span().lo,
                source_path,
                "object extern property names must be identifiers or string literals",
              )
            })?;
            let property_doc: Option<ExternDocumentation> = self
              .jsdoc_parser
              .parse(property.span().lo)
              .or_else(|| documentation.clone());
            let own_parameter_docs: BTreeMap<String, String> =
              self.jsdoc_parser.parameter_docs(property.span().lo);
            let property_parameter_docs: &BTreeMap<String, String> =
              if own_parameter_docs.is_empty() {
                &parameter_docs
              } else {
                &own_parameter_docs
              };
            let property_location: ExternSourceLocation =
              source_location(source_map, property.span().lo, source_path);
            let export: ExternExport = parse_export_value(
              property.value.as_ref(),
              property_doc,
              property_parameter_docs,
              source_map,
              source_path,
            )?;

            declarations.push(ParsedExtern {
              export,
              location: property_location,
              name: format!("{name}.{property_name}"),
            });
          }
        }
        value => declarations.push(ParsedExtern {
          export: parse_export_value(
            value,
            documentation,
            &parameter_docs,
            source_map,
            source_path,
          )?,
          location,
          name,
        }),
      }
    }

    Ok(declarations)
  }
}

fn parse_export_value(
  value: &Expr,
  documentation: Option<ExternDocumentation>,
  parameter_docs: &BTreeMap<String, String>,
  source_map: &SourceMap,
  source_path: &str,
) -> XRayResult<ExternExport> {
  match value {
    Expr::Paren(parenthesized) => parse_export_value(
      parenthesized.expr.as_ref(),
      documentation,
      parameter_docs,
      source_map,
      source_path,
    ),
    Expr::Arrow(arrow) => Ok(ExternExport::Callable(parse_arrow(
      arrow,
      documentation,
      parameter_docs,
      source_map,
      source_path,
    )?)),
    Expr::TsAs(assertion) => {
      if let TsType::TsFnOrConstructorType(TsFnOrConstructorType::TsFnType(function_type)) =
        assertion.type_ann.as_ref()
      {
        return Ok(ExternExport::Callable(parse_function_type(
          function_type,
          documentation,
          parameter_docs,
          source_map,
          source_path,
        )?));
      }

      Ok(ExternExport::Value(ExternValue {
        doc: documentation,
        source: source_path.into(),
        type_name: canonical_type(assertion.type_ann.as_ref(), source_map, source_path)?,
      }))
    }
    _ => Err(invalid_at(
      source_map,
      value.span().lo,
      source_path,
      "extern values must be typed arrow functions or `value as Type` assertions",
    )),
  }
}

fn parse_arrow(
  arrow: &xray_typescript::swc_ecma_ast::ArrowExpr,
  documentation: Option<ExternDocumentation>,
  parameter_docs: &BTreeMap<String, String>,
  source_map: &SourceMap,
  source_path: &str,
) -> XRayResult<ExternCallable> {
  let returns: String = arrow.return_type.as_ref().map_or_else(
    || {
      Err(invalid_at(
        source_map,
        arrow.span.lo,
        source_path,
        "callable externs require an explicit return type annotation",
      ))
    },
    |annotation| canonical_type(annotation.type_ann.as_ref(), source_map, source_path),
  )?;
  let mut params: Vec<ExternParameter> = Vec::with_capacity(arrow.params.len());

  for parameter in &arrow.params {
    params.push(parse_parameter(
      parameter,
      parameter_docs,
      source_map,
      source_path,
    )?);
  }

  Ok(ExternCallable {
    doc: documentation,
    params,
    returns,
    source: source_path.into(),
  })
}

fn parse_function_type(
  function_type: &xray_typescript::swc_ecma_ast::TsFnType,
  documentation: Option<ExternDocumentation>,
  parameter_docs: &BTreeMap<String, String>,
  source_map: &SourceMap,
  source_path: &str,
) -> XRayResult<ExternCallable> {
  let mut params: Vec<ExternParameter> = Vec::with_capacity(function_type.params.len());

  for parameter in &function_type.params {
    let (name, annotation, optional): (
      String,
      Option<&xray_typescript::swc_ecma_ast::TsTypeAnn>,
      bool,
    ) = match parameter {
      xray_typescript::swc_ecma_ast::TsFnParam::Ident(binding) => (
        binding.id.sym.to_string(),
        binding.type_ann.as_deref(),
        binding.id.optional,
      ),
      _ => {
        return Err(invalid_at(
          source_map,
          function_type.span.lo,
          source_path,
          "function type assertions must use named parameters",
        ));
      }
    };
    let type_name: String = annotation.map_or_else(
      || {
        Err(invalid_at(
          source_map,
          function_type.span.lo,
          source_path,
          "function type assertion parameters require explicit type annotations",
        ))
      },
      |value| canonical_type(value.type_ann.as_ref(), source_map, source_path),
    )?;
    params.push(ExternParameter {
      doc: parameter_docs.get(&name).cloned(),
      name,
      optional: optional.then_some(true),
      type_name,
    });
  }

  Ok(ExternCallable {
    doc: documentation,
    params,
    returns: canonical_type(
      function_type.type_ann.type_ann.as_ref(),
      source_map,
      source_path,
    )?,
    source: source_path.into(),
  })
}

fn parse_parameter(
  parameter: &Pat,
  parameter_docs: &BTreeMap<String, String>,
  source_map: &SourceMap,
  source_path: &str,
) -> XRayResult<ExternParameter> {
  let (name, annotation, optional): (
    String,
    Option<&xray_typescript::swc_ecma_ast::TsTypeAnn>,
    bool,
  ) = match parameter {
    Pat::Ident(binding) => (
      binding.id.sym.to_string(),
      binding.type_ann.as_deref(),
      binding.id.optional,
    ),
    Pat::Array(pattern) => (
      pattern_name(source_map, pattern.span, "[parameter]"),
      pattern.type_ann.as_deref(),
      pattern.optional,
    ),
    Pat::Rest(pattern) => {
      let Pat::Ident(binding) = pattern.arg.as_ref() else {
        return Err(invalid_at(
          source_map,
          parameter.span().lo,
          source_path,
          "rest parameters must use explicitly typed identifiers",
        ));
      };
      (
        binding.id.sym.to_string(),
        pattern.type_ann.as_deref().or(binding.type_ann.as_deref()),
        false,
      )
    }
    Pat::Assign(pattern) => match pattern.left.as_ref() {
      Pat::Ident(binding) => (
        binding.id.sym.to_string(),
        binding.type_ann.as_deref(),
        true,
      ),
      Pat::Array(binding) => (
        pattern_name(source_map, binding.span, "[parameter]"),
        binding.type_ann.as_deref(),
        true,
      ),
      _ => {
        return Err(invalid_at(
          source_map,
          parameter.span().lo,
          source_path,
          "callable extern parameters with defaults must use explicitly typed identifiers or array patterns",
        ));
      }
    },
    _ => {
      return Err(invalid_at(
        source_map,
        parameter.span().lo,
        source_path,
        "callable extern parameters must use explicitly typed identifiers or array patterns",
      ));
    }
  };
  let type_name: String = annotation.map_or_else(
    || {
      Err(invalid_at(
        source_map,
        parameter.span().lo,
        source_path,
        "callable extern parameters require explicit type annotations",
      ))
    },
    |value| canonical_type(value.type_ann.as_ref(), source_map, source_path),
  )?;

  Ok(ExternParameter {
    doc: parameter_docs.get(&name).cloned(),
    name,
    optional: optional.then_some(true),
    type_name,
  })
}

fn pattern_name(
  source_map: &SourceMap,
  span: xray_typescript::swc_common::Span,
  fallback: &str,
) -> String {
  let snippet: String = source_map
    .span_to_snippet(span)
    .unwrap_or_else(|_| fallback.into());
  let mut nesting: usize = 0;

  for (index, character) in snippet.char_indices() {
    match character {
      '[' | '(' | '{' => nesting += 1,
      ']' | ')' | '}' => nesting = nesting.saturating_sub(1),
      ':' if nesting == 0 => return snippet[..index].trim_end().into(),
      _ => {}
    }
  }

  snippet.trim_end().into()
}

fn canonical_type(
  type_annotation: &TsType,
  source_map: &SourceMap,
  source_path: &str,
) -> XRayResult<String> {
  canonical_ts_type_to_string(type_annotation, source_map).map_err(|_| {
    invalid_at(
      source_map,
      type_annotation.span().lo,
      source_path,
      "unsupported TypeScript type annotation",
    )
  })
}

fn property_name(name: &PropName) -> Option<String> {
  match name {
    PropName::Ident(value) => Some(value.sym.to_string()),
    PropName::Str(value) => Some(value.value.to_string_lossy().to_string()),
    _ => None,
  }
}

fn source_location(
  source_map: &SourceMap,
  position: BytePos,
  source_path: &str,
) -> ExternSourceLocation {
  let location = source_map.lookup_char_pos(position);

  ExternSourceLocation {
    column: location.col.0 + 1,
    line: location.line,
    path: source_path.into(),
  }
}

fn invalid_at(
  source_map: &SourceMap,
  position: BytePos,
  source_path: &str,
  reason: &str,
) -> XRayError {
  let location: ExternSourceLocation = source_location(source_map, position, source_path);

  XRayError::new_invalid_error(format!(
    "Unsupported extern declaration at {}:{}:{}, {}.",
    location.path, location.line, location.column, reason,
  ))
}
