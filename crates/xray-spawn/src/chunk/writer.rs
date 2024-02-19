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
}

impl Write for ChunkWriter {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    self.buffer.write(buf)
  }

  fn flush(&mut self) -> io::Result<()> {
    self.buffer.flush()
  }
}
