//! Reusable physical and X-Ray-aware asset indexes.

mod directory_asset;
mod directory_asset_index;
pub mod shader;
pub mod texture;
#[cfg(feature = "typescript-bindings")]
mod typescript_bindings;
mod xray_asset;
mod xray_asset_index;
mod xray_asset_location;
mod xray_asset_type;
mod xray_asset_utils;
pub mod xray_path;
mod xray_root;
mod xray_vfs;

#[cfg(feature = "typescript-bindings")]
pub use crate::typescript_bindings::typescript_bindings;
pub use directory_asset::DirectoryAsset;
pub use directory_asset_index::DirectoryAssetIndex;
pub use xray_asset::XrayAsset;
pub use xray_asset_index::XrayAssetIndex;
pub use xray_asset_location::XrayAssetLocation;
pub use xray_asset_type::XrayAssetType;
pub use xray_root::{MESHES_DIRECTORY, TEXTURES_DIRECTORY, implied_asset_root};
pub use xray_vfs::XrayVfs;
