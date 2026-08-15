use serde::{Deserialize, Serialize};

/// One `<string id="…"><text>…</text></string>` as the build emits it.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename = "string")]
pub(crate) struct TranslationEntryCompiled {
  #[serde(rename = "@id")]
  pub id: String,
  pub text: String,
}

/// A whole string table document, for serializing only.
///
/// Reading goes through the span reader instead, which keeps positions so an edit can splice the
/// original bytes. This shape exists to write a file the build generates from scratch.
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename = "string_table")]
pub(crate) struct TranslationCompiledXml {
  pub string: Vec<TranslationEntryCompiled>,
}
