use crate::project::archive_project_constants::{
  ALLOWED_PROJECT_READ_EXTENSIONS, ALLOWED_PROJECT_READ_SIZE,
};
use crate::project::archive_project_read_result::ProjectReadResult;
use crate::ArchiveProject;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use xray_error::{XRayError, XRayResult};

impl ArchiveProject {
  /// Read single file from project as string.
  pub fn read_file_as_string(&self, filename: &str) -> XRayResult<ProjectReadResult> {
    log::info!("Trying to read file from archive: {}", filename);

    if !self.can_read_file(filename) {
      return Err(XRayError::new_read_error(format!(
        "File '{}' cannot be read, file extension is not allowed to be read",
        filename
      )));
    }

    match self.files.get(filename) {
      None => Err(XRayError::new_read_error(format!(
        "File '{}' is not found in the archive project",
        filename
      ))),
      Some(file_descriptor) => {
        if file_descriptor.size_real > ALLOWED_PROJECT_READ_SIZE {
          return Err(XRayError::new_read_error(format!(
            "File '{}' is too big to be read - {}, {} is maximum allowed",
            filename, file_descriptor.size_real, ALLOWED_PROJECT_READ_SIZE
          )));
        } else if file_descriptor.size_real != file_descriptor.size_compressed {
          return Err(XRayError::new_read_error(format!(
            "File '{}' is compressed, reading compressed files is not supported yet",
            filename
          )));
        }

        let mut source_file: File = File::open(file_descriptor.source.as_path())?;
        let mut buf: Vec<u8> = vec![0u8; file_descriptor.size_real as usize];

        source_file
          .seek(SeekFrom::Start(file_descriptor.offset as u64))
          .expect("Expected to be able to seek to start of the source file");

        source_file.read_exact(&mut buf)?;

        Ok(ProjectReadResult::new(
          filename,
          &String::from_utf8_lossy(&buf),
          file_descriptor.size_real,
        ))
      }
    }
  }

  pub fn can_read_file(&self, filename: &str) -> bool {
    if let Some(extension) = PathBuf::from(filename).extension() {
      for allowed in ALLOWED_PROJECT_READ_EXTENSIONS {
        if (*allowed).eq(extension) {
          return true;
        }
      }
    }

    false
  }
}
