use std::sync::{Arc, Mutex};

use xrf_translation::TranslationProjectDescriptor;

/// The open translations root.
pub struct TranslationProjectState {
  pub project: Arc<Mutex<Option<TranslationProjectDescriptor>>>,
}

impl TranslationProjectState {
  pub fn new() -> Self {
    Self {
      project: Arc::new(Mutex::new(None)),
    }
  }
}
