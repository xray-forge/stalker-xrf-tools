use crate::{ChunkDataSource, ChunkReader, XRayByteOrder};
use byteorder::ReadBytesExt;
use fileslice::FileSlice;
use std::io::SeekFrom;

/// Iterate over data in chunk slice, which is stored like [(size)(content)(size)(content)].
pub struct ChunkSizePackedIterator<'a, T: ChunkDataSource = FileSlice> {
  pub index: u32,
  pub reader: &'a mut ChunkReader<T>,
}

impl<T: ChunkDataSource> ChunkSizePackedIterator<'_, T> {
  pub fn from_start(reader: &mut ChunkReader<T>) -> ChunkSizePackedIterator<T> {
    reader
      .reset_pos()
      .expect("Iterator reader position reset expected");

    ChunkSizePackedIterator { index: 0, reader }
  }

  pub fn from_current(reader: &mut ChunkReader<T>) -> ChunkSizePackedIterator<T> {
    ChunkSizePackedIterator { index: 0, reader }
  }
}

impl<T: ChunkDataSource> Iterator for ChunkSizePackedIterator<'_, T> {
  type Item = ChunkReader<T>;

  fn next(&mut self) -> Option<ChunkReader<T>> {
    if self.reader.is_ended() {
      return None;
    }

    let position: u64 = self.reader.data.get_seek().expect("Iterator seek position");

    let size: u64 = self
      .reader
      // todo: Hardcoded byte order, should be part of generics.
      .read_u32::<XRayByteOrder>()
      .expect("Packed iterator size expected") as u64;

    let id: u32 = self.index;

    self.index += 1;
    self
      .reader
      .data
      .set_seek(SeekFrom::Current(size as i64 - 4))
      .expect("Iterator seek position");

    Some(Self::Item {
      id,
      size,
      position,
      data: Box::new(self.reader.data.slice(position + 4..position + size)),
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::{ChunkDataSource, ChunkReader, ChunkSizePackedIterator, InMemoryChunkDataSource};
  use std::io::SeekFrom;
  use xray_error::XRayResult;

  #[test]
  fn test_iterate_empty() -> XRayResult {
    let mut chunk_reader: ChunkReader<InMemoryChunkDataSource> =
      ChunkReader::from_source(InMemoryChunkDataSource::from_buffer(&[]))?;

    if ChunkSizePackedIterator::from_start(&mut chunk_reader)
      .next()
      .is_some()
    {
      panic!("No iterations expected in empty data");
    }

    if ChunkSizePackedIterator::from_current(&mut chunk_reader)
      .next()
      .is_some()
    {
      panic!("No iterations expected in empty data");
    }

    Ok(())
  }

  #[test]
  fn test_iterate_single() -> XRayResult {
    let mut chunk_reader: ChunkReader<InMemoryChunkDataSource> =
      ChunkReader::from_source(InMemoryChunkDataSource::from_buffer(&[5, 0, 0, 0, 255]))?;

    let mut vec: Vec<ChunkReader<InMemoryChunkDataSource>> = Vec::new();

    for it in ChunkSizePackedIterator::from_start(&mut chunk_reader) {
      vec.push(it);
    }

    assert_eq!(vec.len(), 1, "Expected count to be 1");
    assert_eq!(vec[0].id, 0, "Expected id to be 0");
    assert_eq!(vec[0].size, 5, "Expected size to be 5");
    assert_eq!(vec[0].position, 0, "Expected position to be 0");

    Ok(())
  }

  #[test]
  fn test_iterate_few_start() -> XRayResult {
    let mut chunk_reader: ChunkReader<InMemoryChunkDataSource> =
      ChunkReader::from_source(InMemoryChunkDataSource::from_buffer(&[
        8, 0, 0, 0, 255, 255, 255, 255, 12, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255,
      ]))?;

    let mut vec: Vec<ChunkReader<InMemoryChunkDataSource>> = Vec::new();

    for it in ChunkSizePackedIterator::from_start(&mut chunk_reader) {
      vec.push(it);
    }

    assert_eq!(vec.len(), 2, "Expected count to be 2");
    assert_eq!(vec[0].id, 0, "Expected [0] id to be 0");
    assert_eq!(vec[0].size, 8, "Expected [0] size to be 9");
    assert_eq!(vec[0].position, 0, "Expected [0] position to be 0");
    assert_eq!(vec[1].id, 1, "Expected [1] id to be 1");
    assert_eq!(vec[1].size, 12, "Expected [1] size to be 12");
    assert_eq!(vec[1].position, 8, "Expected [1] position to be 8");

    Ok(())
  }

  #[test]
  fn test_iterate_few_mid() -> XRayResult {
    let mut chunk_reader: ChunkReader<InMemoryChunkDataSource> =
      ChunkReader::from_source(InMemoryChunkDataSource::from_buffer(&[
        8, 0, 0, 0, 255, 255, 255, 255, 12, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255,
      ]))?;

    chunk_reader.data.set_seek(SeekFrom::Start(8))?;

    let mut vec: Vec<ChunkReader<InMemoryChunkDataSource>> = Vec::new();

    for it in ChunkSizePackedIterator::from_current(&mut chunk_reader) {
      vec.push(it);
    }

    assert_eq!(vec.len(), 1, "Expected count to be 2");
    assert_eq!(vec[0].id, 0, "Expected [1] id to be 0");
    assert_eq!(vec[0].size, 12, "Expected [1] size to be 5");
    assert_eq!(vec[0].position, 8, "Expected [1] position to be 0");

    Ok(())
  }
}
