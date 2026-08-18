use std::io::{Cursor, Read, Result as IoResult, Seek, SeekFrom};
use std::ops::RangeBounds;

use crate::source::chunk_data_source::ChunkDataSource;

#[derive(Clone)]
pub struct InMemoryChunkDataSource {
  cursor: Cursor<Vec<u8>>,
}

impl InMemoryChunkDataSource {
  /// Copies `buf` into a seekable in-memory source.
  pub fn from_buffer(buf: &[u8]) -> Self {
    Self {
      cursor: Cursor::new(buf.to_vec()),
    }
  }

  /// Count of bytes left to read from the current cursor position.
  pub fn len(&self) -> u64 {
    (self.cursor.get_ref().len() as u64).saturating_sub(self.cursor.position())
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
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
    self.cursor.get_ref().len() as u64
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

impl Read for InMemoryChunkDataSource {
  fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
    self.cursor.read(buf)
  }
}
