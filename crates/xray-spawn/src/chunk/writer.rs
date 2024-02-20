use crate::types::Vector3d;
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

  pub fn flush_chunk<T: ByteOrder>(&mut self, file: &mut File, index: u32) -> io::Result<usize> {
    self.buffer.flush().unwrap();

    file.write_u32::<T>(index).unwrap();
    file.write_u32::<T>(self.buffer.len() as u32)?;

    file.write(self.buffer.as_slice())
  }

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
}

impl Write for ChunkWriter {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    self.buffer.write(buf)
  }

  fn flush(&mut self) -> io::Result<()> {
    self.buffer.flush()
  }
}
