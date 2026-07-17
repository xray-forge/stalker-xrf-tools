use xray_error::{XRayError, XRayResult};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TupleSeparator {
  Comma,
  Pipe,
}

impl TupleSeparator {
  pub fn from_tuple_type(field_name: &str, section_name: &str, value: &str) -> XRayResult<Self> {
    match value {
      "tuple" | "tuple@comma" => Ok(Self::Comma),
      "tuple@pipe" => Ok(Self::Pipe),
      _ => Err(XRayError::new_ltx_scheme_error(
        section_name,
        field_name,
        "Failed to parse tuple type, expected 'tuple', 'tuple@comma', or 'tuple@pipe'",
      )),
    }
  }

  pub fn as_char(self) -> char {
    match self {
      Self::Comma => ',',
      Self::Pipe => '|',
    }
  }

  pub fn as_name(self) -> &'static str {
    match self {
      Self::Comma => "comma",
      Self::Pipe => "pipe",
    }
  }
}
