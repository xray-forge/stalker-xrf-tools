use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions, Permissions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use xrf_error::{XrfError, XrfResult};

static STAGED_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Replace an existing file only after its complete contents have been staged beside it.
pub(crate) fn write_file_staged(path: &Path, contents: &[u8]) -> XrfResult {
  let sequence: u64 = STAGED_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
  let staged_path: PathBuf = sibling_path(path, "xrf-tmp", sequence)?;

  let result: XrfResult = (|| -> XrfResult {
    let permissions: Permissions = fs::metadata(path)?.permissions();
    let mut staged_file: File = OpenOptions::new().write(true).create_new(true).open(&staged_path)?;

    staged_file.write_all(contents)?;
    staged_file.flush()?;
    staged_file.set_permissions(permissions)?;
    drop(staged_file);

    replace_staged_file(&staged_path, path, sequence)
  })();

  if result.is_err() {
    let _ = fs::remove_file(&staged_path);
  }

  result
}

fn sibling_path(path: &Path, suffix: &str, sequence: u64) -> XrfResult<PathBuf> {
  let parent: &Path = path
    .parent()
    .ok_or_else(|| XrfError::new_invalid_error(format!("File has no parent directory: {}", path.display())))?;
  let file_name: &OsStr = path
    .file_name()
    .ok_or_else(|| XrfError::new_invalid_error(format!("File has no name: {}", path.display())))?;

  Ok(parent.join(format!(
    ".{}.{}-{}-{sequence}",
    file_name.to_string_lossy(),
    suffix,
    std::process::id(),
  )))
}

#[cfg(not(windows))]
fn replace_staged_file(staged_path: &Path, path: &Path, _sequence: u64) -> XrfResult {
  Ok(fs::rename(staged_path, path)?)
}

#[cfg(windows)]
fn replace_staged_file(staged_path: &Path, path: &Path, sequence: u64) -> XrfResult {
  let backup_path: PathBuf = sibling_path(path, "xrf-backup", sequence)?;

  fs::rename(path, &backup_path)?;

  if let Err(replace_error) = fs::rename(staged_path, path) {
    return match fs::rename(&backup_path, path) {
      Ok(()) => Err(replace_error.into()),
      Err(restore_error) => Err(XrfError::new_io_error(
        format!(
          "Failed to replace '{}': {replace_error}; failed to restore '{}': {restore_error}",
          path.display(),
          backup_path.display(),
        ),
        restore_error.kind(),
      )),
    };
  }

  if let Err(error) = fs::remove_file(&backup_path) {
    log::warn!(
      "Replaced '{}', but failed to remove backup '{}': {error}",
      path.display(),
      backup_path.display(),
    );
  }

  Ok(())
}

#[cfg(test)]
mod tests;
