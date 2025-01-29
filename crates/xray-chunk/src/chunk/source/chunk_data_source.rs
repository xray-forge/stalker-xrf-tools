use byteorder::ReadBytesExt;
use std::io;
use std::io::{Read, SeekFrom};

pub trait ChunkDataSource: ReadBytesExt + Read {
  fn start_pos(&self) -> u64;

  fn cursor_pos(&self) -> u64;

  fn end_pos(&self) -> u64;

  fn set_seek(&mut self, pos: SeekFrom) -> io::Result<u64>;

  fn read_bytes(&mut self, count: usize) -> io::Result<Vec<u8>> {
    let mut buffer: Vec<u8> = vec![0; count];

    self.read_exact(&mut buffer)?;

    Ok(buffer)
  }
}
