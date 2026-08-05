use std::path::Path;
use xray_error::{XRayError, XRayResult};

pub(crate) fn logical_path(path: &Path) -> XRayResult<String> {
  normalize(path.to_str().ok_or_else(|| {
    XRayError::new_asset_error(format!(
      "directory asset path is not valid UTF-8: {}",
      path.display()
    ))
  })?)
}

pub(crate) fn normalize(path: &str) -> XRayResult<String> {
  let normalized: String = path.replace('/', "\\").to_lowercase();
  let normalized: &str = normalized.trim_matches('\\');

  if normalized.is_empty()
    || normalized
      .split('\\')
      .any(|part| part.is_empty() || matches!(part, "." | ".."))
  {
    return Err(XRayError::new_asset_error(format!(
      "invalid X-Ray logical path: {path}"
    )));
  }

  Ok(normalized.to_string())
}

pub(crate) fn is_component_prefix(path: &str, prefix: &str) -> bool {
  path == prefix
    || path
      .strip_prefix(prefix)
      .is_some_and(|rest| rest.starts_with('\\'))
}
