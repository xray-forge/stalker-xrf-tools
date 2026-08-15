use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::types::TranslationEntry;

/// Which layout a translations root is read with.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub enum TranslationProjectMode {
  /// XRF sources: multi-language JSON and language-suffixed XML side by side in one tree.
  #[default]
  Source,
  /// Shipped gamedata: `text/<language>/*.xml`, where the directory carries the language.
  Gamedata,
}

/// Something worth reporting about a file that was opened anyway.
///
/// The reader refuses nothing on content: an editor that will not open the file you need to fix is
/// no use, and the build and verifier keep their own guards.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct TranslationFinding {
  pub rule: String,
  pub subject: Option<String>,
  pub message: String,
}

impl TranslationFinding {
  pub fn new(rule: impl Into<String>, subject: Option<String>, message: impl Into<String>) -> Self {
    Self {
      rule: rule.into(),
      subject,
      message: message.into(),
    }
  }
}

/// One logical translation file, and where each language's copy of it lives.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct TranslationFile {
  /// Language to the file on disk that holds it, which is what an edit has to be written back to.
  /// A JSON source lists every language it carries against the same path.
  pub sources: IndexMap<String, String>,
  pub entries: IndexMap<String, TranslationEntry>,
}

/// An opened translations root, whichever layout it turned out to have.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct TranslationProjectDescriptor {
  pub mode: TranslationProjectMode,
  pub root: String,
  /// Every language the root offers, in discovery order.
  pub languages: Vec<String>,
  /// The code page each language is written in, which is what limits the characters it can hold.
  ///
  /// Taken from the files themselves in gamedata mode, so a language XRF has never heard of still
  /// reports the encoding its own declaration claims.
  pub encodings: IndexMap<String, String>,
  pub files: IndexMap<String, TranslationFile>,
  pub findings: Vec<TranslationFinding>,
}
