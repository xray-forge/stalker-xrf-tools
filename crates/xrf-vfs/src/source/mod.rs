//! Mountable asset sources, and the indexing a directory source needs.
//!
//! [`XrayAssetSource`] is the seam: a source for a format this crate does not own belongs beside that format and
//! implements the trait. The directory index behind the directory source is deliberately private — resolution belongs to
//! [`crate::XrayVfs`] alone.

mod directory_asset;
mod directory_asset_index;
mod indexed_asset;
mod xray_archive_source;
mod xray_asset_index;
mod xray_asset_source;
mod xray_directory_source;

pub(crate) use directory_asset::DirectoryAsset;
pub(crate) use directory_asset_index::DirectoryAssetIndex;
pub(crate) use indexed_asset::IndexedAsset;
pub use xray_archive_source::XrayArchiveSource;
pub(crate) use xray_asset_index::XrayAssetIndex;
pub use xray_asset_source::{XrayAssetSource, XraySourceKind};
pub(crate) use xray_directory_source::XrayDirectorySource;
