//! Composing sources into a searchable order: what to mount, where, and how a path is turned into a plan.

mod mount_plan;
mod open;
mod skipped_mount;
#[cfg(test)]
mod tests;
mod xray_mount;
mod xray_mount_mode;
mod xray_mount_plan;
mod xray_root;

pub use mount_plan::mount_plan;
pub use open::{open_plan, open_vfs};
pub use skipped_mount::XraySkippedMount;
pub use xray_mount::{XrayMount, XrayMountId};
pub use xray_mount_mode::XrayMountMode;
pub use xray_mount_plan::{XrayMountPlan, XrayPlannedMount};
pub use xray_root::find_implied_asset_root;
