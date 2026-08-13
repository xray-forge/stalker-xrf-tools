use std::fmt;

/// Newline style.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LineSeparator {
  /// Uses "\n" as new line separator.
  CR,
  /// Uses "\r\n" as new line separator.
  CRLF,
}

pub static DEFAULT_LINE_SEPARATOR: &str = "\r\n";

pub static DEFAULT_KV_SEPARATOR: &str = " = ";

impl fmt::Display for LineSeparator {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    formatter.write_str(self.as_str())
  }
}

impl LineSeparator {
  /// String representation
  pub fn as_str(self) -> &'static str {
    match self {
      Self::CR => "\n",
      Self::CRLF => "\r\n",
    }
  }
}
