/// A Lua method call with its receiver, method name, source line, and literal string arguments.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XRayLuaMethodCall {
  line_number: usize,
  literal_string_arguments: Option<Vec<String>>,
  method: String,
  receiver: String,
}

impl XRayLuaMethodCall {
  pub(crate) fn from_parts(
    line_number: usize,
    receiver: String,
    method: String,
    literal_string_arguments: Option<Vec<String>>,
  ) -> Self {
    Self {
      line_number,
      literal_string_arguments,
      method,
      receiver,
    }
  }

  pub fn line_number(&self) -> usize {
    self.line_number
  }

  pub fn literal_string_arguments(&self) -> Option<&[String]> {
    self.literal_string_arguments.as_deref()
  }

  pub fn method(&self) -> &str {
    &self.method
  }

  pub fn receiver(&self) -> &str {
    &self.receiver
  }
}
