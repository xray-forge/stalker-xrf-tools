use std::sync::{Arc, Mutex};

use xrf_translation::TranslationProjectJson;

pub struct TranslationProjectState {
  pub project: Arc<Mutex<Option<TranslationProjectJson>>>,
}

impl TranslationProjectState {
  pub fn new() -> Self {
    Self {
      project: Arc::new(Mutex::new(None)),
    }
  }
}
