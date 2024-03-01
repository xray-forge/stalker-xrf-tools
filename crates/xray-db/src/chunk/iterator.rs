use crate::chunk::constants::CFS_COMPRESS_MARK;
use crate::chunk::reader::ChunkReader;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;
use fileslice::FileSlice;
use std::io::{Seek, SeekFrom};

/// Iterate over samples in provided file slice.
/// Mutates parent object to keep track of what was read during execution.
#[derive(Debug)]
pub struct ChunkIterator<'lifetime> {
  pub index: u32,
  pub reader: &'lifetime mut ChunkReader,
}

impl<'lifetime> ChunkIterator<'lifetime> {
  pub fn new(reader: &mut ChunkReader) -> ChunkIterator {
    reader.file.seek(SeekFrom::Start(0)).unwrap();

    ChunkIterator { index: 0, reader }
  }
}

/// Iterates over chunk and read child samples.
impl<'lifetime> Iterator for ChunkIterator<'lifetime> {
  type Item = ChunkReader;

  fn next(&mut self) -> Option<ChunkReader> {
    let chunk_type = self.reader.read_u32::<SpawnByteOrder>();
    let chunk_size = self.reader.read_u32::<SpawnByteOrder>();

    if chunk_type.is_err() || chunk_size.is_err() {
      return None;
    }

    let chunk_id: u32 = chunk_type.unwrap();
    let chunk_size: u32 = chunk_size.unwrap();

    if self.index == chunk_id {
      let position: u64 = self.reader.file.stream_position().unwrap();
      let mut file: FileSlice = self
        .reader
        .file
        .slice(position..(position + chunk_size as u64));

      file.seek(SeekFrom::Start(0)).unwrap();

      let reader: ChunkReader = ChunkReader {
        index: chunk_id,
        is_compressed: chunk_id & CFS_COMPRESS_MARK == 1,
        size: chunk_size as u64,
        position: self.reader.file.stream_position().unwrap(),
        file: Box::new(file),
      };

      if reader.is_compressed {
        panic!("Parsing not implemented compressed chunk");
      }

      // Rewind for next iteration.
      self
        .reader
        .file
        .seek(SeekFrom::Current(chunk_size as i64))
        .unwrap();

      // Iterate to next item.
      self.index += 1;

      Some(reader)
    } else {
      None
    }
  }
}

/// Iterate over data in chunk slice, which is stored like (size)(content)(size)(content).
#[derive(Debug)]
pub struct ChunkSizePackedIterator<'lifetime> {
  pub index: u32,
  pub next_seek: u64,
  pub reader: &'lifetime mut ChunkReader,
}

impl<'lifetime> ChunkSizePackedIterator<'lifetime> {
  pub fn new(reader: &mut ChunkReader) -> ChunkSizePackedIterator {
    ChunkSizePackedIterator {
      index: 0,
      next_seek: reader.file.stream_position().unwrap(),
      reader,
    }
  }
}

impl<'lifetime> Iterator for ChunkSizePackedIterator<'lifetime> {
  type Item = ChunkReader;

  fn next(&mut self) -> Option<ChunkReader> {
    let current: u64 = self.reader.file.stream_position().unwrap();

    if current > self.next_seek {
      panic!("Unexpected iteration over chunk packed data, previous iteration moved seek too far")
    } else if self.reader.is_ended() {
      return None;
    }

    let current: u64 = self.next_seek;

    self.reader.file.seek(SeekFrom::Start(current)).unwrap();

    let chunk_size: u32 = self.reader.read_u32::<SpawnByteOrder>().unwrap();

    self.index += 1;
    self.next_seek = self
      .reader
      .file
      .seek(SeekFrom::Start(current + chunk_size as u64))
      .unwrap();

    Some(ChunkReader {
      index: self.index,
      is_compressed: false,
      size: chunk_size as u64,
      position: self.reader.file.stream_position().unwrap(),
      file: Box::new(
        self
          .reader
          .file
          .slice(current + 4..(current + chunk_size as u64)),
      ),
    })
  }
}
