use std::sync::{Arc, Mutex};

use xrf_volume::ArchiveProject;

pub struct ArchiveProjectState {
  pub project: Arc<Mutex<Option<ArchiveProject>>>,
}

impl ArchiveProjectState {
  pub fn new() -> Self {
    Self {
      project: Arc::new(Mutex::new(None)),
    }
  }
}
