use crate::archive::archive_file_descriptor::ArchiveFileReplicationDescriptor;
use crate::project::archive_project_unpack_result::ArchiveUnpackResult;
use crate::ArchiveProject;
use minilzo_rs::LZO;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::ErrorKind::AlreadyExists;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use xray_error::XRayResult;
use xray_utils::{assert, assert_equal, assert_not_equal};

impl ArchiveProject {
  pub fn unpack<P: AsRef<Path>>(&self, destination: P) -> XRayResult<ArchiveUnpackResult> {
    let start: Instant = Instant::now();
    let lzo: LZO = LZO::init().unwrap();

    let mut unpacked_files_count: usize = 0;
    let unpacked_files_chunk: usize = max(self.files.len() / 100 * 5, 5);

    // Prepare structure of folders for further unpacking.
    self.unpack_dirs(destination.as_ref())?;

    let prepared_at: Duration = start.elapsed();

    // Unpack each separate file.
    for file_descriptor in self.files.values() {
      if file_descriptor.size_real > 0 {
        Self::unpack_file(&lzo, destination.as_ref(), file_descriptor)?;
      }

      unpacked_files_count += 1;

      if unpacked_files_count % unpacked_files_chunk == 0 {
        log::info!(
          "Unpacked {}/{} files",
          unpacked_files_count,
          self.files.len()
        )
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
  ) -> XRayResult<ArchiveUnpackResult> {
    let start: Instant = Instant::now();

    let mut unpacked_files_count: usize = 0;
    let unpacked_files_chunk: usize = max(self.files.len() / 100 * 5, 5);

    // Prepare structure of folders for further unpacking.
    self.unpack_dirs(destination.as_ref())?;

    let prepared_at: Duration = start.elapsed();

    let mut tasks_set = bounded_join_set::JoinSet::new(concurrency);

    // Unpack each separate file.
    for file_descriptor in self.files.values() {
      if file_descriptor.size_real > 0 {
        let descriptor: ArchiveFileReplicationDescriptor = file_descriptor.clone();
        let destination: PathBuf = destination.as_ref().into();

        tasks_set
          .spawn(async move { Self::unpack_file(&LZO::init().unwrap(), destination, &descriptor) });
      }
    }

    while tasks_set.join_next().await.is_some() {
      unpacked_files_count += 1;

      if unpacked_files_count % unpacked_files_chunk == 0 {
        log::info!(
          "Unpacked {unpacked_files_count} / {} files",
          self.files.len()
        )
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

  fn unpack_file<P: AsRef<Path>>(
    lzo: &LZO,
    destination: P,
    file_descriptor: &ArchiveFileReplicationDescriptor,
  ) -> XRayResult {
    let mut file_path: PathBuf = destination.as_ref().into();

    file_path.push(&file_descriptor.destination);
    file_path.push(&file_descriptor.name);

    let mut source_file: File = File::open(file_descriptor.source.as_path())?;

    source_file
      .seek(SeekFrom::Start(file_descriptor.offset as u64))
      .expect("Expected to be able to seek to start of the source file");

    let mut dest_file: File = File::options()
      .read(false)
      .write(true)
      .create(true)
      .truncate(true)
      .open(file_path)
      .expect("File can not be opened for writing");

    if file_descriptor.size_real != file_descriptor.size_compressed {
      let mut buf: Vec<u8> = vec![0u8; file_descriptor.size_compressed as usize];
      source_file.read_exact(buf.as_mut_slice())?;

      let decompressed_buf: Vec<u8> = lzo
        .decompress_safe(buf.as_slice(), file_descriptor.size_real as usize)
        .expect("Valid LZO data");

      let actual_crc: u32 = crc32fast::hash(decompressed_buf.as_slice());

      assert_equal(file_descriptor.crc, actual_crc, "CRCs do not match")?;

      dest_file
        .write_all(decompressed_buf.as_slice())
        .expect("Unable to write to dest file");
    } else {
      let mut remaining_bytes: usize = file_descriptor.size_real as usize;
      let mut buf: Vec<u8> = vec![0u8; min(256 * 1024, remaining_bytes)];

      while remaining_bytes > 0 {
        let to_read: usize = min(buf.len(), remaining_bytes);
        let read: usize = source_file.read(&mut buf[..to_read])?;

        assert(
          read <= remaining_bytes,
          "Must not read more bytes than remaining",
        )?;
        assert_not_equal(read, 0, "Unexpected End Of File")?;

        let written: usize = dest_file
          .write(&buf[..read])
          .expect("Unable to write to destination file");

        remaining_bytes -= read;

        assert_not_equal(written, 0, "Unable to write bytes")?;
      }
    }

    dest_file.set_len(file_descriptor.size_real as u64)?;

    Ok(())
  }

  fn unpack_dirs<P: AsRef<Path>>(&self, destination: P) -> XRayResult {
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
