use derive_more::Display;
use std::str::FromStr;
use xray_error::{XRayError, XRayResult};
use xray_utils::{XRayEncoding, get_windows1250_encoder, get_windows1251_encoder};

pub const MULTILANGUAGE: &str = "multilang";

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

impl FromStr for TranslationLanguage {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "all" => Ok(Self::All),
      "eng" => Ok(Self::English),
      "rus" => Ok(Self::Russian),
      "ukr" => Ok(Self::Ukrainian),
      "pol" => Ok(Self::Polish),
      "fra" => Ok(Self::French),
      "ger" => Ok(Self::German),
      "ita" => Ok(Self::Italian),
      "spa" => Ok(Self::Spanish),
      _ => Err(format!("Unknown language: {}", s)),
    }
  }
}

impl TranslationLanguage {
  pub fn get_language_encoding(&self) -> String {
    match self {
      Self::Russian | Self::Ukrainian => String::from("windows-1251"),
      _ => String::from("windows-1250"),
    }
  }

  pub fn get_language_encoder(&self) -> XRayEncoding {
    match self {
      Self::Russian | Self::Ukrainian => get_windows1251_encoder(),
      _ => get_windows1250_encoder(),
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

  pub fn from_str_single(language: &str) -> XRayResult<Self> {
    match Self::from_str(language).map_err(|it| XRayError::new_parsing_error(it.to_string()))? {
      Self::All => Err(XRayError::new_unknown_language_error(String::from(
        "Unexpected language 'all' provided'",
      ))),
      language => Ok(language),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::TranslationLanguage;
  use std::str::FromStr;

  #[test]
  fn test_from_str() {
    assert_eq!(
      TranslationLanguage::from_str("eng").unwrap(),
      TranslationLanguage::English
    );
    assert_eq!(
      TranslationLanguage::from_str("ukr").unwrap(),
      TranslationLanguage::Ukrainian
    );
    assert_eq!(
      TranslationLanguage::from_str("all").unwrap(),
      TranslationLanguage::All
    );
  }

  #[test]
  fn test_from_str_single() {
    assert!(TranslationLanguage::from_str_single("all").is_err());
    assert_eq!(
      TranslationLanguage::from_str_single("eng").unwrap(),
      TranslationLanguage::English
    );
    assert_eq!(
      TranslationLanguage::from_str_single("spa").unwrap(),
      TranslationLanguage::Spanish
    );
  }
}
