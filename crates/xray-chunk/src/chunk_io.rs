use crate::reader::chunk_reader::ChunkReader;
use crate::source::chunk_data_source::ChunkDataSource;
use bytes::Bytes;
use fileslice::FileSlice;
use parquet::file::reader::{ChunkReader as ParquetChunkReader, Length};
use std::io::Read;

impl<T: ChunkDataSource> Length for ChunkReader<T> {
  fn len(&self) -> u64 {
    self.data.end_pos() - self.data.start_pos()
  }
}

impl<T: ChunkDataSource> Read for ChunkReader<T> {
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    self.data.read(buf)
  }
}

impl ParquetChunkReader for ChunkReader<FileSlice> {
  type T = FileSlice;

  fn get_read(&self, start: u64) -> parquet::errors::Result<FileSlice> {
    Ok(self.data.slice(start..self.data.end_pos()))
  }

  fn get_bytes(&self, start: u64, length: usize) -> parquet::errors::Result<Bytes> {
    let mut buffer: Vec<u8> = vec![0; length];

    self
      .data
      .slice(start..(start + length as u64))
      .read_exact(&mut buffer)?;

    Ok(buffer.into())
  }
}
