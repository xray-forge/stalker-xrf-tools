use derive_more::{Display, FromStr};
use xray_error::{XRayError, XRayResult};
use xray_utils::{get_windows1250_encoder, get_windows1251_encoder, XRayEncoding};

#[derive(Clone, Debug, PartialEq, Display, FromStr)]
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
