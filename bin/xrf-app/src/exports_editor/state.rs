use std::sync::{Arc, Mutex};

use xray_export::ExportDescriptor;

pub struct ExportsEditorState {
  pub exports: Arc<Mutex<Option<Vec<ExportDescriptor>>>>,
}

impl ExportsEditorState {
  pub fn new() -> Self {
    Self {
      exports: Arc::new(Mutex::new(None)),
    }
  }
}
