#![doc = include_str!("../README.md")]

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
