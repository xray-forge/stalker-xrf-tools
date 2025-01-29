use crate::chunk::source::chunk_data_source::ChunkDataSource;
use bytes::Buf;
use parquet::file::reader::Length;
use std::io;
use std::io::{Cursor, Read, Seek, SeekFrom};

pub struct InMemoryChunkDataSource {
  pub cursor: Cursor<Vec<u8>>,
}

impl InMemoryChunkDataSource {
  pub fn from_buffer(buf: &[u8]) -> Self {
    Self {
      cursor: Cursor::new(buf.to_vec()),
    }
  }

  pub fn is_empty(&self) -> bool {
    !self.cursor.has_remaining()
  }
}

impl ChunkDataSource for InMemoryChunkDataSource {
  fn start_pos(&self) -> u64 {
    0
  }

  fn cursor_pos(&self) -> u64 {
    self.cursor.position()
  }

  fn end_pos(&self) -> u64 {
    self.cursor.remaining() as u64 + self.cursor.position()
  }

  fn set_seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
    self.cursor.seek(pos)
  }
}

impl Length for InMemoryChunkDataSource {
  fn len(&self) -> u64 {
    self.cursor.remaining() as u64
  }
}

impl Read for InMemoryChunkDataSource {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    self.cursor.read(buf)
  }
}
