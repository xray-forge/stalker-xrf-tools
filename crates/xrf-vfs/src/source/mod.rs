//! Mountable asset sources, and the indexing a directory source needs.
//!
//! [`XrayAssetSource`] is the seam: a source for a format this crate does not own belongs beside that format and
//! implements the trait. The directory index behind [`XrayDirectorySource`] is deliberately private — resolution belongs to
//! [`crate::XrayVfs`] alone.

mod archive_asset_source;
pub(crate) mod directory_asset;
pub(crate) mod directory_asset_index;
pub(crate) mod indexed_asset;
pub(crate) mod xray_asset_index;
mod xray_asset_source;
pub(crate) mod xray_directory_source;

pub use archive_asset_source::ArchiveAssetSource;
pub(crate) use directory_asset::DirectoryAsset;
pub(crate) use directory_asset_index::DirectoryAssetIndex;
pub(crate) use indexed_asset::IndexedAsset;
pub(crate) use xray_asset_index::XrayAssetIndex;
pub use xray_asset_source::{XrayAssetSource, XrayMountKind};
pub(crate) use xray_directory_source::XrayDirectorySource;
