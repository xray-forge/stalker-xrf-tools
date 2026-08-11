use std::cmp::min;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use minilzo_rs::LZO;
use serde::Serialize;
use xray_error::{XRayError, XRayResult};
use xray_utils::{assert, assert_equal, assert_not_equal};

use crate::ArchiveProject;
use crate::archive::archive_file_descriptor::ArchiveFileDescriptor;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveExtractResult {
  pub name: String,
  pub destination: String,
  pub size: u64,
}

impl ArchiveProject {
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
