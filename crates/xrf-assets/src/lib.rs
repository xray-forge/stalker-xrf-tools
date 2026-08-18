//! Indexes X-Ray assets and layers their physical sources in a virtual file system.

mod directory_asset;
mod directory_asset_index;
pub mod shader;
pub mod texture;
#[cfg(feature = "typescript-bindings")]
mod typescript_bindings;
mod xray_asset;
mod xray_asset_index;
mod xray_asset_location;
mod xray_asset_source;
mod xray_asset_type;
pub(crate) mod xray_asset_utils;
mod xray_directory_source;
mod xray_mount;
pub mod xray_path;
mod xray_root;
mod xray_scope;
mod xray_vfs;

#[cfg(feature = "typescript-bindings")]
pub use crate::typescript_bindings::typescript_bindings;
pub use directory_asset::DirectoryAsset;
pub use directory_asset_index::DirectoryAssetIndex;
pub use xray_asset::XrayAsset;
pub use xray_asset_index::XrayAssetIndex;
pub use xray_asset_location::{XrayAssetContainer, XrayAssetLocation};
pub use xray_asset_source::{XrayAssetSource, XrayMountKind};
pub use xray_asset_type::XrayAssetType;
pub use xray_directory_source::XrayDirectorySource;
pub use xray_mount::{XrayMount, XrayMountId};
pub use xray_root::{MESHES_DIRECTORY, TEXTURES_DIRECTORY, implied_asset_root};
pub use xray_scope::{XrayMountSelection, XrayScope};
pub use xray_vfs::XrayVfs;
