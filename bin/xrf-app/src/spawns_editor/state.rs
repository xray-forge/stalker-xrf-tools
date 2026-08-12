use std::sync::{Arc, Mutex};

use xray_db::SpawnFile;

pub struct SpawnsEditorState {
  pub file: Arc<Mutex<Option<SpawnFile>>>,
  /// Where the open file came from.
  pub path: Arc<Mutex<Option<String>>>,
}

impl SpawnsEditorState {
  pub fn new() -> Self {
    Self {
      file: Arc::new(Mutex::new(None)),
      path: Arc::new(Mutex::new(None)),
    }
  }
}
