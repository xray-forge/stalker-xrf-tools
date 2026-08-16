use std::io;
use std::io::Write;

use byteorder::{ByteOrder, WriteBytesExt};
use xrf_error::{XrfError, XrfResult};

#[derive(Default)]
pub struct ChunkWriter {
  pub buffer: Vec<u8>,
}

impl ChunkWriter {
  pub fn new() -> Self {
    Self::default()
  }

  /// Flush all the written data as chunk into the writable object.
  pub fn flush_chunk_into<T: ByteOrder>(&mut self, destination: &mut dyn Write, id: u32) -> XrfResult<usize> {
    let payload_size: u32 = u32::try_from(self.buffer.len())
      .map_err(|_| XrfError::new_invalid_error("Chunk payload exceeds the u32 format limit"))?;

    destination.write_u32::<T>(id)?;
    destination.write_u32::<T>(payload_size)?;
    destination.write_all(&self.buffer)?;

    Ok(self.buffer.len())
  }

  /// Flush all the written data as raw buffer into writable.
  pub fn flush_raw_into(&mut self, file: &mut dyn Write) -> XrfResult {
    Ok(file.write_all(&self.buffer)?)
  }

  /// Flush all the written data as chunk into the file.
  pub fn flush_chunk_into_buffer<T: ByteOrder>(&mut self, id: u32) -> XrfResult<Vec<u8>> {
    let capacity: usize = self
      .buffer
      .len()
      .checked_add(size_of::<u32>() * 2)
      .ok_or_else(|| XrfError::new_invalid_error("Framed chunk size exceeds the platform limit"))?;

    let mut buffer: Vec<u8> = Vec::with_capacity(capacity);

    let payload_size: u32 = u32::try_from(self.buffer.len())
      .map_err(|_| XrfError::new_invalid_error("Chunk payload exceeds the u32 format limit"))?;

    buffer.write_u32::<T>(id)?;
    buffer.write_u32::<T>(payload_size)?;
    buffer.write_all(&self.buffer)?;

    Ok(buffer)
  }

  /// Flush all the written data as chunk into the file.
  pub fn flush_raw_into_buffer(&mut self) -> XrfResult<Vec<u8>> {
    let mut buffer: Vec<u8> = Vec::with_capacity(self.buffer.len());

    buffer.write_all(&self.buffer)?;

    Ok(buffer)
  }

  /// Get count of bytes written into internal buffer.
  pub fn bytes_written(&self) -> usize {
    self.buffer.len()
  }
}

impl Write for ChunkWriter {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    self.buffer.write(buf)
  }

  fn flush(&mut self) -> io::Result<()> {
    self.buffer.flush()
  }
}

#[cfg(test)]
mod tests {
  use std::io::{Result as IoResult, Write};

  use xrf_error::XrfResult;

  use super::ChunkWriter;
  use crate::XRayByteOrder;

  struct ShortWriter {
    bytes: Vec<u8>,
    max_write: usize,
  }

  impl Write for ShortWriter {
    fn write(&mut self, buffer: &[u8]) -> IoResult<usize> {
      let written: usize = buffer.len().min(self.max_write);

      self.bytes.extend_from_slice(&buffer[..written]);

      Ok(written)
    }

    fn flush(&mut self) -> IoResult<()> {
      Ok(())
    }
  }

  #[test]
  fn flush_chunk_writes_the_complete_payload_to_short_writers() -> XrfResult {
    let mut chunk: ChunkWriter = ChunkWriter::new();
    let mut destination = ShortWriter {
      bytes: Vec::new(),
      max_write: 2,
    };

    chunk.write_all(&[1, 2, 3, 4])?;

    assert_eq!(chunk.flush_chunk_into::<XRayByteOrder>(&mut destination, 7)?, 4);
    assert_eq!(destination.bytes, [7, 0, 0, 0, 4, 0, 0, 0, 1, 2, 3, 4]);

    Ok(())
  }
}
