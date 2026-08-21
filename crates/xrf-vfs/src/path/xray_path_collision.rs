use std::path::PathBuf;

use crate::path::XrayPath;

/// Two files in one source claiming the same engine identity.
///
/// An authoring error rather than shadowing: shadowing is what happens *between* mounts, where a loose file legitimately
/// overrides an archived one. Inside one source there is no priority to appeal to, so one file simply cannot be reached.
///
/// Reported rather than fatal, because a tool must be able to open a project and say what is wrong with it — an editor
/// cannot refuse to load a mod because one texture is authored twice. A consumer that treats a project as invalid decides
/// that for itself.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XrayPathCollision {
  /// Engine identity both files normalize to.
  pub logical_path: XrayPath,
  /// File the source resolves, being the first one indexed.
  pub kept: PathBuf,
  /// File no lookup can reach, because `kept` already claims its identity.
  pub unreachable: PathBuf,
}
