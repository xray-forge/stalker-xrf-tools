use std::sync::{Arc, Mutex};

use xrf_export::ExportsProject;

pub struct ExportsEditorState {
  pub project: Arc<Mutex<Option<ExportsProject>>>,
}

impl ExportsEditorState {
  pub fn new() -> Self {
    Self {
      project: Arc::new(Mutex::new(None)),
    }
  }
}
