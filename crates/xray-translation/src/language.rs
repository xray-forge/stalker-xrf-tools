use crate::{TranslationError, TranslationResult};
use derive_more::Display;
use encoding_rs::{Encoding, WINDOWS_1250, WINDOWS_1251};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Display)]
pub enum TranslationLanguage {
  #[display("all")]
  All,
  #[display("eng")]
  English,
  #[display("rus")]
  Russian,
  #[display("ukr")]
  Ukrainian,
  #[display("pol")]
  Polish,
  #[display("fra")]
  French,
  #[display("ger")]
  German,
  #[display("ita")]
  Italian,
  #[display("spa")]
  Spanish,
}

impl TranslationLanguage {
  pub fn get_language_encoding(&self) -> String {
    match self {
      Self::Russian | Self::Ukrainian => String::from("windows-1251"),
      _ => String::from("windows-1250"),
    }
  }

  pub fn get_language_encoder(&self) -> &'static Encoding {
    match self {
      Self::Russian | Self::Ukrainian => WINDOWS_1251,
      _ => WINDOWS_1250,
    }
  }

  pub fn get_all() -> Vec<Self> {
    vec![
      Self::English,
      Self::French,
      Self::German,
      Self::Italian,
      Self::Polish,
      Self::Russian,
      Self::Spanish,
      Self::Ukrainian,
    ]
  }

  pub fn get_all_strings() -> Vec<String> {
    Self::get_all().iter().map(|it| it.to_string()).collect()
  }

  pub fn from_str_single(language: &str) -> TranslationResult<Self> {
    match Self::from_str(language)? {
      Self::All => Err(TranslationError::new_unknown_language_error(String::from(
        "Unexpected language 'all' provided'",
      ))),
      language => Ok(language),
    }
  }
}

impl FromStr for TranslationLanguage {
  type Err = TranslationError;

  fn from_str(language: &str) -> Result<Self, Self::Err> {
    match language {
      "all" => Ok(Self::All),
      "eng" => Ok(Self::English),
      "fra" => Ok(Self::French),
      "ger" => Ok(Self::German),
      "ita" => Ok(Self::Italian),
      "pol" => Ok(Self::Polish),
      "rus" => Ok(Self::Russian),
      "spa" => Ok(Self::Spanish),
      "ukr" => Ok(Self::Ukrainian),
      language => Err(TranslationError::new_unknown_language_error(format!(
        "Unexpected language '{language} provided'",
      ))),
    }
  }
}
