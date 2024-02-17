use crate::chunk::chunk::Chunk;
use crate::chunk::constants::CFS_COMPRESS_MARK;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;
use fileslice::FileSlice;
use std::io::{Seek, SeekFrom};

#[derive(Debug)]
pub struct ChunkIterator<'lifetime> {
  pub index: u32,
  pub file: &'lifetime mut FileSlice,
}

impl<'lifetime> ChunkIterator<'lifetime> {
  pub fn new(file: &mut FileSlice) -> ChunkIterator {
    file.seek(SeekFrom::Start(0)).unwrap();

    return ChunkIterator { index: 0, file };
  }
}

/// Iterates over chunk and read child chunks.
impl<'lifetime> Iterator for ChunkIterator<'lifetime> {
  type Item = Chunk;

  fn next(&mut self) -> Option<Chunk> {
    let chunk_type = self.file.read_u32::<SpawnByteOrder>();
    let chunk_size = self.file.read_u32::<SpawnByteOrder>();

    if chunk_type.is_err() || chunk_size.is_err() {
      return None;
    }

    let chunk_id: u32 = chunk_type.unwrap();
    let chunk_size: u32 = chunk_size.unwrap();

    return if self.index == chunk_id & (!CFS_COMPRESS_MARK) {
      let position: u64 = self.file.seek(SeekFrom::Current(0)).unwrap();
      let mut file: FileSlice = self.file.slice(position..(position + chunk_size as u64));

      file.seek(SeekFrom::Start(0)).unwrap();

      let chunk = Chunk {
        index: chunk_id,
        is_compressed: chunk_id & CFS_COMPRESS_MARK == 1,
        size: chunk_size,
        position: self.file.seek(SeekFrom::Current(0)).unwrap(),
        file,
      };

      if chunk.is_compressed {
        panic!("Parsing not implemented compressed chunk.");
      }

      // Rewind for next iteration.
      self
        .file
        .seek(SeekFrom::Current(chunk_size as i64))
        .unwrap();

      // Iterate to next item.
      self.index += 1;

      Some(chunk)
    } else {
      None
    };
  }
}
