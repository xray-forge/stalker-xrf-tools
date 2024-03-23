use crate::archive::file_descriptor::ArchiveFileDescriptor;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct ArchiveHeader {
  pub archive_path: PathBuf,
  pub output_root_path: PathBuf,
  pub files: HashMap<String, ArchiveFileDescriptor>,
}
