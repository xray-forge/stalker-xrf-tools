use std::sync::{Arc, Mutex};

use xrf_export::ExportsProject;

pub struct ExportsProjectState {
  pub project: Arc<Mutex<Option<ExportsProject>>>,
}

impl ExportsProjectState {
  pub fn new() -> Self {
    Self {
      project: Arc::new(Mutex::new(None)),
    }
  }
}
