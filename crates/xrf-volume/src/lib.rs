//! The X-Ray `.db`/`.xdb` archive volume format: headers, entry descriptors, and moving payloads in and out.
//!
//! This crate answers how a volume set is encoded — [`xrf-vfs`] mounts one as an asset source, and `xrf-archive`
//! packs and unpacks them. Neither could own the format without the other reaching into it, so it lives below both.
//!
//! [`ArchiveProject`] is the entry point: it merges a volume set into one name table with the later volume winning,
//! matching how the engine registers archives.
//!
//! [`xrf-vfs`]: https://github.com/xray-forge/stalker-xrf-tools

mod archive_descriptor;
mod archive_file_descriptor;
mod archive_header;
mod byte_order;
mod constants;
mod file_io;
mod project;
mod reader;

pub use archive_descriptor::ArchiveDescriptor;
pub use archive_file_descriptor::ArchiveFileDescriptor;
pub use constants::CHUNK_ID_COMPRESSED_MASK;
pub use file_io::write_descriptor_contents;
pub use project::{ArchiveProject, ArchiveProjectReadPolicy, ProjectReadResult};
