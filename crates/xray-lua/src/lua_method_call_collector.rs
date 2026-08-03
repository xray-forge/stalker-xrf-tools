use crate::XRayLuaMethodCall;
use full_moon::ast::{Ast, Call, Expression, FunctionArgs, FunctionCall, Prefix, Suffix};
use full_moon::tokenizer::TokenType;
use full_moon::visitors::Visitor;

pub struct LuaMethodCallCollector {
  method_calls: Vec<XRayLuaMethodCall>,
}

impl LuaMethodCallCollector {
  pub fn collect(ast: &Ast) -> Vec<XRayLuaMethodCall> {
    let mut collector: Self = Self {
      method_calls: Vec::new(),
    };

    collector.visit_ast(ast);

    collector.method_calls
  }

  fn literal_string(expression: &Expression) -> Option<String> {
    let Expression::String(string) = expression else {
      return None;
    };
    let TokenType::StringLiteral { literal, .. } = string.token().token_type() else {
      return None;
    };

    Some(literal.to_string())
  }

  fn literal_string_arguments(arguments: &FunctionArgs) -> Option<Vec<String>> {
    match arguments {
      FunctionArgs::Parentheses { arguments, .. } => arguments
        .iter()
        .map(Self::literal_string)
        .collect::<Option<Vec<String>>>(),
      FunctionArgs::String(string) => match string.token().token_type() {
        TokenType::StringLiteral { literal, .. } => Some(vec![literal.to_string()]),
        _ => None,
      },
      FunctionArgs::TableConstructor(_) => None,
      _ => None,
    }
  }
}

impl Visitor for LuaMethodCallCollector {
  fn visit_function_call(&mut self, function_call: &FunctionCall) {
    let Prefix::Name(receiver) = function_call.prefix() else {
      return;
    };
    let Some(Suffix::Call(Call::MethodCall(method_call))) = function_call.suffixes().next() else {
      return;
    };

    self.method_calls.push(XRayLuaMethodCall::from_parts(
      method_call.name().token().start_position().line(),
      receiver.token().to_string(),
      method_call.name().token().to_string(),
      Self::literal_string_arguments(method_call.args()),
    ));
  }
}
