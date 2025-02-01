use crate::chunk::source::chunk_data_source::ChunkDataSource;
use crate::ChunkReader;
use byteorder::{ByteOrder, ReadBytesExt};
use xray_error::XRayResult;

impl<D: ChunkDataSource> ChunkReader<D> {
  /// Read serialized vector from chunk, where u32 count N is followed by N u16 entries.
  pub fn read_u16_vector<T: ByteOrder>(&mut self) -> XRayResult<Vec<u16>> {
    let count: u32 = self.read_u32::<T>()?;
    let mut vector: Vec<u16> = Vec::with_capacity(count as usize);

    for _ in 0..count {
      vector.push(self.read_u16::<T>()?)
    }

    Ok(vector)
  }

  /// Read raw bytes.
  pub fn read_bytes(&mut self, count: usize) -> XRayResult<Vec<u8>> {
    Ok(self.source.read_bytes(count)?)
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::chunk_reader::ChunkReader;
  use crate::chunk::source::chunk_memory_source::InMemoryChunkDataSource;
  use crate::XRayByteOrder;
  use xray_error::XRayResult;

  #[test]
  fn test_read_u16_vector() -> XRayResult {
    let mut chunk: ChunkReader<InMemoryChunkDataSource> =
      ChunkReader::from_bytes(&[4, 0, 0, 0, 0, 0, 1, 0, 2, 0, 3, 0])?;

    assert_eq!(chunk.read_bytes_remain(), 12, "Expect 12 bytes remaining");
    assert_eq!(chunk.cursor_pos(), 0, "Expect 0 bytes read");

    assert_eq!(
      chunk.read_u16_vector::<XRayByteOrder>()?,
      vec!(0u16, 1u16, 2u16, 3u16),
      "Expect correctly read vector"
    );
    assert_eq!(chunk.cursor_pos(), 12, "Expect 12 bytes read");

    Ok(())
  }

  #[test]
  fn test_read_bytes() -> XRayResult {
    let mut chunk: ChunkReader<InMemoryChunkDataSource> =
      ChunkReader::from_bytes(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9])?;

    assert_eq!(chunk.read_bytes_remain(), 10, "Expect 10 bytes remaining");
    assert_eq!(chunk.cursor_pos(), 0, "Expect 0 bytes read");

    assert_eq!(
      chunk.read_bytes(10)?,
      vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9),
      "Expect correctly read raw bytes"
    );
    assert_eq!(chunk.cursor_pos(), 10, "Expect 10 bytes read");

    Ok(())
  }
}
