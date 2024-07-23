use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TranslationVariant {
  String(String),
  MultiString(Vec<String>),
}

pub type TranslationEntry = HashMap<String, Option<TranslationVariant>>;

pub type TranslationJson = HashMap<String, TranslationEntry>;

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
