use swc_ecma_ast::{
  TsArrayType, TsKeywordType, TsKeywordTypeKind, TsType, TsTypeParamInstantiation, TsTypeRef,
  TsUnionOrIntersectionType,
};

pub fn ts_type_to_string(ty: &TsType) -> String {
  match ty {
    TsType::TsKeywordType(keyword_type) => ts_keyword_type_to_string(keyword_type),
    TsType::TsTypeRef(type_ref) => ts_type_ref_to_string(type_ref),
    TsType::TsArrayType(array_type) => ts_array_type_to_string(array_type),
    TsType::TsUnionOrIntersectionType(union_type) => ts_union_or_intersection_to_string(union_type),
    _ => String::from("unsupported"),
  }
}

pub fn ts_type_ref_to_string(type_ref: &TsTypeRef) -> String {
  let name: String = type_ref.type_name.as_ident().unwrap().sym.to_string();

  if let Some(type_params) = &type_ref.type_params {
    let params_str = ts_type_params_to_string(type_params);
    format!("{}<{}>", name, params_str)
  } else {
    name
  }
}

// Helper function to transform type parameters to a string
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
    TsUnionOrIntersectionType::TsUnionType(union) => {
      let types = union
        .types
        .iter()
        .map(|ts_type| ts_type_to_string(ts_type))
        .collect::<Vec<_>>()
        .join(" | ");

      format!("({})", types)
    }
    TsUnionOrIntersectionType::TsIntersectionType(intersection) => {
      let types = intersection
        .types
        .iter()
        .map(|ts_type| ts_type_to_string(ts_type))
        .collect::<Vec<_>>()
        .join(" & ");

      format!("({})", types)
    }
  }
}

pub fn ts_array_type_to_string(array_type: &TsArrayType) -> String {
  format!("Array<{}>", ts_type_to_string(&array_type.elem_type))
}

pub fn ts_keyword_type_to_string(keyword_type: &TsKeywordType) -> String {
  match keyword_type.kind {
    TsKeywordTypeKind::TsStringKeyword => String::from("string"),
    TsKeywordTypeKind::TsNumberKeyword => String::from("number"),
    TsKeywordTypeKind::TsBooleanKeyword => String::from("boolean"),
    _ => String::from("unknown"),
  }
}
