use std::cmp::max;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::ErrorKind::AlreadyExists;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use minilzo_rs::LZO;
use xrf_error::XrfResult;

use crate::ArchiveProject;
use crate::archive::archive_file_descriptor::ArchiveFileDescriptor;
use crate::project::archive_project_unpack_result::ArchiveUnpackResult;

impl ArchiveProject {
  pub fn unpack<P: AsRef<Path>>(&self, destination: P) -> XrfResult<ArchiveUnpackResult> {
    let start: Instant = Instant::now();
    let lzo: LZO = Self::init_lzo()?;

    let mut unpacked_files_count: usize = 0;
    let unpacked_files_chunk: usize = max(self.files.len() / 100 * 5, 5);

    // Prepare structure of directories for further unpacking.
    self.unpack_dirs(destination.as_ref())?;

    let prepared_at: Duration = start.elapsed();

    // Unpack each separate file.
    for file_descriptor in self.files.values() {
      if file_descriptor.size_real > 0 {
        Self::unpack_file(&lzo, destination.as_ref(), file_descriptor)?;
      }

      unpacked_files_count += 1;

      if unpacked_files_count.is_multiple_of(unpacked_files_chunk) {
        log::info!("Unpacked {}/{} files", unpacked_files_count, self.files.len())
      }
    }

    let unpacked_at: Duration = start.elapsed();

    Ok(ArchiveUnpackResult {
      archives: self
        .archives
        .iter()
        .map(|it| it.path.to_str().unwrap().into())
        .collect(),
      destination: destination.as_ref().to_str().unwrap().into(),
      duration: unpacked_at.as_millis(),
      prepare_duration: prepared_at.as_millis(),
      unpack_duration: unpacked_at.as_millis() - prepared_at.as_millis(),
      unpacked_size: self.get_real_size(),
    })
  }

  pub async fn unpack_parallel<P: AsRef<Path>>(
    &self,
    destination: P,
    concurrency: usize,
  ) -> XrfResult<ArchiveUnpackResult> {
    let start: Instant = Instant::now();

    let mut unpacked_files_count: usize = 0;
    let unpacked_files_chunk: usize = max(self.files.len() / 100 * 5, 5);

    // Prepare structure of directories for further unpacking.
    self.unpack_dirs(destination.as_ref())?;

    let prepared_at: Duration = start.elapsed();

    let mut tasks_set = bounded_join_set::JoinSet::new(concurrency);

    // Unpack each separate file.
    for file_descriptor in self.files.values() {
      if file_descriptor.size_real > 0 {
        let descriptor: ArchiveFileDescriptor = file_descriptor.clone();
        let destination: PathBuf = destination.as_ref().into();

        tasks_set.spawn(async move { Self::unpack_file(&Self::init_lzo()?, destination, &descriptor) });
      }
    }

    while tasks_set.join_next().await.is_some() {
      unpacked_files_count += 1;

      if unpacked_files_count.is_multiple_of(unpacked_files_chunk) {
        log::info!("Unpacked {unpacked_files_count} / {} files", self.files.len())
      }
    }

    let unpacked_at: Duration = start.elapsed();

    Ok(ArchiveUnpackResult {
      archives: self
        .archives
        .iter()
        .map(|it| it.path.to_str().unwrap().into())
        .collect(),
      destination: destination.as_ref().to_str().unwrap().into(),
      duration: unpacked_at.as_millis(),
      prepare_duration: prepared_at.as_millis(),
      unpack_duration: unpacked_at.as_millis() - prepared_at.as_millis(),
      unpacked_size: self.get_real_size(),
    })
  }

  fn unpack_file<P: AsRef<Path>>(lzo: &LZO, destination: P, file_descriptor: &ArchiveFileDescriptor) -> XrfResult {
    let mut file_path: PathBuf = destination.as_ref().into();

    file_path.push(&file_descriptor.destination);
    file_path.push(&file_descriptor.name);

    let mut dest_file: File = File::options()
      .read(false)
      .write(true)
      .create(true)
      .truncate(true)
      .open(file_path)?;

    Self::write_file_contents(lzo, &mut dest_file, file_descriptor)
  }

  fn unpack_dirs<P: AsRef<Path>>(&self, destination: P) -> XrfResult {
    let mut set: HashSet<PathBuf> = HashSet::new();

    for descriptor in self.files.values() {
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
