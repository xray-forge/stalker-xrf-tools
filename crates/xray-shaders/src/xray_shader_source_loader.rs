use std::path::Path;

use xray_error::XRayResult;

/// Loads shader source bytes for `XRayShader` import resolution.
///
/// `Ok(None)` means the path does not exist. Other loading failures are
/// returned as `Err` and preserve the caller's storage-specific context.
pub trait XRayShaderSourceLoader {
  fn load_source(&self, path: &Path) -> XRayResult<Option<Vec<u8>>>;
}
