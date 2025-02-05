use crate::constants::CFS_COMPRESS_MARK;
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
  pub fn from_start(reader: &mut ChunkReader<T>) -> ChunkIterator<T> {
    reader
      .source
      .set_seek(SeekFrom::Start(0))
      .expect("Iterator stream position seeking expected");

    ChunkIterator { reader }
  }

  pub fn from_current(reader: &mut ChunkReader<T>) -> ChunkIterator<T> {
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

    let position: u64 = self
      .reader
      .source
      .get_seek()
      .expect("Iterator seek position");

    if id & CFS_COMPRESS_MARK != 0 {
      todo!("Parsing not implemented compressed chunk");
    }

    self
      .reader
      .source
      .set_seek(SeekFrom::Current(size as i64))
      .unwrap();

    Some(Self::Item {
      id,
      is_compressed: false,
      size: size as u64,
      position,
      source: Box::new(self.reader.source.slice(position..(position + size as u64))),
    })
  }
}
