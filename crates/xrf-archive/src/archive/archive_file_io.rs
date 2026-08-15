use std::cmp::min;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

use minilzo_rs::LZO;
use xrf_error::{XrfError, XrfResult};
use xrf_utils::{assert, assert_equal, assert_not_equal};

use crate::archive::archive_file_descriptor::ArchiveFileDescriptor;

/// Bytes held in memory at a time while copying a stored entry out.
const COPY_BUFFER_SIZE: usize = 256 * 1024;

/// Build the decompressor, reporting a failure instead of taking the process down with it.
pub(crate) fn init_lzo() -> XrfResult<LZO> {
  LZO::init().map_err(|error| XrfError::new_unexpected_error(format!("Failed to initialize LZO: {error:?}.")))
}

/// Read one archived entry into memory, decompressing it when it is stored compressed.
///
/// The caller holds the whole entry, so a caller that cannot afford to should use
/// [`write_descriptor_contents`] instead, which streams a stored entry straight through.
pub(crate) fn read_descriptor_bytes(descriptor: &ArchiveFileDescriptor) -> XrfResult<Vec<u8>> {
  let mut source: File = open_at_descriptor(descriptor)?;
  let mut raw: Vec<u8> = vec![0u8; descriptor.size_compressed as usize];

  source.read_exact(raw.as_mut_slice())?;

  // Equal sizes are how the format says "stored", so there is nothing to decompress.
  if descriptor.size_real == descriptor.size_compressed {
    return Ok(raw);
  }

  decompress_descriptor(&init_lzo()?, &raw, descriptor)
}

/// Copy one archived entry into an already opened target, decompressing when it is stored compressed.
///
/// Shared by whole-archive unpacking and single file extraction so the two cannot drift on CRC
/// verification or on how stored entries are streamed.
pub(crate) fn write_descriptor_contents(lzo: &LZO, target: &mut File, descriptor: &ArchiveFileDescriptor) -> XrfResult {
  let mut source: File = open_at_descriptor(descriptor)?;

  if descriptor.size_real != descriptor.size_compressed {
    let mut raw: Vec<u8> = vec![0u8; descriptor.size_compressed as usize];

    source.read_exact(raw.as_mut_slice())?;
    target.write_all(&decompress_descriptor(lzo, &raw, descriptor)?)?;
  } else {
    // A stored entry can be arbitrarily large, so it goes through a fixed buffer rather than memory.
    let mut remaining: usize = descriptor.size_real as usize;
    let mut buffer: Vec<u8> = vec![0u8; min(COPY_BUFFER_SIZE, remaining.max(1))];

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

/// Open the volume holding an entry, positioned at its payload.
fn open_at_descriptor(descriptor: &ArchiveFileDescriptor) -> XrfResult<File> {
  let mut source: File = File::open(descriptor.source.as_path())?;

  source.seek(SeekFrom::Start(descriptor.offset as u64))?;

  Ok(source)
}

/// Decompress an entry's payload and verify it against the checksum the archive recorded.
fn decompress_descriptor(lzo: &LZO, raw: &[u8], descriptor: &ArchiveFileDescriptor) -> XrfResult<Vec<u8>> {
  let decompressed: Vec<u8> = lzo
    .decompress_safe(raw, descriptor.size_real as usize)
    .map_err(|error| {
      XrfError::new_read_error(format!(
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
