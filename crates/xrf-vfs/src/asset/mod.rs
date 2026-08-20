//! What a resolved asset is, and the per-kind rules for turning an engine reference into one.

mod shader;
mod sound;
mod xray_asset;
mod xray_asset_type;

pub use shader::SHADER_LIBRARY_LOGICAL_PATH;
pub use sound::sound_reference_name;
pub use xray_asset::{XrayAsset, XrayAssetContainer};
pub use xray_asset_type::{XrayAssetRules, XrayAssetType};
