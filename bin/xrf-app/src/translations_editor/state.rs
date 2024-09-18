use std::sync::{Arc, Mutex};
use xray_translation::TranslationProjectJson;

pub struct TranslationsEditorState {
  pub project: Arc<Mutex<Option<TranslationProjectJson>>>,
}

impl TranslationsEditorState {
  pub fn new() -> TranslationsEditorState {
    TranslationsEditorState {
      project: Arc::new(Mutex::new(None)),
    }
  }
}
