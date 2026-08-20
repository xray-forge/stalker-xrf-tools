//! Composing sources into a searchable order: what to mount, where, and how a path is turned into a plan.

pub(crate) mod mount_plan;
mod open;
pub(crate) mod plan;
mod xray_mount;
mod xray_mount_mode;
mod xray_root;

pub use mount_plan::mount_plan;
pub use open::{open_plan, open_vfs};
pub use plan::{XrayMountPlan, XrayPlannedMount};
pub use xray_mount::{XrayMount, XrayMountId};
pub use xray_mount_mode::XrayMountMode;
pub use xray_root::implied_asset_root;
