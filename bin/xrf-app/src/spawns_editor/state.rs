use std::sync::{Arc, Mutex};
use xray_db::file::spawn_file::SpawnFile;

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
