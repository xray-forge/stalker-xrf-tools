use std::sync::{Arc, Mutex};
use xray_translation::TranslationProjectJson;

pub struct TranslationsEditorState {
  pub project: Arc<Mutex<Option<TranslationProjectJson>>>,
}

impl TranslationsEditorState {
  pub fn new() -> Self {
    Self {
      project: Arc::new(Mutex::new(None)),
    }
  }
}
