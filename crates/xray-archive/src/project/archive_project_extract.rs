use std::cmp::min;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use minilzo_rs::LZO;
use serde::Serialize;
use xray_error::{XRayError, XRayResult};
use xray_utils::{assert, assert_equal, assert_not_equal};

use crate::ArchiveProject;
use crate::archive::archive_file_descriptor::ArchiveFileDescriptor;

#[cfg_attr(
  feature = "typescript-bindings",
  derive(ts_rs::TS),
  ts(export, export_to = "xray-archive.ts")
)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveExtractResult {
  pub name: String,
  pub destination: String,
  pub size: u64,
}

#[cfg_attr(
  feature = "typescript-bindings",
  derive(ts_rs::TS),
  ts(export, export_to = "xray-archive.ts")
)]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveExtractFolderResult {
  pub prefix: String,
  pub destination: String,
  pub extracted_count: usize,
  pub size: u64,
}

impl ArchiveProject {
  /// Write every archived file under one directory to a destination root.
  ///
  /// Keeps the layout below the prefix but not the prefix itself: extracting `configs\gameplay` into
  /// `C:\out` produces `C:\out\dialogs.xml`, not `C:\out\configs\gameplay\dialogs.xml`. The user picked
  /// the destination for the folder they named, so repeating that folder inside it is surprising.
  ///
  /// An empty prefix means the whole archive, which is what selecting the tree root does.
  pub fn extract_folder<P: AsRef<Path>>(&self, prefix: &str, destination: P) -> XRayResult<ArchiveExtractFolderResult> {
    let normalized: String = prefix.trim_end_matches(['\\', '/']).to_string();
    let lzo: LZO = Self::init_lzo()?;

    let mut extracted_count: usize = 0;
    let mut size: u64 = 0;

    for descriptor in self.files.values() {
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

      let mut target: File = File::options()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(&target_path)?;

      Self::write_file_contents(&lzo, &mut target, descriptor)?;

      extracted_count += 1;
      size += descriptor.size_real as u64;
    }

    if extracted_count == 0 {
      return Err(XRayError::new_not_found_error(format!(
        "Cannot extract '{normalized}' - no files in the archive are under it."
      )));
    }

    Ok(ArchiveExtractFolderResult {
      prefix: normalized,
      destination: destination.as_ref().to_string_lossy().into(),
      extracted_count,
      size,
    })
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

  /// Write one archived file to an exact path of the caller's choosing.
  pub fn extract_file<P: AsRef<Path>>(&self, name: &str, destination: P) -> XRayResult<ArchiveExtractResult> {
    let descriptor: &ArchiveFileDescriptor = self.files.get(name).ok_or_else(|| {
      XRayError::new_not_found_error(format!("Cannot extract '{name}' - no such file in the archive."))
    })?;

    if let Some(parent) = destination.as_ref().parent() {
      fs::create_dir_all(parent)?;
    }

    let mut target: File = File::options()
      .read(false)
      .write(true)
      .create(true)
      .truncate(true)
      .open(destination.as_ref())?;

    Self::write_file_contents(&Self::init_lzo()?, &mut target, descriptor)?;

    Ok(ArchiveExtractResult {
      name: descriptor.name.clone(),
      destination: destination.as_ref().to_string_lossy().into(),
      size: descriptor.size_real as u64,
    })
  }

  /// Read one archived file into memory, decompressing it when it is stored compressed.
  ///
  /// Separate from `write_file_contents`, which streams uncompressed entries straight to disk without
  /// ever holding them whole. Callers that need the bytes themselves - previewing an image, say - have
  /// to accept holding them, so the size guard belongs with them rather than here.
  pub fn read_file_bytes(&self, name: &str) -> XRayResult<Vec<u8>> {
    let descriptor: &ArchiveFileDescriptor = self
      .files
      .get(name)
      .ok_or_else(|| XRayError::new_not_found_error(format!("Cannot read '{name}' - no such file in the archive.")))?;

    let mut source: File = File::open(descriptor.source.as_path())?;

    source.seek(SeekFrom::Start(descriptor.offset as u64))?;

    let mut raw: Vec<u8> = vec![0u8; descriptor.size_compressed as usize];

    source.read_exact(raw.as_mut_slice())?;

    if descriptor.size_real == descriptor.size_compressed {
      return Ok(raw);
    }

    let decompressed: Vec<u8> = Self::init_lzo()?
      .decompress_safe(raw.as_slice(), descriptor.size_real as usize)
      .map_err(|error| {
        XRayError::new_read_error(format!(
          "Failed to decompress '{}' from '{}': {error:?}.",
          descriptor.name,
          descriptor.source.display()
        ))
      })?;

    assert_equal(
      descriptor.crc,
      crc32fast::hash(decompressed.as_slice()),
      "CRCs do not match",
    )?;

    Ok(decompressed)
  }

  /// Build the decompressor, reporting a failure instead of taking the process down with it.
  pub(crate) fn init_lzo() -> XRayResult<LZO> {
    LZO::init().map_err(|error| XRayError::new_unexpected_error(format!("Failed to initialize LZO: {error:?}.")))
  }

  /// Copy one archived file into an already opened target, decompressing when it is stored compressed.
  ///
  /// Shared by whole-archive unpacking and single file extraction so the two cannot drift on CRC
  /// verification or on how uncompressed entries are streamed.
  pub(crate) fn write_file_contents(lzo: &LZO, target: &mut File, descriptor: &ArchiveFileDescriptor) -> XRayResult {
    let mut source: File = File::open(descriptor.source.as_path())?;

    source.seek(SeekFrom::Start(descriptor.offset as u64))?;

    if descriptor.size_real != descriptor.size_compressed {
      let mut buffer: Vec<u8> = vec![0u8; descriptor.size_compressed as usize];

      source.read_exact(buffer.as_mut_slice())?;

      let decompressed: Vec<u8> = lzo
        .decompress_safe(buffer.as_slice(), descriptor.size_real as usize)
        .map_err(|error| {
          XRayError::new_read_error(format!(
            "Failed to decompress '{}' from '{}': {error:?}.",
            descriptor.name,
            descriptor.source.display()
          ))
        })?;

      assert_equal(
        descriptor.crc,
        crc32fast::hash(decompressed.as_slice()),
        "CRCs do not match",
      )?;

      target.write_all(decompressed.as_slice())?;
    } else {
      let mut remaining: usize = descriptor.size_real as usize;
      let mut buffer: Vec<u8> = vec![0u8; min(256 * 1024, remaining.max(1))];

      while remaining > 0 {
        let to_read: usize = min(buffer.len(), remaining);
        let read: usize = source.read(&mut buffer[..to_read])?;

        assert(read <= remaining, "Must not read more bytes than remaining")?;
        assert_not_equal(read, 0, "Unexpected End Of File")?;

        let written: usize = target.write(&buffer[..read])?;

        remaining -= read;

        assert_not_equal(written, 0, "Unable to write bytes")?;
      }
    }

    target.set_len(descriptor.size_real as u64)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::ArchiveProject;

  #[test]
  fn relative_to_prefix_treats_the_prefix_as_whole_segments() {
    // The trap: a raw `starts_with` would pull `configs_backup` into an extraction of `configs`.
    assert_eq!(
      ArchiveProject::relative_to_prefix("configs\\gameplay\\dialogs.xml", "configs"),
      Some("gameplay\\dialogs.xml")
    );
    assert_eq!(
      ArchiveProject::relative_to_prefix("configs_backup\\a.ltx", "configs"),
      None
    );
  }

  #[test]
  fn relative_to_prefix_returns_everything_for_an_empty_prefix() {
    assert_eq!(
      ArchiveProject::relative_to_prefix("configs\\a.ltx", ""),
      Some("configs\\a.ltx")
    );
  }

  #[test]
  fn relative_to_prefix_rejects_the_prefix_itself_and_shorter_names() {
    assert_eq!(ArchiveProject::relative_to_prefix("configs", "configs"), None);
    assert_eq!(ArchiveProject::relative_to_prefix("a.ltx", "configs"), None);
  }

  #[test]
  fn relative_to_prefix_ignores_case_like_the_archives_do() {
    assert_eq!(
      ArchiveProject::relative_to_prefix("Configs\\a.ltx", "configs"),
      Some("a.ltx")
    );
  }
}
