//! The `.db` volume format the archive source is built on: headers, entry descriptors, and reading payloads out.
//!
//! Everything the rest of the crate needs is re-exported here, so nothing reaches past this module. Packing and
//! unpacking volumes belongs to `xrf-archive`, which is tooling over this; the two items it needs are re-exported from
//! the crate root.

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
