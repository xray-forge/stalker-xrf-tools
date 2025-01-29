use crate::chunk::source::chunk_data_source::ChunkDataSource;
use fileslice::FileSlice;
use std::io::{Seek, SeekFrom};

impl ChunkDataSource for FileSlice {
  fn start_pos(&self) -> u64 {
    self.start_pos()
  }

  fn cursor_pos(&self) -> u64 {
    self.cursor_pos()
  }

  fn end_pos(&self) -> u64 {
    self.end_pos()
  }

  fn set_seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
    self.seek(pos)
  }
}
