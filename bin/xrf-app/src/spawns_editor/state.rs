use std::sync::{Arc, Mutex};
use xray_db::file::spawn_file::SpawnFile;

pub struct SpawnsEditorState {
  pub file: Arc<Mutex<Option<SpawnFile>>>,
}
