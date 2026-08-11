use xray_error::{XRayError, XRayResult};

use crate::ArchiveProject;
use crate::archive::archive_file_descriptor::ArchiveFileDescriptor;
use crate::project::archive_project_read_result::ProjectReadResult;

impl ArchiveProject {
  /// Read single file from project as string.
  pub fn read_file_as_string(&self, filename: &str) -> XRayResult<ProjectReadResult> {
    log::info!("Trying to read file from archive: {}", filename);

    if !self.read_policy.supports_file(filename) {
      return Err(XRayError::new_read_error(format!(
        "File '{}' cannot be read, file extension is not allowed to be read",
        filename
      )));
    }

    let descriptor: &ArchiveFileDescriptor = self
      .files
      .get(filename)
      .ok_or_else(|| XRayError::new_read_error(format!("File '{}' is not found in the archive project", filename)))?;

    if descriptor.size_real > self.read_policy.maximum_size {
      return Err(XRayError::new_read_error(format!(
        "File '{}' is too big to be read - {}, {} is maximum allowed",
        filename, descriptor.size_real, self.read_policy.maximum_size
      )));
    }

    let bytes: Vec<u8> = self.read_file_bytes(filename)?;

    Ok(ProjectReadResult::new(
      filename,
      &String::from_utf8_lossy(&bytes),
      descriptor.size_real,
    ))
  }
}
