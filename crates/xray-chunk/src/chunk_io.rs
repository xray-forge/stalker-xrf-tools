use std::io::Read;

use crate::reader::chunk_reader::ChunkReader;
use crate::source::chunk_data_source::ChunkDataSource;

impl<T: ChunkDataSource> Read for ChunkReader<T> {
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    self.data.read(buf)
  }
}
