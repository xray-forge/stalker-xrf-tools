use std::fmt;

/// Newline style.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LineSeparator {
  /// System-dependent line separator.
  ///
  /// On UNIX system, uses "\n"
  /// On Windows system, uses "\r\n"
  SystemDefault,
  /// Uses "\n" as new line separator.
  CR,
  /// Uses "\r\n" as new line separator.
  CRLF,
}

#[cfg(not(windows))]
pub static DEFAULT_LINE_SEPARATOR: &str = "\n";
#[cfg(windows)]
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
      Self::SystemDefault => DEFAULT_LINE_SEPARATOR,
      Self::CR => "\n",
      Self::CRLF => "\r\n",
    }
  }
}
