use std::path::Path;

use xrf_archive::mount_plan;
use xrf_assets::{FSGAME_FILE_NAME, XrayMountPlan, XrayVfs};
use xrf_error::XrfResult;

/// Mounts a game installation's declared sources, or returns `None` when `path` is not one.
///
/// Both LTX commands treat only a path directly holding `fsgame.ltx` as an installation, and share the rule from here so
/// they cannot drift. A named directory stays a directory: widening `--path <install>\gamedata\configs` to the whole game
/// would read or rewrite thousands of configs nobody asked about.
///
/// # Errors
///
/// Returns an error when `fsgame.ltx` is present but cannot be read, decoded, or parsed, or when a declared source cannot
/// be mounted.
pub fn mount_installation(path: &Path) -> XrfResult<Option<XrayVfs>> {
  if !path.join(FSGAME_FILE_NAME).is_file() {
    return Ok(None);
  }

  let mut vfs: XrayVfs = XrayVfs::new();

  mount_plan(&mut vfs, &XrayMountPlan::from_fsgame(path)?)?;

  Ok(Some(vfs))
}
