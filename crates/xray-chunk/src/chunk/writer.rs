use crate::{ChunkResult, U32Bytes};
use byteorder::{ByteOrder, WriteBytesExt};
use encoding_rs::WINDOWS_1251;
use std::borrow::Cow;
use std::io;
use std::io::Write;

pub struct ChunkWriter {
  pub buffer: Vec<u8>,
}

impl ChunkWriter {
  pub fn new() -> ChunkWriter {
    ChunkWriter { buffer: Vec::new() }
  }

  /// Flush all the written data as chunk into the writable object.
  pub fn flush_chunk_into<T: ByteOrder>(
    &mut self,
    file: &mut dyn Write,
    index: u32,
  ) -> ChunkResult<usize> {
    self.buffer.flush()?;

    file.write_u32::<T>(index)?;
    file.write_u32::<T>(self.buffer.len() as u32)?;

    Ok(file.write(self.buffer.as_slice())?)
  }

  /// Flush all the written data as raw buffer into writable.
  pub fn flush_raw_into(&mut self, file: &mut dyn Write) -> ChunkResult {
    self.buffer.flush()?;

    Ok(file.write_all(self.buffer.as_slice())?)
  }

  /// Flush all the written data as chunk into the file.
  pub fn flush_chunk_into_buffer<T: ByteOrder>(&mut self, index: usize) -> ChunkResult<Vec<u8>> {
    self.buffer.flush()?;

    let mut buffer: Vec<u8> = Vec::new();

    buffer.write_u32::<T>(index as u32)?;
    buffer.write_u32::<T>(self.buffer.len() as u32)?;
    buffer.write_all(self.buffer.as_slice())?;

    Ok(buffer)
  }

  /// Flush all the written data as chunk into the file.
  pub fn flush_raw_into_buffer(&mut self) -> ChunkResult<Vec<u8>> {
    self.buffer.flush()?;

    let mut buffer: Vec<u8> = Vec::new();

    buffer.write_all(self.buffer.as_slice())?;

    Ok(buffer)
  }

  /// Get count of bytes written into internal buffer.
  pub fn bytes_written(&self) -> usize {
    self.buffer.len()
  }

  /// Write null terminated windows encoded string.
  pub fn write_null_terminated_win_string(&mut self, value: &str) -> ChunkResult<usize> {
    let (transformed, _, had_errors) = WINDOWS_1251.encode(value);

    if had_errors {
      panic!("Unexpected errors when encoding windows-1251 string data");
    }

    // Try with windows 1251 conversion:
    let value = match transformed {
      Cow::Borrowed(value) => value.to_vec(),
      Cow::Owned(value) => value,
    };

    Ok(self.write(&value)? + self.write(&[0u8])?)
  }
  /// Write 4 bytes value as 4 separate byte entries.
  pub fn write_u32_bytes(&mut self, value: &U32Bytes) -> ChunkResult {
    self.write_u8(value.0)?;
    self.write_u8(value.1)?;
    self.write_u8(value.2)?;
    self.write_u8(value.3)?;

    Ok(())
  }

  /// Write serialized vector into vector, where u32 count N is followed by N u16 entries.
  pub fn write_u16_vector<T: ByteOrder>(&mut self, vector: &[u16]) -> ChunkResult {
    self.write_u32::<T>(vector.len() as u32)?;

    for it in vector {
      self.write_u16::<T>(*it)?;
    }

    Ok(())
  }
}

impl Default for ChunkWriter {
  fn default() -> Self {
    ChunkWriter::new()
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
