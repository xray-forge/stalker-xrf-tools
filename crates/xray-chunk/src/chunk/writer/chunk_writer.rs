use crate::ChunkResult;
use byteorder::{ByteOrder, WriteBytesExt};
use std::io;
use std::io::Write;

#[derive(Default)]
pub struct ChunkWriter {
  pub buffer: Vec<u8>,
}

impl ChunkWriter {
  pub fn new() -> Self {
    Self::default()
  }

  /// Flush all the written data as chunk into the writable object.
  pub fn flush_chunk_into<T: ByteOrder>(
    &mut self,
    destination: &mut dyn Write,
    id: u32,
  ) -> ChunkResult<usize> {
    destination.write_u32::<T>(id)?;
    destination.write_u32::<T>(self.buffer.len() as u32)?;

    Ok(destination.write(&self.buffer)?)
  }

  /// Flush all the written data as raw buffer into writable.
  pub fn flush_raw_into(&mut self, file: &mut dyn Write) -> ChunkResult {
    Ok(file.write_all(&self.buffer)?)
  }

  /// Flush all the written data as chunk into the file.
  pub fn flush_chunk_into_buffer<T: ByteOrder>(&mut self, id: u32) -> ChunkResult<Vec<u8>> {
    let mut buffer: Vec<u8> = Vec::with_capacity(size_of::<u32>() * 2 + self.buffer.len());

    buffer.write_u32::<T>(id)?;
    buffer.write_u32::<T>(self.buffer.len() as u32)?;
    buffer.write_all(&self.buffer)?;

    Ok(buffer)
  }

  /// Flush all the written data as chunk into the file.
  pub fn flush_raw_into_buffer(&mut self) -> ChunkResult<Vec<u8>> {
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
