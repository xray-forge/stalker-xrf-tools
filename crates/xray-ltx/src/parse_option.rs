/// Parsing configuration.
pub struct ParseOption {
  /// Allow quote (`"` or `'`) in value.
  pub enabled_quote: bool,
  /// Interpret `\` as an escape character.
  pub enabled_escape: bool,
}

impl Default for ParseOption {
  fn default() -> ParseOption {
    ParseOption {
      enabled_quote: true,
      enabled_escape: true,
    }
  }
}
