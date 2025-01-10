use crate::project::project_constants::{
  ALLOWED_PROJECT_READ_EXTENSIONS, ALLOWED_PROJECT_READ_SIZE,
};
use crate::project::project_read_result::ProjectReadResult;
use crate::{
  ArchiveError, ArchiveProject, ArchiveResult, ARCHIVE_READ_ERROR_INVALID_FORMAT,
  ARCHIVE_READ_ERROR_NOT_FOUND,
};
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

impl ArchiveProject {
  /// Read single file from project as string.
  pub fn read_file_as_string(&self, filename: &str) -> ArchiveResult<ProjectReadResult> {
    log::info!("Trying to read file from archive: {filename}");

    if !self.can_read_file(filename) {
      return Err(ArchiveError::new_read_error_with_code(
        ARCHIVE_READ_ERROR_INVALID_FORMAT,
        format!("File '{filename}' cannot be read, extension is not allowed to be read"),
      ));
    }

    return match self.files.get(filename) {
      None => Err(ArchiveError::new_read_error_with_code(
        ARCHIVE_READ_ERROR_NOT_FOUND,
        format!("File '{filename}' is not found in the archive project"),
      )),
      Some(file_descriptor) => {
        if file_descriptor.size_real > ALLOWED_PROJECT_READ_SIZE {
          return Err(ArchiveError::new_read_error_with_code(
            ARCHIVE_READ_ERROR_NOT_FOUND,
            format!("File '{filename}' is too big to be read - {}, {ALLOWED_PROJECT_READ_SIZE} is maximum allowed", file_descriptor.size_real),
          ));
        } else if file_descriptor.size_real != file_descriptor.size_compressed {
          return Err(
            ArchiveError::new_read_error_with_code(
              ARCHIVE_READ_ERROR_INVALID_FORMAT,
              format!(
                "File '{filename}' is compressed, reading compressed files is not supported yet"
              ),
            )
            .into(),
          );
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
    };
  }

  pub fn can_read_file(&self, filename: &str) -> bool {
    let path: PathBuf = PathBuf::from(filename);
    let extension: Option<&OsStr> = path.extension();

    if let Some(extension) = extension {
      for allowed in ALLOWED_PROJECT_READ_EXTENSIONS {
        if (*allowed).eq(extension) {
          return true;
        }
      }
    }

    false
  }
}
