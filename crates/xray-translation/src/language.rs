use crate::error::translation_error::TranslationError;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum TranslationLanguage {
  All,
  English,
  Russian,
  Ukrainian,
  Polish,
  French,
  German,
  Italian,
  Spanish,
}

impl TranslationLanguage {
  pub fn as_str(&self) -> &'static str {
    match self {
      TranslationLanguage::All => "all",
      TranslationLanguage::English => "eng",
      TranslationLanguage::French => "fra",
      TranslationLanguage::German => "ger",
      TranslationLanguage::Italian => "ita",
      TranslationLanguage::Polish => "pol",
      TranslationLanguage::Russian => "rus",
      TranslationLanguage::Spanish => "spa",
      TranslationLanguage::Ukrainian => "ukr",
    }
  }

  pub fn get_all() -> Vec<TranslationLanguage> {
    vec![
      TranslationLanguage::English,
      TranslationLanguage::French,
      TranslationLanguage::German,
      TranslationLanguage::Italian,
      TranslationLanguage::Polish,
      TranslationLanguage::Russian,
      TranslationLanguage::Spanish,
      TranslationLanguage::Ukrainian,
    ]
  }
}

impl FromStr for TranslationLanguage {
  type Err = TranslationError;

  fn from_str(language: &str) -> Result<Self, Self::Err> {
    match language {
      "all" => Ok(TranslationLanguage::All),
      "eng" => Ok(TranslationLanguage::English),
      "fra" => Ok(TranslationLanguage::French),
      "ger" => Ok(TranslationLanguage::German),
      "ita" => Ok(TranslationLanguage::Italian),
      "pol" => Ok(TranslationLanguage::Polish),
      "rus" => Ok(TranslationLanguage::Russian),
      "spa" => Ok(TranslationLanguage::Spanish),
      "ukr" => Ok(TranslationLanguage::Ukrainian),
      language => Err(TranslationError::UnknownLanguage(format!(
        "Unexpected language '{language} provided'",
      ))),
    }
  }
}
