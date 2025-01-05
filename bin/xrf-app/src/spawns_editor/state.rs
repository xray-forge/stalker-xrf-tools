use std::sync::{Arc, Mutex};
use xray_db::SpawnFile;

pub struct SpawnsEditorState {
  pub file: Arc<Mutex<Option<SpawnFile>>>,
}

impl SpawnsEditorState {
  pub fn new() -> Self {
    Self {
      file: Arc::new(Mutex::new(None)),
    }
  }
}
