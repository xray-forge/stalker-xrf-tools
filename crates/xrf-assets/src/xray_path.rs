use xrf_error::XrfResult;

/// Normalizes a path into the canonical X-Ray logical form: lower case, backslash separated, no leading or trailing
/// separator.
///
/// Public because an out-of-crate [`crate::XrayAssetSource`] cannot key its entries correctly without the same rule.
/// Paths handed to a source are already normalized; a source normalizes only its own keys.
///
/// @returns The normalized path, or an error when it contains an empty, `.` or `..` component.
pub fn normalize_logical(path: &str) -> XrfResult<String> {
  crate::xray_asset_utils::normalize(path)
}

/// Whether a logical path sits under a prefix, matching on component boundaries so `configs_backup` does not match
/// `configs`.
///
/// Public for the same reason as [`normalize_logical`]: an out-of-crate source must scope its enumeration by the same rule,
/// and two copies that drift would make scoping depend on which kind of source answered.
pub fn is_component_prefix(path: &str, prefix: &str) -> bool {
  crate::xray_asset_utils::is_component_prefix(path, prefix)
}

/// Appends an extension to an X-Ray logical path when it is not already present.
pub fn with_extension(path: &str, extension: &str) -> String {
  if path.ends_with(extension) {
    path.to_string()
  } else {
    format!("{path}{extension}")
  }
}

#[cfg(test)]
mod tests {
  use super::with_extension;

  #[test]
  fn preserves_an_existing_extension() {
    assert_eq!(with_extension("actors\\stalker.ogf", ".ogf"), "actors\\stalker.ogf");
  }

  #[test]
  fn appends_a_missing_extension() {
    assert_eq!(with_extension("actors\\stalker", ".ogf"), "actors\\stalker.ogf");
  }
}
