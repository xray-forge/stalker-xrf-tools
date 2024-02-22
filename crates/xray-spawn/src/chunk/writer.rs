use crate::types::{U32Bytes, Vector3d};
use byteorder::{ByteOrder, WriteBytesExt};
use std::fs::File;
use std::io;
use std::io::Write;

pub struct ChunkWriter {
  pub buffer: Vec<u8>,
}

impl ChunkWriter {
  pub fn new() -> ChunkWriter {
    ChunkWriter { buffer: Vec::new() }
  }

  /// Flush all the written data as chunk into the file.
  pub fn flush_chunk_into_file<T: ByteOrder>(
    &mut self,
    file: &mut File,
    index: u32,
  ) -> io::Result<usize> {
    self.buffer.flush()?;

    file.write_u32::<T>(index)?;
    file.write_u32::<T>(self.buffer.len() as u32)?;
    file.write(self.buffer.as_slice())
  }

  /// Flush all the written data as chunk into the file.
  pub fn flush_chunk_into_buffer<T: ByteOrder>(&mut self, index: usize) -> io::Result<Vec<u8>> {
    self.buffer.flush()?;

    let mut buffer: Vec<u8> = Vec::new();

    buffer.write_u32::<T>(index as u32)?;
    buffer.write_u32::<T>(self.buffer.len() as u32)?;

    let bytes_written: usize = buffer.write(self.buffer.as_slice())?;

    assert_eq!(bytes_written, self.buffer.len());

    Ok(buffer)
  }

  /// Get count of bytes written into internal buffer.
  pub fn bytes_written(&self) -> usize {
    self.buffer.len()
  }

  /// Write null terminated string.
  pub fn write_null_terminated_string(&mut self, value: &String) -> io::Result<usize> {
    Ok(self.write(value.as_bytes())? + self.write(&[0u8])?)
  }

  /// Write three float values.
  pub fn write_f32_3d_vector<T: ByteOrder>(&mut self, value: &Vector3d<f32>) -> io::Result<()> {
    self.write_f32::<T>(value.0)?;
    self.write_f32::<T>(value.1)?;
    self.write_f32::<T>(value.2)?;

    Ok(())
  }

  /// Write 4 bytes value as 4 separate byte entries.
  pub fn write_u32_bytes(&mut self, value: &U32Bytes) -> io::Result<()> {
    self.write_u8(value.0)?;
    self.write_u8(value.1)?;
    self.write_u8(value.2)?;
    self.write_u8(value.3)?;

    Ok(())
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
