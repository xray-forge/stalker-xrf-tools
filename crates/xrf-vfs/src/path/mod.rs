//! X-Ray logical paths: the engine identities every mount, scope and lookup is keyed by.

mod xray_path;
mod xray_path_collision;

pub use xray_path::{XrayPath, is_component_prefix, normalize_logical};
pub(crate) use xray_path::{join, normalize, normalize_base, normalize_host_relative, to_host_relative};
pub use xray_path_collision::XrayPathCollision;
