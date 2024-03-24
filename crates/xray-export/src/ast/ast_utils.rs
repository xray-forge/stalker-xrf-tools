use crate::ast::ast_to_string::{ts_type_ref_to_string, ts_type_to_string};
use crate::extern_descriptor::ExternParameterDescriptor;
use swc_ecma_ast::{Callee, Expr, ExprOrSpread, Lit, Pat, TsType, TsTypeAnn};

pub fn get_expression_callee_name(callee: &Callee) -> Option<String> {
  if let Callee::Expr(callee_expression) = callee {
    if let Expr::Ident(identifier) = callee_expression.as_ref() {
      return Some(identifier.sym.to_string());
    }
  }

  None
}

pub fn get_expression_parameter_as_string_name(expression: &ExprOrSpread) -> Option<String> {
  if let Expr::Lit(literal) = expression.expr.as_ref() {
    return if let Lit::Str(string_literal) = literal {
      Some(string_literal.value.to_string())
    } else {
      None
    };
  }

  None
}

pub fn get_parameters_from_arrow_expression(
  expression: &ExprOrSpread,
) -> Vec<ExternParameterDescriptor> {
  if let Expr::Arrow(arrow) = expression.expr.as_ref() {
    if arrow.params.len() == 3 {
      let third = arrow.params.get(2).unwrap();

      match third {
        Pat::Ident(identifier) => {
          let mut params: Vec<ExternParameterDescriptor> = Vec::new();

          if let Some(type_annotation) = identifier.type_ann.as_ref() {
            match type_annotation.type_ann.as_ref() {
              TsType::TsTupleType(tuple_type) => {
                for (index, element_type) in tuple_type.elem_types.iter().enumerate() {
                  params.push(ExternParameterDescriptor {
                    name: format!("{}.{}", identifier.sym, index),
                    typing: ts_type_to_string(element_type.ty.as_ref()),
                    comment: None,
                  });
                }
              }
              TsType::TsTypeRef(type_ref) => {
                params.push(ExternParameterDescriptor {
                  name: format!("...{}", identifier.sym,),
                  typing: ts_type_ref_to_string(type_ref),
                  comment: None,
                });
              }
              _ => {
                log::warn!(
                  "Unexpected type was not parsed {:?}: {:?}",
                  identifier.sym,
                  identifier.type_ann
                )
              }
            }
          }

          return params;
        }
        Pat::Array(array_pattern) => {
          if let Some(type_annotation) = array_pattern.type_ann.as_ref() {
            let names: Vec<String> = array_pattern
              .elems
              .iter()
              .filter_map(|it| match it.as_ref().unwrap() {
                Pat::Ident(ident) => Some(ident.sym.to_string()),
                Pat::Assign(ident) => Some(ident.left.as_ident().unwrap().sym.to_string()),
                pattern => {
                  log::warn!("Unexpected pattern: {:?}", pattern);
                  None
                }
              })
              .collect();

            return get_parameters_descriptors_from_annotations(&names, type_annotation);
          }
        }

        _ => {}
      }
    }
  }

  Vec::new()
}

pub fn get_parameters_descriptors_from_annotations(
  names: &[String],
  type_annotation: &TsTypeAnn,
) -> Vec<ExternParameterDescriptor> {
  let mut parameters: Vec<ExternParameterDescriptor> = Vec::new();

  match type_annotation.type_ann.as_ref() {
    TsType::TsTupleType(tuple_type) => {
      assert_eq!(
        names.len(),
        tuple_type.elem_types.len(),
        "Expected same types count as array params"
      );

      for (index, name) in names.iter().enumerate() {
        parameters.push(ExternParameterDescriptor {
          name: name.clone(),
          typing: ts_type_to_string(tuple_type.elem_types.get(index).unwrap().ty.as_ref()),
          comment: None,
        })
      }
    }
    other => {
      log::warn!("Unexpected parameters type: {:?}", other);
    }
  }

  parameters
}
