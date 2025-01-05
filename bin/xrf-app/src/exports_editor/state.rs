use serde::Serialize;
use std::sync::{Arc, Mutex};
use xray_export::ExportDescriptor;

pub struct ExportsEditorState {
  pub conditions: Arc<Mutex<Option<Vec<ExportDescriptor>>>>,
  pub dialogs: Arc<Mutex<Option<Vec<ExportDescriptor>>>>,
  pub effects: Arc<Mutex<Option<Vec<ExportDescriptor>>>>,
}

impl ExportsEditorState {
  pub fn new() -> Self {
    Self {
      conditions: Arc::new(Mutex::new(None)),
      dialogs: Arc::new(Mutex::new(None)),
      effects: Arc::new(Mutex::new(None)),
    }
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportsDeclarations {
  pub conditions: Vec<ExportDescriptor>,
  pub dialogs: Vec<ExportDescriptor>,
  pub effects: Vec<ExportDescriptor>,
}
