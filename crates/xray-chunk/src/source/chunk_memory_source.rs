use crate::source::chunk_data_source::ChunkDataSource;
use bytes::Buf;
use parquet::file::reader::Length;
use std::io::{Cursor, Read, Result as IoResult, Seek, SeekFrom};
use std::ops::RangeBounds;

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

  fn set_seek(&mut self, pos: SeekFrom) -> IoResult<u64> {
    self.cursor.seek(pos)
  }

  fn get_seek(&mut self) -> IoResult<u64> {
    self.cursor.stream_position()
  }

  fn slice<T: RangeBounds<u64>>(&self, range: T) -> Self {
    let start: usize = match range.start_bound() {
      std::ops::Bound::Included(&start) => start as usize,
      std::ops::Bound::Excluded(&start) => start as usize + 1,
      std::ops::Bound::Unbounded => 0,
    };

    let end: usize = match range.end_bound() {
      std::ops::Bound::Included(&end) => end as usize + 1,
      std::ops::Bound::Excluded(&end) => end as usize,
      std::ops::Bound::Unbounded => self.cursor.get_ref().len(),
    };

    Self {
      cursor: Cursor::new(self.cursor.get_ref()[start..end].to_vec()),
    }
  }
}

impl Length for InMemoryChunkDataSource {
  fn len(&self) -> u64 {
    self.cursor.remaining() as u64
  }
}

impl Read for InMemoryChunkDataSource {
  fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
    self.cursor.read(buf)
  }
}
