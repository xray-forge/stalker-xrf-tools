use byteorder::ReadBytesExt;
use std::io::{Read, Result as IoResult, SeekFrom};
use std::ops::RangeBounds;

pub trait ChunkDataSource: ReadBytesExt + Read {
  fn start_pos(&self) -> u64;

  fn cursor_pos(&self) -> u64;

  fn end_pos(&self) -> u64;

  fn set_seek(&mut self, pos: SeekFrom) -> IoResult<u64>;

  fn get_seek(&mut self) -> IoResult<u64>;

  fn slice<T: RangeBounds<u64>>(&self, range: T) -> Self;

  fn read_bytes(&mut self, count: usize) -> IoResult<Vec<u8>> {
    let mut buffer: Vec<u8> = vec![0; count];

    self.read_exact(&mut buffer)?;

    Ok(buffer)
  }
}
