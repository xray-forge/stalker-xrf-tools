use crate::chunk::chunk::Chunk;
use bytes::Bytes;
use fileslice::FileSlice;
use parquet::file::reader::{ChunkReader, Length};
use std::io::Read;

impl Length for Chunk {
  fn len(&self) -> u64 {
    self.file.end_pos() - self.file.start_pos()
  }
}

impl ChunkReader for Chunk {
  type T = FileSlice;

  fn get_read(&self, start: u64) -> parquet::errors::Result<FileSlice> {
    Ok(self.file.slice(start..self.file.end_pos()))
  }

  fn get_bytes(&self, start: u64, length: usize) -> parquet::errors::Result<Bytes> {
    let mut buf = vec![0; length];
    self
      .file
      .slice(start..(start + length as u64))
      .read_exact(&mut buf)?;
    Ok(buf.into())
  }
}

impl Read for Chunk {
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    self.file.read(buf)
  }
}
