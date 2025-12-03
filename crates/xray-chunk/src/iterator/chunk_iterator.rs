use crate::reader::chunk_reader::ChunkReader;
use crate::{ChunkDataSource, XRayByteOrder};
use byteorder::ReadBytesExt;
use fileslice::FileSlice;
use std::io::SeekFrom;

/// Iterate over samples in provided file slice.
/// Mutates parent object to keep track of what was read during execution.
pub struct ChunkIterator<'a, T: ChunkDataSource = FileSlice> {
  pub reader: &'a mut ChunkReader<T>,
}

impl<T: ChunkDataSource> ChunkIterator<'_, T> {
  pub fn from_start(reader: &mut ChunkReader<T>) -> ChunkIterator<'_, T> {
    reader
      .reset_pos()
      .expect("Iterator reader position reset expected");

    ChunkIterator { reader }
  }

  pub fn from_current(reader: &mut ChunkReader<T>) -> ChunkIterator<'_, T> {
    ChunkIterator { reader }
  }
}

/// Iterates over chunk and read child samples.
impl<T: ChunkDataSource> Iterator for ChunkIterator<'_, T> {
  type Item = ChunkReader<T>;

  fn next(&mut self) -> Option<ChunkReader<T>> {
    if self.reader.is_ended() {
      return None;
    }

    let id: u32 = self
      .reader
      // todo: Hardcoded byte order, should be part of generics.
      .read_u32::<XRayByteOrder>()
      .expect("Chunk ID read");
    let size: u32 = self
      .reader
      // todo: Hardcoded byte order, should be part of generics.
      .read_u32::<XRayByteOrder>()
      .expect("Chunk size read");

    let position: u64 = self.reader.data.get_seek().expect("Iterator seek position");

    if id & (1 << 31) != 0 {
      todo!("Parsing not implemented compressed chunk");
    }

    self
      .reader
      .data
      .set_seek(SeekFrom::Current(size as i64))
      .unwrap();

    Some(Self::Item {
      id,
      size: size as u64,
      position,
      data: self.reader.data.slice(position..(position + size as u64)),
    })
  }
}
