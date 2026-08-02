use swc_ecma_ast::{
  Callee, Expr, ExprOrSpread, Lit, TsArrayType, TsEntityName, TsKeywordType, TsKeywordTypeKind,
  TsLitType, TsType, TsTypeOperator, TsTypeOperatorOp, TsTypeParamInstantiation, TsTypeQuery,
  TsTypeQueryExpr, TsTypeRef, TsUnionOrIntersectionType,
};

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
    other => {
      log::warn!("Parsed unsupported TypeScript type: {:?}", other);
      String::from("unsupported")
    }
  }
}

pub fn ts_literal_type_to_string(literal_type: &TsLitType) -> String {
  format!(
    "\"{}\"",
    literal_type.lit.as_str().unwrap().value.to_string_lossy()
  )
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
  format!(
    "typeof {}",
    ts_entity_query_to_string(&type_query.expr_name)
  )
}

pub fn ts_entity_query_to_string(name: &TsTypeQueryExpr) -> String {
  match name {
    TsTypeQueryExpr::TsEntityName(entity_name) => ts_entity_name_to_string(entity_name),
    TsTypeQueryExpr::Import(_) => String::from("unsupported"),
  }
}

pub fn ts_entity_name_to_string(name: &TsEntityName) -> String {
  match name {
    TsEntityName::Ident(ident) => ident.sym.to_string(),
    TsEntityName::TsQualifiedName(qname) => {
      format!(
        "{}.{}",
        ts_entity_name_to_string(&qname.left),
        qname.right.sym
      )
    }
  }
}

pub fn ts_type_ref_to_string(type_ref: &TsTypeRef) -> String {
  let name: String = type_ref.type_name.as_ident().unwrap().sym.to_string();

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

pub fn ts_union_or_intersection_to_string(
  union_or_intersection: &TsUnionOrIntersectionType,
) -> String {
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
  format!(
    "Array<{}>",
    ts_type_to_string(array_type.elem_type.as_ref())
  )
}

pub fn ts_keyword_type_to_string(keyword_type: &TsKeywordType) -> String {
  match keyword_type.kind {
    TsKeywordTypeKind::TsStringKeyword => String::from("string"),
    TsKeywordTypeKind::TsNumberKeyword => String::from("number"),
    TsKeywordTypeKind::TsBooleanKeyword => String::from("boolean"),
    _ => String::from("unknown"),
  }
}
