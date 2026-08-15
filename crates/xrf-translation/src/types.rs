use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// One translation's text, which is a single line or a run of them joined on build.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[serde(untagged)]
pub enum TranslationVariant {
  String(String),
  MultiString(Vec<String>),
}

impl TranslationVariant {
  /// Render the value as the single line a string table holds.
  ///
  /// The engine reads a literal `\n` in a string table as a line break, so a multi-line entry joins
  /// on it rather than losing its structure. Only a JSON source can hold the array form.
  pub fn to_single_line(&self) -> String {
    match self {
      Self::String(value) => value.clone(),
      Self::MultiString(values) => values.join("\\n"),
    }
  }
}

/// One id's text in each language that has it. A missing language is a gap the engine fills with the
/// id itself, so absence is meaningful and is not the same as an empty string.
pub type TranslationEntry = IndexMap<String, Option<TranslationVariant>>;

/// Every id in one source file.
pub type TranslationJson = IndexMap<String, TranslationEntry>;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn a_single_string_renders_as_itself() {
    assert_eq!(
      TranslationVariant::String(String::from("one line")).to_single_line(),
      "one line"
    );
  }

  #[test]
  fn a_multi_line_value_joins_on_the_engine_line_break() {
    assert_eq!(
      TranslationVariant::MultiString(vec![String::from("first"), String::from("second")]).to_single_line(),
      "first\\nsecond"
    );
  }
}
