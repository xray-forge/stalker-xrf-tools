use xray_typescript::ast::canonical_ts_type_to_string;
use xray_typescript::swc_common::SourceMap;
use xray_typescript::swc_ecma_ast::TsType;

/// Render a type annotation without failing the extern parser for unsupported syntax.
pub fn canonical_type(type_annotation: &TsType, source_map: &SourceMap) -> String {
  canonical_ts_type_to_string(type_annotation, source_map).unwrap_or_else(|_| String::from("unknown"))
}
