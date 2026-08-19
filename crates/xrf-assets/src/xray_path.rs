use std::fmt::{Display, Formatter, Result as FormatResult};
use std::path::Path;

use xrf_error::{XrfError, XrfResult};

/// An X-Ray logical path: lower case, backslash separated, with no empty, `.` or `..` component.
///
/// This is an engine identity, not a location on disk. The asset it names may sit inside an archive and have no file at
/// all, so the type deliberately does not implement `AsRef<Path>` — handing one to host I/O must not compile. Read it
/// through an [`crate::XrayVfs`], and ask [`crate::XrayAssetLocation::physical_path`] when a real file is genuinely
/// required.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct XrayPath(String);

impl XrayPath {
  /// Normalizes a reference into an engine identity.
  ///
  /// # Errors
  ///
  /// Returns an error when the path is empty or holds an empty, `.` or `..` component.
  pub fn new(path: &str) -> XrfResult<Self> {
    Ok(Self(normalize(path)?))
  }

  /// The normalized path, for lookups and messages.
  pub fn as_str(&self) -> &str {
    &self.0
  }

  /// Appends a component below this path.
  ///
  /// # Errors
  ///
  /// Returns an error when the result is not a valid logical path.
  pub fn join(&self, component: &str) -> XrfResult<Self> {
    Ok(Self(join(&self.0, component)?))
  }

  /// The directory holding this path, or `None` when it names a top-level entry.
  pub fn parent(&self) -> Option<Self> {
    self.0.rsplit_once('\\').map(|(parent, _)| Self(parent.to_string()))
  }

  /// The final component, which for a file is its name and extension.
  pub fn file_name(&self) -> &str {
    self.0.rsplit_once('\\').map_or(self.0.as_str(), |(_, name)| name)
  }

  /// Whether the path carries `extension`, which is compared without case.
  ///
  /// `extension` is matched with its leading dot, as in `.ltx`.
  pub fn has_extension(&self, extension: &str) -> bool {
    has_extension(&self.0, extension)
  }

  /// Whether this path sits under `prefix`, matching on component boundaries.
  ///
  /// # Errors
  ///
  /// Returns an error when `prefix` is not a valid logical path.
  pub fn is_under(&self, prefix: &str) -> XrfResult<bool> {
    Ok(is_component_prefix(&self.0, &normalize(prefix)?))
  }
}

impl Display for XrayPath {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
    formatter.write_str(&self.0)
  }
}

/// Normalizes a path into the canonical X-Ray logical form: lower case, backslash separated, no leading or trailing
/// separator.
///
/// Public because an out-of-crate [`crate::XrayAssetSource`] cannot key its entries correctly without the same rule.
/// Paths handed to a source are already normalized; a source normalizes only its own keys. A source keys a map of many
/// thousands of names, which is why this answers a `String` rather than an [`XrayPath`].
///
/// # Errors
///
/// Returns an error when the path contains an empty, `.` or `..` component.
pub fn normalize_logical(path: &str) -> XrfResult<String> {
  normalize(path)
}

/// Whether a logical path sits under a prefix, matching on component boundaries so `configs_backup` does not match
/// `configs`.
///
/// Public for the same reason as [`normalize_logical`]: an out-of-crate source must scope its enumeration by the same rule,
/// and two copies that drift would make scoping depend on which kind of source answered. Both arguments are expected to be
/// normalized already.
pub fn is_component_prefix(path: &str, prefix: &str) -> bool {
  path == prefix || path.strip_prefix(prefix).is_some_and(|rest| rest.starts_with('\\'))
}

pub(crate) fn normalize(path: &str) -> XrfResult<String> {
  let normalized: String = path.replace('/', "\\").to_lowercase();
  let normalized: &str = normalized.trim_matches('\\');

  if normalized.is_empty()
    || normalized
      .split('\\')
      .any(|part| part.is_empty() || matches!(part, "." | ".."))
  {
    return Err(XrfError::new_asset_error(format!("invalid X-Ray logical path: {path}")));
  }

  Ok(normalized.to_string())
}

/// Converts a root-relative host path into the canonical X-Ray logical path used for indexing.
///
/// Named for the domain it crosses: the input is a host path fragment, the output an engine identity.
pub(crate) fn normalize_host_relative(path: &Path) -> XrfResult<String> {
  normalize(
    path.to_str().ok_or_else(|| {
      XrfError::new_asset_error(format!("directory asset path is not valid UTF-8: {}", path.display()))
    })?,
  )
}

pub(crate) fn join(prefix: &str, path: &str) -> XrfResult<String> {
  match (prefix.is_empty(), path.is_empty()) {
    (true, true) => normalize(""),
    (true, false) => normalize(path),
    (false, true) => normalize(prefix),
    (false, false) => normalize(&format!("{prefix}\\{path}")),
  }
}

/// Whether a logical path carries `extension`, compared without case.
pub(crate) fn has_extension(path: &str, extension: &str) -> bool {
  path.len() > extension.len()
    && path
      .get(path.len() - extension.len()..)
      .is_some_and(|tail| tail.eq_ignore_ascii_case(extension))
}

/// Appends an extension to an X-Ray logical path when it is not already present.
///
/// The comparison ignores case, because a reference authored as `actors\stalker.OGF` names the same asset as
/// `actors\stalker.ogf`, and appending a second extension would resolve to nothing.
pub(crate) fn with_extension(path: &str, extension: &str) -> String {
  if has_extension(path, extension) {
    path.to_string()
  } else {
    format!("{path}{extension}")
  }
}

#[cfg(test)]
mod tests {
  use std::path::PathBuf;

  use super::{XrayPath, has_extension, join, normalize, normalize_host_relative, with_extension};

  #[test]
  fn preserves_an_existing_extension() {
    assert_eq!(with_extension("actors\\stalker.ogf", ".ogf"), "actors\\stalker.ogf");
  }

  #[test]
  fn appends_a_missing_extension() {
    assert_eq!(with_extension("actors\\stalker", ".ogf"), "actors\\stalker.ogf");
  }

  #[test]
  fn treats_an_authored_extension_as_present_whatever_its_case() {
    // A doubled extension normalizes to `actors\stalker.ogf.ogf`, which resolves to nothing.
    assert_eq!(with_extension("actors\\stalker.OGF", ".ogf"), "actors\\stalker.OGF");
    assert_eq!(with_extension("actors\\stalker.Omf", ".omf"), "actors\\stalker.Omf");
  }

  #[test]
  fn does_not_treat_a_bare_extension_as_a_named_asset() {
    assert_eq!(with_extension(".ogf", ".ogf"), ".ogf.ogf");
    assert!(!has_extension(".ltx", ".ltx"));
  }

  #[test]
  fn normalizes_case_and_separators() {
    assert_eq!(normalize("Configs/System.LTX").expect("valid"), "configs\\system.ltx");
    assert_eq!(normalize("\\configs\\").expect("valid"), "configs");
  }

  #[test]
  fn rejects_traversal_and_empty_components() {
    assert!(normalize("configs\\..\\textures").is_err());
    assert!(normalize("configs\\\\system.ltx").is_err());
    assert!(normalize("").is_err());
  }

  #[test]
  fn joins_either_side_being_empty() {
    assert_eq!(join("configs", "system.ltx").expect("valid"), "configs\\system.ltx");
    assert_eq!(join("", "system.ltx").expect("valid"), "system.ltx");
    assert_eq!(join("configs", "").expect("valid"), "configs");
  }

  #[test]
  fn converts_a_host_relative_path_into_an_identity() {
    assert_eq!(
      normalize_host_relative(&PathBuf::from("Configs").join("System.ltx")).expect("valid"),
      "configs\\system.ltx"
    );
  }

  #[test]
  fn normalizes_a_typed_path_on_construction() {
    let path: XrayPath = XrayPath::new("Configs/System.LTX").expect("valid");

    assert_eq!(path.as_str(), "configs\\system.ltx");
    assert_eq!(path.to_string(), "configs\\system.ltx");
    assert!(XrayPath::new("configs\\..\\system.ltx").is_err());
  }

  #[test]
  fn answers_its_parent_and_name() {
    let nested: XrayPath = XrayPath::new("configs\\weapons\\w_ak74.ltx").expect("valid");

    assert_eq!(nested.file_name(), "w_ak74.ltx");
    assert_eq!(
      nested.parent().map(|parent| parent.as_str().to_string()),
      Some(String::from("configs\\weapons"))
    );

    let top_level: XrayPath = XrayPath::new("system.ltx").expect("valid");

    assert_eq!(top_level.file_name(), "system.ltx");
    assert_eq!(top_level.parent(), None);
  }

  #[test]
  fn matches_an_extension_without_case_and_a_prefix_by_component() {
    let path: XrayPath = XrayPath::new("configs\\system.ltx").expect("valid");

    assert!(path.has_extension(".LTX"));
    assert!(path.is_under("configs").expect("valid prefix"));
    assert!(!path.is_under("configs_backup").expect("valid prefix"));
  }

  #[test]
  fn joins_below_itself() {
    let directory: XrayPath = XrayPath::new("configs\\weapons").expect("valid");

    assert_eq!(
      directory.join("w_ak74.ltx").expect("valid").as_str(),
      "configs\\weapons\\w_ak74.ltx"
    );
  }
}
