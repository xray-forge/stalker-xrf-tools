//! What a resolved asset is, and the per-kind rules for turning an engine reference into one.

pub mod shader;
pub mod sound;
mod xray_asset;
mod xray_asset_type;

pub use xray_asset::{XrayAsset, XrayAssetContainer};
pub use xray_asset_type::{XrayAssetRules, XrayAssetType};
