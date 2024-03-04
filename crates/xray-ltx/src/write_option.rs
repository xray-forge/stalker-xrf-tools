use crate::escape_policy::EscapePolicy;
use crate::line_separator::{LineSeparator, DEFAULT_KV_SEPARATOR};

#[derive(Debug, Clone)]
pub struct WriteOption {
  /// Policies about how to escape characters.
  pub escape_policy: EscapePolicy,
  /// Newline style.
  pub line_separator: LineSeparator,
  /// Key value separator.
  pub kv_separator: &'static str,
}

impl Default for WriteOption {
  fn default() -> WriteOption {
    WriteOption {
      escape_policy: EscapePolicy::Basics,
      line_separator: LineSeparator::SystemDefault,
      kv_separator: DEFAULT_KV_SEPARATOR,
    }
  }
}
