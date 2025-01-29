use crate::chunk::reader::chunk_reader::ChunkReader;
use crate::ChunkDataSource;
use bytes::Bytes;
use fileslice::FileSlice;
use parquet::file::reader::{ChunkReader as ParquetChunkReader, Length};
use std::io::Read;

impl<T: ChunkDataSource> Length for ChunkReader<T> {
  fn len(&self) -> u64 {
    self.source.end_pos() - self.source.start_pos()
  }
}

impl<T: ChunkDataSource> Read for ChunkReader<T> {
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    self.source.read(buf)
  }
}

impl ParquetChunkReader for ChunkReader<FileSlice> {
  type T = FileSlice;

  fn get_read(&self, start: u64) -> parquet::errors::Result<FileSlice> {
    Ok(self.source.slice(start..self.source.end_pos()))
  }

  fn get_bytes(&self, start: u64, length: usize) -> parquet::errors::Result<Bytes> {
    let mut buffer: Vec<u8> = vec![0; length];

    self
      .source
      .slice(start..(start + length as u64))
      .read_exact(&mut buffer)?;

    Ok(buffer.into())
  }
}
