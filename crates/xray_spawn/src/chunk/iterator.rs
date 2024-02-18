use crate::chunk::chunk::Chunk;
use crate::chunk::constants::CFS_COMPRESS_MARK;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;
use fileslice::FileSlice;
use std::io::{Seek, SeekFrom};

/// Iterate over chunks in provided file slice.
/// Mutates parent object to keep track of what was read during execution.
#[derive(Debug)]
pub struct FileChunkIterator<'lifetime> {
  pub index: u32,
  pub chunk: &'lifetime mut Chunk,
}

impl<'lifetime> FileChunkIterator<'lifetime> {
  // todo: Replace with chunk based approach.
  pub fn new(chunk: &mut Chunk) -> FileChunkIterator {
    chunk.file.seek(SeekFrom::Start(0)).unwrap();

    return FileChunkIterator { index: 0, chunk };
  }
}

/// Iterates over chunk and read child chunks.
impl<'lifetime> Iterator for FileChunkIterator<'lifetime> {
  type Item = Chunk;

  fn next(&mut self) -> Option<Chunk> {
    let chunk_type = self.chunk.read_u32::<SpawnByteOrder>();
    let chunk_size = self.chunk.read_u32::<SpawnByteOrder>();

    if chunk_type.is_err() || chunk_size.is_err() {
      return None;
    }

    let chunk_id: u32 = chunk_type.unwrap();
    let chunk_size: u32 = chunk_size.unwrap();

    return if self.index == chunk_id & (!CFS_COMPRESS_MARK) {
      let position: u64 = self.chunk.file.seek(SeekFrom::Current(0)).unwrap();
      let mut file: FileSlice = self
        .chunk
        .file
        .slice(position..(position + chunk_size as u64));

      file.seek(SeekFrom::Start(0)).unwrap();

      let chunk = Chunk {
        index: chunk_id,
        is_compressed: chunk_id & CFS_COMPRESS_MARK == 1,
        size: chunk_size as u64,
        position: self.chunk.file.seek(SeekFrom::Current(0)).unwrap(),
        file,
      };

      if chunk.is_compressed {
        panic!("Parsing not implemented compressed chunk.");
      }

      // Rewind for next iteration.
      self
        .chunk
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
