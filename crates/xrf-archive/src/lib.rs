//! Packing and unpacking X-Ray database archives.
//!
//! The volume format, the aggregated archive project and its VFS source live in `xrf-vfs`, because a
//! directory and a `.db` volume set are the two sources the engine itself resolves from. This crate is
//! the two directions between such a set and a directory on disk: [`ArchivePacker`] builds volumes from
//! a directory and [`ArchiveUnpacker`] writes them back out to one.
//!
//! Both operations own no state and borrow the project, so neither is a method on it.

#[cfg(test)]
mod asset_source_tests;
pub(crate) mod pack;
pub(crate) mod types;
#[cfg(feature = "typescript-bindings")]
mod typescript_bindings;
pub(crate) mod unpack;

pub use crate::pack::archive_pack_config::{
  ArchivePackConfig, ArchivePackFolder, ArchivePackMode, ArchiveVolumeExtension, VOLUME_SIZE_MAX,
};
pub use crate::pack::archive_pack_result::ArchivePackResult;
pub use crate::pack::archive_packer::ArchivePacker;
#[cfg(feature = "typescript-bindings")]
pub use crate::typescript_bindings::typescript_bindings;
pub use crate::unpack::archive_extract_result::{ArchiveExtractDirectoryResult, ArchiveExtractResult};
pub use crate::unpack::archive_unpack_result::ArchiveUnpackResult;
pub use crate::unpack::archive_unpacker::ArchiveUnpacker;
