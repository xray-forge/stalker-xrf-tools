/// Parsing configuration.
#[derive(Clone)]
pub struct ParseOptions {
  /// Allow quote (`"` or `'`) in value.
  pub enabled_quote: bool,
  /// Interpret `\` as an escape character.
  pub enabled_escape: bool,
}

impl Default for ParseOptions {
  fn default() -> ParseOptions {
    ParseOptions {
      enabled_quote: true,
      enabled_escape: true,
    }
  }
}
