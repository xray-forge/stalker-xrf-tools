use std::sync::{Arc, Mutex};

use xrf_db::SpawnFile;

pub struct SpawnFileState {
  pub file: Arc<Mutex<Option<SpawnFile>>>,
  /// Where the open file came from.
  pub path: Arc<Mutex<Option<String>>>,
}

impl SpawnFileState {
  pub fn new() -> Self {
    Self {
      file: Arc::new(Mutex::new(None)),
      path: Arc::new(Mutex::new(None)),
    }
  }
}
