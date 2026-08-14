use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[serde(untagged)]
pub enum TranslationVariant {
  String(String),
  MultiString(Vec<String>),
}

pub type TranslationEntry = IndexMap<String, Option<TranslationVariant>>;

pub type TranslationJson = IndexMap<String, TranslationEntry>;

pub type TranslationProjectJson = IndexMap<String, TranslationJson>;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename = "string")]
pub struct TranslationEntryCompiled {
  #[serde(rename = "@id")]
  pub id: String,
  pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename = "string_table")]
pub struct TranslationCompiledXml {
  pub string: Vec<TranslationEntryCompiled>,
}
