use std::cmp::max;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::ErrorKind::AlreadyExists;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use xrf_error::{XrfError, XrfResult};
use xrf_vfs::ArchiveFileDescriptor;
use xrf_vfs::ArchiveProject;
use xrf_vfs::write_descriptor_contents;

use crate::unpack::archive_extract_result::{ArchiveExtractDirectoryResult, ArchiveExtractResult};
use crate::unpack::archive_unpack_result::ArchiveUnpackResult;

/// Writes the contents of an archive project back out to a directory.
///
/// The mirror of [`crate::ArchivePacker`], and deliberately separate from [`ArchiveProject`]: the project
/// answers what an archive set holds, while this decides what lands on disk. Nothing here mutates the
/// project, so a caller may read from one while unpacking it.
pub struct ArchiveUnpacker;

impl ArchiveUnpacker {
  /// Write every file in the project beneath a destination root.
  pub fn unpack<P: AsRef<Path>>(project: &ArchiveProject, destination: P) -> XrfResult<ArchiveUnpackResult> {
    let start: Instant = Instant::now();

    let mut unpacked_files_count: usize = 0;
    let unpacked_files_chunk: usize = max(project.files.len() / 100 * 5, 5);

    Self::unpack_dirs(project, destination.as_ref())?;

    let prepared_at: Duration = start.elapsed();

    for file_descriptor in project.files.values() {
      if file_descriptor.size_real > 0 {
        Self::unpack_file(destination.as_ref(), file_descriptor)?;
      }

      unpacked_files_count += 1;

      if unpacked_files_count.is_multiple_of(unpacked_files_chunk) {
        log::info!("Unpacked {}/{} files", unpacked_files_count, project.files.len())
      }
    }

    let unpacked_at: Duration = start.elapsed();

    Ok(Self::describe(project, destination.as_ref(), prepared_at, unpacked_at))
  }

  /// Write every file in the project beneath a destination root, up to `concurrency` at a time.
  pub async fn unpack_parallel<P: AsRef<Path>>(
    project: &ArchiveProject,
    destination: P,
    concurrency: usize,
  ) -> XrfResult<ArchiveUnpackResult> {
    let start: Instant = Instant::now();

    let mut unpacked_files_count: usize = 0;
    let unpacked_files_chunk: usize = max(project.files.len() / 100 * 5, 5);

    Self::unpack_dirs(project, destination.as_ref())?;

    let prepared_at: Duration = start.elapsed();

    let mut tasks_set = bounded_join_set::JoinSet::new(concurrency);

    for file_descriptor in project.files.values() {
      if file_descriptor.size_real > 0 {
        let descriptor: ArchiveFileDescriptor = file_descriptor.clone();
        let destination: PathBuf = destination.as_ref().into();

        tasks_set.spawn(async move { Self::unpack_file(destination, &descriptor) });
      }
    }

    while tasks_set.join_next().await.is_some() {
      unpacked_files_count += 1;

      if unpacked_files_count.is_multiple_of(unpacked_files_chunk) {
        log::info!("Unpacked {unpacked_files_count} / {} files", project.files.len())
      }
    }

    let unpacked_at: Duration = start.elapsed();

    Ok(Self::describe(project, destination.as_ref(), prepared_at, unpacked_at))
  }

  /// Write every archived file under one directory to a destination root.
  ///
  /// Keeps the layout below the prefix but not the prefix itself: extracting `configs\gameplay` into
  /// `C:\out` produces `C:\out\dialogs.xml`, not `C:\out\configs\gameplay\dialogs.xml`. The user picked
  /// the destination for the directory they named, so repeating that directory inside it is surprising.
  ///
  /// An empty prefix means the whole archive, which is what selecting the tree root does.
  pub fn extract_directory<P: AsRef<Path>>(
    project: &ArchiveProject,
    prefix: &str,
    destination: P,
  ) -> XrfResult<ArchiveExtractDirectoryResult> {
    let normalized: String = prefix.trim_end_matches(['\\', '/']).to_string();

    let mut extracted_count: usize = 0;
    let mut size: u64 = 0;

    for descriptor in project.files.values() {
      // Archives carry entries that name a directory rather than a file, and entries with no bytes at
      // all. `unpack` skips them; opening one as a file is an operating system error, not a file.
      if descriptor.size_real == 0 || descriptor.name.ends_with(['\\', '/']) {
        continue;
      }

      let Some(relative) = Self::relative_to_prefix(&descriptor.name, &normalized) else {
        continue;
      };

      let target_path: PathBuf = destination.as_ref().join(relative.replace('\\', "/"));

      if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
      }

      write_descriptor_contents(&mut Self::create_target(&target_path)?, descriptor)?;

      extracted_count += 1;
      size += descriptor.size_real as u64;
    }

    if extracted_count == 0 {
      return Err(XrfError::new_not_found_error(format!(
        "Cannot extract '{normalized}' - no files in the archive are under it."
      )));
    }

    Ok(ArchiveExtractDirectoryResult {
      prefix: normalized,
      destination: destination.as_ref().to_string_lossy().into(),
      extracted_count,
      size,
    })
  }

  /// Write one archived file to an exact path of the caller's choosing.
  pub fn extract_file<P: AsRef<Path>>(
    project: &ArchiveProject,
    name: &str,
    destination: P,
  ) -> XrfResult<ArchiveExtractResult> {
    let descriptor: &ArchiveFileDescriptor = project.files.get(name).ok_or_else(|| {
      XrfError::new_not_found_error(format!("Cannot extract '{name}' - no such file in the archive."))
    })?;

    if let Some(parent) = destination.as_ref().parent() {
      fs::create_dir_all(parent)?;
    }

    write_descriptor_contents(&mut Self::create_target(destination.as_ref())?, descriptor)?;

    Ok(ArchiveExtractResult {
      name: descriptor.name.clone(),
      destination: destination.as_ref().to_string_lossy().into(),
      size: descriptor.size_real as u64,
    })
  }

  fn describe(
    project: &ArchiveProject,
    destination: &Path,
    prepared_at: Duration,
    unpacked_at: Duration,
  ) -> ArchiveUnpackResult {
    ArchiveUnpackResult {
      archives: project
        .archives
        .iter()
        .map(|it| it.path.to_str().unwrap().into())
        .collect(),
      destination: destination.to_str().unwrap().into(),
      duration: unpacked_at.as_millis(),
      prepare_duration: prepared_at.as_millis(),
      unpack_duration: unpacked_at.as_millis() - prepared_at.as_millis(),
      unpacked_size: project.get_real_size(),
    }
  }

  /// Path of an archived file relative to a directory prefix, or none when it lies outside it.
  ///
  /// Compared segment-wise rather than by raw `starts_with`, so `configs` does not swallow
  /// `configs_backup\...`.
  fn relative_to_prefix<'a>(name: &'a str, prefix: &str) -> Option<&'a str> {
    if prefix.is_empty() {
      return Some(name);
    }

    if name.len() <= prefix.len() {
      return None;
    }

    let (head, tail) = name.split_at(prefix.len());

    if head.eq_ignore_ascii_case(prefix) && tail.starts_with(['\\', '/']) {
      Some(&tail[1..])
    } else {
      None
    }
  }

  fn create_target(path: &Path) -> XrfResult<File> {
    Ok(
      File::options()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?,
    )
  }

  fn unpack_file<P: AsRef<Path>>(destination: P, descriptor: &ArchiveFileDescriptor) -> XrfResult {
    let mut file_path: PathBuf = destination.as_ref().into();

    file_path.push(&descriptor.destination);
    file_path.push(&descriptor.name);

    write_descriptor_contents(&mut Self::create_target(&file_path)?, descriptor)
  }

  fn unpack_dirs<P: AsRef<Path>>(project: &ArchiveProject, destination: P) -> XrfResult {
    let mut set: HashSet<PathBuf> = HashSet::new();

    for descriptor in project.files.values() {
      set.insert(
        destination
          .as_ref()
          .join(&descriptor.destination)
          .join(&descriptor.name)
          .parent()
          .expect("Unpacked archive dire parent expected")
          .into(),
      );
    }

    for path in set {
      match fs::create_dir_all(path) {
        Ok(_) => {}
        Err(error) if error.kind() == AlreadyExists => {}
        Err(error) => return Err(error.into()),
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::ArchiveUnpacker;

  #[test]
  fn relative_to_prefix_treats_the_prefix_as_whole_segments() {
    // The trap: a raw `starts_with` would pull `configs_backup` into an extraction of `configs`.
    assert_eq!(
      ArchiveUnpacker::relative_to_prefix("configs\\gameplay\\dialogs.xml", "configs"),
      Some("gameplay\\dialogs.xml")
    );
    assert_eq!(
      ArchiveUnpacker::relative_to_prefix("configs_backup\\a.ltx", "configs"),
      None
    );
  }

  #[test]
  fn relative_to_prefix_returns_everything_for_an_empty_prefix() {
    assert_eq!(
      ArchiveUnpacker::relative_to_prefix("configs\\a.ltx", ""),
      Some("configs\\a.ltx")
    );
  }

  #[test]
  fn relative_to_prefix_rejects_the_prefix_itself_and_shorter_names() {
    assert_eq!(ArchiveUnpacker::relative_to_prefix("configs", "configs"), None);
    assert_eq!(ArchiveUnpacker::relative_to_prefix("a.ltx", "configs"), None);
  }

  #[test]
  fn relative_to_prefix_ignores_case_like_the_archives_do() {
    assert_eq!(
      ArchiveUnpacker::relative_to_prefix("Configs\\a.ltx", "configs"),
      Some("a.ltx")
    );
  }
}
