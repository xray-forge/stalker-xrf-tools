use std::path::Path;

use xray_error::{XRayError, XRayResult};

/// Converts a root-relative physical path into the canonical X-Ray logical path used for indexing.
pub(crate) fn logical_path(path: &Path) -> XRayResult<String> {
  normalize(path.to_str().ok_or_else(|| {
    XRayError::new_asset_error(format!("directory asset path is not valid UTF-8: {}", path.display()))
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
  path == prefix || path.strip_prefix(prefix).is_some_and(|rest| rest.starts_with('\\'))
}

pub(crate) fn join(prefix: &str, path: &str) -> XRayResult<String> {
  match (prefix.is_empty(), path.is_empty()) {
    (true, true) => normalize(""),
    (true, false) => normalize(path),
    (false, true) => normalize(prefix),
    (false, false) => normalize(&format!("{prefix}\\{path}")),
  }
}

#[cfg(test)]
mod tests {
  use super::{is_component_prefix, normalize};

  #[test]
  fn normalizes_case_and_separators() {
    assert_eq!(
      normalize("Textures/Sky\\Clouds.DDS").unwrap(),
      "textures\\sky\\clouds.dds"
    );
  }

  #[test]
  fn rejects_ambiguous_path_components() {
    assert!(normalize("textures//clouds.dds").is_err());
    assert!(normalize("textures/../clouds.dds").is_err());
  }

  #[test]
  fn prefixes_end_on_component_boundaries() {
    assert!(is_component_prefix("scripts\\ui.script", "scripts"));
    assert!(!is_component_prefix("scripts_extra\\ui.script", "scripts"));
  }
}
