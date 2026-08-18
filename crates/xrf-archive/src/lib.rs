//! Reading and writing X-Ray database archives.
//!
//! Layered from the format outward: `archive` parses one volume, `project` aggregates a set of them into
//! one addressable tree, and `pack` and `unpack` are the two directions between that tree and a
//! directory on disk.
//!
//! [`ArchiveProject`] answers what an archive set holds, including reading an entry into memory.
//! [`ArchivePacker`] builds volumes from a directory and [`ArchiveUnpacker`] writes them back out to one.
//! The two operations own no state and borrow the project, so neither is a method on it.

pub(crate) mod archive;
pub(crate) mod pack;
pub(crate) mod project;
pub(crate) mod types;
#[cfg(feature = "typescript-bindings")]
mod typescript_bindings;
pub(crate) mod unpack;

pub use crate::archive::archive_descriptor::ArchiveDescriptor;
pub use crate::archive::archive_file_descriptor::ArchiveFileDescriptor;
pub use crate::pack::archive_pack_config::{
  ArchivePackConfig, ArchivePackFolder, ArchivePackMode, ArchiveVolumeExtension, VOLUME_SIZE_MAX,
};
pub use crate::pack::archive_pack_result::ArchivePackResult;
pub use crate::pack::archive_packer::ArchivePacker;
pub use crate::project::archive_asset_source::ArchiveAssetSource;
pub use crate::project::archive_project::ArchiveProject;
pub use crate::project::archive_project_read_policy::ArchiveProjectReadPolicy;
pub use crate::project::archive_project_read_result::ProjectReadResult;
#[cfg(feature = "typescript-bindings")]
pub use crate::typescript_bindings::typescript_bindings;
pub use crate::unpack::archive_extract_result::{ArchiveExtractDirectoryResult, ArchiveExtractResult};
pub use crate::unpack::archive_unpack_result::ArchiveUnpackResult;
pub use crate::unpack::archive_unpacker::ArchiveUnpacker;
