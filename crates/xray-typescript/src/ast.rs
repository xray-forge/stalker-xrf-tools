use swc_common::{SourceMap, SourceMapper, Spanned};
use swc_ecma_ast::{
  Callee, Expr, ExprOrSpread, Lit, TsArrayType, TsEntityName, TsFnOrConstructorType, TsFnParam, TsFnType, TsImportType,
  TsKeywordType, TsKeywordTypeKind, TsLit, TsLitType, TsType, TsTypeOperator, TsTypeOperatorOp,
  TsTypeParamInstantiation, TsTypeQuery, TsTypeQueryExpr, TsTypeRef, TsUnionOrIntersectionType,
};
use xray_error::{XRayError, XRayResult};

/// Return the unqualified identifier called by a call expression, if present.
pub fn expression_callee_name(callee: &Callee) -> Option<String> {
  if let Callee::Expr(callee_expression) = callee
    && let Expr::Ident(identifier) = callee_expression.as_ref()
  {
    return Some(identifier.sym.to_string());
  }

  None
}

/// Return an expression argument when it is a string literal.
pub fn expression_string_argument(expression: &ExprOrSpread) -> Option<String> {
  if let Expr::Lit(Lit::Str(string_literal)) = expression.expr.as_ref() {
    return Some(string_literal.value.to_string_lossy().to_string());
  }

  None
}

/// Render supported TypeScript type nodes as TypeScript-like text.
///
/// Unsupported nodes are rendered as `unsupported` and logged as warnings.
pub fn ts_type_to_string(ts_type: &TsType) -> String {
  match ts_type {
    TsType::TsKeywordType(keyword_type) => ts_keyword_type_to_string(keyword_type),
    TsType::TsTypeRef(type_ref) => ts_type_ref_to_string(type_ref),
    TsType::TsArrayType(array_type) => ts_array_type_to_string(array_type),
    TsType::TsUnionOrIntersectionType(union_type) => ts_union_or_intersection_to_string(union_type),
    TsType::TsLitType(literal_type) => ts_literal_type_to_string(literal_type),
    TsType::TsTypeOperator(type_operator) => ts_type_operator_to_string(type_operator),
    TsType::TsTypeQuery(type_query) => ts_type_query_to_string(type_query),
    TsType::TsFnOrConstructorType(TsFnOrConstructorType::TsFnType(function_type)) => {
      ts_function_type_to_string(function_type)
    }
    TsType::TsTupleType(tuple_type) => format!(
      "[{}]",
      tuple_type
        .elem_types
        .iter()
        .map(|element| ts_type_to_string(&element.ty))
        .collect::<Vec<String>>()
        .join(", ")
    ),
    TsType::TsOptionalType(optional_type) => {
      format!("{}?", ts_type_to_string(&optional_type.type_ann))
    }
    TsType::TsRestType(rest_type) => format!("...{}", ts_type_to_string(&rest_type.type_ann)),
    TsType::TsParenthesizedType(parenthesized_type) => {
      format!("({})", ts_type_to_string(&parenthesized_type.type_ann))
    }
    TsType::TsImportType(import_type) => ts_import_type_to_string(import_type),
    other => {
      log::warn!("Parsed unsupported TypeScript type: {:?}", other);
      String::from("unsupported")
    }
  }
}

/// Render a TypeScript type as stable TypeScript-like text.
///
/// The AST renderer handles the supported type nodes directly. For complex
/// source-dependent nodes, such as type literals, this uses their source
/// snippet and normalizes whitespace instead of losing information.
pub fn canonical_ts_type_to_string(ts_type: &TsType, source_map: &SourceMap) -> XRayResult<String> {
  if matches!(ts_type, TsType::TsTypeLit(_) | TsType::TsImportType(_)) {
    return canonical_source_type(ts_type, source_map);
  }
  let value: String = ts_type_to_string(ts_type);

  if value.contains("unsupported") {
    return canonical_source_type(ts_type, source_map);
  }

  Ok(value)
}

fn ts_function_type_to_string(function_type: &TsFnType) -> String {
  let params: String = function_type
    .params
    .iter()
    .map(|parameter| match parameter {
      TsFnParam::Ident(binding) => {
        let type_name: String = binding
          .type_ann
          .as_ref()
          .map(|annotation| ts_type_to_string(&annotation.type_ann))
          .unwrap_or_else(|| String::from("unsupported"));
        let optional: &str = if binding.id.optional { "?" } else { "" };

        format!("{}{}: {type_name}", binding.id.sym, optional)
      }
      _ => String::from("unsupported"),
    })
    .collect::<Vec<String>>()
    .join(", ");

  format!("({params}) => {}", ts_type_to_string(&function_type.type_ann.type_ann))
}

fn canonical_source_type(ts_type: &TsType, source_map: &SourceMap) -> XRayResult<String> {
  source_map
    .span_to_snippet(ts_type.span())
    .map(|value| value.split_whitespace().collect::<Vec<&str>>().join(" "))
    .map_err(|_| XRayError::new_invalid_error("Failed to render TypeScript type annotation."))
}

pub fn ts_literal_type_to_string(literal_type: &TsLitType) -> String {
  match &literal_type.lit {
    TsLit::Str(value) => format!("\"{}\"", value.value.to_string_lossy()),
    TsLit::Bool(value) => value.value.to_string(),
    TsLit::Number(value) => value.value.to_string(),
    TsLit::BigInt(value) => format!("{}n", value.value),
    _ => String::from("unsupported"),
  }
}

fn ts_type_operator_to_string(type_operator: &TsTypeOperator) -> String {
  match type_operator.op {
    TsTypeOperatorOp::KeyOf => format!("keyof {}", ts_type_to_string(&type_operator.type_ann)),
    TsTypeOperatorOp::Unique => format!("unique {}", ts_type_to_string(&type_operator.type_ann)),
    TsTypeOperatorOp::ReadOnly => {
      format!("readonly {}", ts_type_to_string(&type_operator.type_ann))
    }
  }
}

pub fn ts_type_query_to_string(type_query: &TsTypeQuery) -> String {
  format!("typeof {}", ts_entity_query_to_string(&type_query.expr_name))
}

pub fn ts_entity_query_to_string(name: &TsTypeQueryExpr) -> String {
  match name {
    TsTypeQueryExpr::TsEntityName(entity_name) => ts_entity_name_to_string(entity_name),
    TsTypeQueryExpr::Import(import_type) => ts_import_type_to_string(import_type),
  }
}

pub fn ts_import_type_to_string(import_type: &TsImportType) -> String {
  let qualifier: String = import_type
    .qualifier
    .as_ref()
    .map(|value| format!(".{}", ts_entity_name_to_string(value)))
    .unwrap_or_default();
  let arguments: String = import_type
    .type_args
    .as_ref()
    .map(|value| format!("<{}>", ts_type_params_to_string(value)))
    .unwrap_or_default();

  format!(
    "import(\"{}\"){}{}",
    import_type.arg.value.to_string_lossy(),
    qualifier,
    arguments,
  )
}

pub fn ts_entity_name_to_string(name: &TsEntityName) -> String {
  match name {
    TsEntityName::Ident(ident) => ident.sym.to_string(),
    TsEntityName::TsQualifiedName(qname) => {
      format!("{}.{}", ts_entity_name_to_string(&qname.left), qname.right.sym)
    }
  }
}

pub fn ts_type_ref_to_string(type_ref: &TsTypeRef) -> String {
  let name: String = ts_entity_name_to_string(&type_ref.type_name);

  if let Some(type_params) = &type_ref.type_params {
    format!("{}<{}>", name, ts_type_params_to_string(type_params))
  } else {
    name
  }
}

fn ts_type_params_to_string(type_params: &TsTypeParamInstantiation) -> String {
  type_params
    .params
    .iter()
    .map(|param| ts_type_to_string(param))
    .collect::<Vec<_>>()
    .join(", ")
}

pub fn ts_union_or_intersection_to_string(union_or_intersection: &TsUnionOrIntersectionType) -> String {
  match union_or_intersection {
    TsUnionOrIntersectionType::TsUnionType(union) => format!(
      "({})",
      union
        .types
        .iter()
        .map(|ts_type| ts_type_to_string(ts_type))
        .collect::<Vec<_>>()
        .join(" | ")
    ),
    TsUnionOrIntersectionType::TsIntersectionType(intersection) => format!(
      "({})",
      intersection
        .types
        .iter()
        .map(|ts_type| ts_type_to_string(ts_type))
        .collect::<Vec<_>>()
        .join(" & ")
    ),
  }
}

pub fn ts_array_type_to_string(array_type: &TsArrayType) -> String {
  format!("Array<{}>", ts_type_to_string(array_type.elem_type.as_ref()))
}

pub fn ts_keyword_type_to_string(keyword_type: &TsKeywordType) -> String {
  match keyword_type.kind {
    TsKeywordTypeKind::TsAnyKeyword => String::from("any"),
    TsKeywordTypeKind::TsUnknownKeyword => String::from("unknown"),
    TsKeywordTypeKind::TsStringKeyword => String::from("string"),
    TsKeywordTypeKind::TsNumberKeyword => String::from("number"),
    TsKeywordTypeKind::TsBooleanKeyword => String::from("boolean"),
    TsKeywordTypeKind::TsBigIntKeyword => String::from("bigint"),
    TsKeywordTypeKind::TsObjectKeyword => String::from("object"),
    TsKeywordTypeKind::TsSymbolKeyword => String::from("symbol"),
    TsKeywordTypeKind::TsVoidKeyword => String::from("void"),
    TsKeywordTypeKind::TsUndefinedKeyword => String::from("undefined"),
    TsKeywordTypeKind::TsNullKeyword => String::from("null"),
    TsKeywordTypeKind::TsNeverKeyword => String::from("never"),
    TsKeywordTypeKind::TsIntrinsicKeyword => String::from("intrinsic"),
  }
}
