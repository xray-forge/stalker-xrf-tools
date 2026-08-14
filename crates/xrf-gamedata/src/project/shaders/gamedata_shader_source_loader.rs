use std::fs;
use std::path::Path;

use xrf_error::{XrfError, XrfResult};
use xrf_shaders::XRayShaderSourceLoader;

pub struct GamedataShaderSourceLoader;

impl XRayShaderSourceLoader for GamedataShaderSourceLoader {
  fn load_source(&self, path: &Path) -> XrfResult<Option<Vec<u8>>> {
    if !path.is_file() {
      return Ok(None);
    }

    fs::read(path)
      .map(Some)
      .map_err(|error| XrfError::new_read_error(format!("Failed to read shader source {}: {error}", path.display())))
  }
}
