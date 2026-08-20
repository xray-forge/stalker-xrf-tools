use std::path::Path;

use xrf_error::XrfResult;

use crate::mount::mount_plan::mount_plan;
use crate::{XrayMountMode, XrayMountPlan, XrayVfs};

/// Opens a mounted VFS for a path, interpreting it the way `mode` says.
///
/// The front door: one call from a path and a mode to something you can resolve against, so a command, the app and an
/// editor do not each assemble it. Use [`XrayMountMode::plan`] with [`mount_plan`] directly when a caller needs to inspect
/// or chain plans first, as layering a loose tree over an installation does.
///
/// Sources that cannot be opened are logged and skipped, so a partly readable installation still resolves what it can.
///
/// # Errors
///
/// Returns an error when the mode cannot plan the path — most often an `fsgame.ltx` that is absent, unreadable or invalid.
pub fn open_vfs(mode: XrayMountMode, path: impl AsRef<Path>) -> XrfResult<XrayVfs> {
  open_plan(&mode.plan(path)?)
}

/// Opens a mounted VFS from a plan already decided.
///
/// The seam for callers that shape the plan first — layering a tree over an installation with [`XrayMountPlan::behind`], or
/// applying ignored prefixes with [`XrayMountPlan::ignoring`] — without reassembling the mounting themselves.
///
/// # Errors
///
/// Returns an error when mounting cannot proceed. A source that fails to open is logged and skipped, so a partly readable
/// installation still resolves what it can.
pub fn open_plan(plan: &XrayMountPlan) -> XrfResult<XrayVfs> {
  let mut vfs: XrayVfs = XrayVfs::new();

  mount_plan(&mut vfs, plan)?;

  Ok(vfs)
}
