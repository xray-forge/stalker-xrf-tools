use xrf_error::XrfResult;

use crate::project::archive_asset_source::ArchiveAssetSource;
use crate::{XrayDirectorySource, XrayMountId, XrayMountKind, XrayMountPlan, XrayPlannedMount, XrayVfs};

/// Mounts each planned source that can be opened, in plan order.
///
/// Planning is a decision about the filesystem; this is the construction of the sources it named.
///
/// Sources that fail to open or mount are logged and omitted. The returned mount IDs preserve plan order.
pub fn mount_plan(vfs: &mut XrayVfs, plan: &XrayMountPlan) -> XrfResult<Vec<XrayMountId>> {
  let mut mounted: Vec<XrayMountId> = Vec::with_capacity(plan.len());

  for planned in plan.mounts() {
    match mount_one(vfs, planned) {
      Ok(id) => mounted.push(id),
      Err(error) => log::warn!(
        "Skipping planned mount {} at {}: {error}",
        planned.origin,
        planned.path.display()
      ),
    }
  }

  Ok(mounted)
}

fn mount_one(vfs: &mut XrayVfs, planned: &XrayPlannedMount) -> XrfResult<XrayMountId> {
  match planned.kind {
    XrayMountKind::Archive => vfs.mount(&planned.base, Box::new(ArchiveAssetSource::read(&planned.path)?)),
    XrayMountKind::Directory => vfs.mount(
      &planned.base,
      Box::new(XrayDirectorySource::read_ignoring(&planned.path, &planned.ignored)?),
    ),
  }
}
