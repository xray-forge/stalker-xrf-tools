use std::path::PathBuf;

/// A source a plan named that could not be opened, and why.
///
/// Mounting is deliberately tolerant — a corrupt volume or an unreadable directory must not stop a tool from opening the
/// rest of an installation. That tolerance is only honest if the omission is visible: a check that enumerates a mount
/// which never opened reports its assets as missing, which reads as a content problem rather than the read failure it is.
/// So every skip is retained on the [`crate::XrayVfs`] for callers to surface.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XraySkippedMount {
  /// Where the source that failed to open lives.
  pub path: PathBuf,
  /// How the plan described it, such as an `fsgame.ltx` alias.
  pub origin: String,
  /// Why it could not be opened, rendered for a person.
  pub reason: String,
}
