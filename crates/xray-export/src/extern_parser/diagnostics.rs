use xray_error::XRayError;
use xray_typescript::swc_common::{BytePos, SourceMap, SourceMapper, Spanned};
use xray_typescript::swc_ecma_ast::Expr;

use crate::extern_manifest::ExternSourceLocation;

/// Return an extern declaration source location.
pub fn source_location(source_map: &SourceMap, position: BytePos, source_path: &str) -> ExternSourceLocation {
  let location = source_map.lookup_char_pos(position);

  ExternSourceLocation {
    column: location.col.0 + 1,
    line: location.line,
    path: source_path.into(),
  }
}

/// Create a location-aware invalid extern declaration error.
pub fn invalid_at(source_map: &SourceMap, position: BytePos, source_path: &str, reason: &str) -> XRayError {
  let location: ExternSourceLocation = source_location(source_map, position, source_path);

  XRayError::new_invalid_error(format!(
    "Invalid extern declaration at {}:{}:{}: {}.",
    location.path, location.line, location.column, reason,
  ))
}

/// Explain why an expression cannot provide a stable extern contract.
pub fn unsupported_export_value_reason(value: &Expr, export_name: &str, source_map: &SourceMap) -> String {
  let expression: String = source_map
    .span_to_snippet(value.span())
    .unwrap_or_else(|_| String::from("<unavailable>"));

  match value {
    Expr::Ident(identifier) => format!(
      "function reference `{}` for extern '{export_name}' needs a type; write `{expression} as (arg: Type) => ReturnType` or wrap it in an arrow function",
      identifier.sym,
    ),
    _ => format!(
      "extern '{export_name}' uses unsupported expression `{expression}`; use an arrow function or a `value as Type` assertion",
    ),
  }
}
