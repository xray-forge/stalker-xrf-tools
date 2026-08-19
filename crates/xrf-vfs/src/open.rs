use std::path::Path;

use xrf_error::XrfResult;

use crate::project::plan_mount::mount_plan;
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
  let plan: XrayMountPlan = mode.plan(path)?;
  let mut vfs: XrayVfs = XrayVfs::new();

  mount_plan(&mut vfs, &plan)?;

  Ok(vfs)
}
