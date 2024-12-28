use std::sync::{Arc, Mutex};
use xray_db::spawn_file::spawn_file::SpawnFile;

pub struct SpawnsEditorState {
  pub file: Arc<Mutex<Option<SpawnFile>>>,
}

impl SpawnsEditorState {
  pub fn new() -> SpawnsEditorState {
    SpawnsEditorState {
      file: Arc::new(Mutex::new(None)),
    }
  }
}
