use std::collections::HashMap;
use std::path::PathBuf;

use crate::archive::archive_file_descriptor::ArchiveFileDescriptor;

pub struct ArchiveHeader {
  pub archive_path: PathBuf,
  pub output_root_path: PathBuf,
  pub files: HashMap<String, ArchiveFileDescriptor>,
}
