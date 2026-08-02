use crate::reader::chunk_reader::ChunkReader;
use crate::{ChunkDataSource, XRayByteOrder};
use byteorder::ReadBytesExt;
use fileslice::FileSlice;
use std::io::SeekFrom;
use xray_error::{XRayError, XRayResult};

/// Iterate over samples in provided file slice.
/// Mutates parent object to keep track of what was read during execution.
pub struct ChunkIterator<'a, T: ChunkDataSource = FileSlice> {
  pub reader: &'a mut ChunkReader<T>,
  pub failed: bool,
}

impl<T: ChunkDataSource> ChunkIterator<'_, T> {
  pub fn from_start(reader: &mut ChunkReader<T>) -> XRayResult<ChunkIterator<'_, T>> {
    reader.reset_pos()?;

    Ok(ChunkIterator {
      reader,
      failed: false,
    })
  }

  pub fn from_current(reader: &mut ChunkReader<T>) -> ChunkIterator<'_, T> {
    ChunkIterator {
      reader,
      failed: false,
    }
  }

  fn fail(&mut self, error: XRayError) -> Option<XRayResult<ChunkReader<T>>> {
    self.failed = true;

    Some(Err(error))
  }
}

/// Iterates over chunk and read child samples.
impl<T: ChunkDataSource> Iterator for ChunkIterator<'_, T> {
  type Item = XRayResult<ChunkReader<T>>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.failed || self.reader.is_ended() {
      return None;
    }

    let header_size: u64 = 8;
    let remaining: u64 = self.reader.read_bytes_remain();

    if remaining < header_size {
      return self.fail(XRayError::new_invalid_error(format!(
        "Incomplete chunk header at position {}, expected {} bytes but only {} remain",
        self.reader.cursor_pos(),
        header_size,
        remaining
      )));
    }

    // todo: Hardcoded byte order, should be part of generics.
    let id: u32 = match self.reader.read_u32::<XRayByteOrder>() {
      Ok(id) => id,
      Err(error) => return self.fail(error.into()),
    };
    // todo: Hardcoded byte order, should be part of generics.
    let size: u32 = match self.reader.read_u32::<XRayByteOrder>() {
      Ok(size) => size,
      Err(error) => return self.fail(error.into()),
    };

    let position: u64 = match self.reader.data.get_seek() {
      Ok(position) => position,
      Err(error) => return self.fail(error.into()),
    };

    if id & (1 << 31) != 0 {
      return self.fail(XRayError::new_not_implemented_error(format!(
        "Compressed chunk {id:#010x} at position {position}"
      )));
    }

    let end_position: u64 = match position.checked_add(size as u64) {
      Some(end_position) => end_position,
      None => {
        return self.fail(XRayError::new_invalid_error(format!(
          "Chunk {id:#010x} size {size} overflows its position {position}"
        )));
      }
    };

    if end_position > self.reader.end_pos() {
      return self.fail(XRayError::new_invalid_error(format!(
        "Chunk {id:#010x} at position {position} declares {size} bytes, beyond source end {}",
        self.reader.end_pos()
      )));
    }

    if let Err(error) = self.reader.data.set_seek(SeekFrom::Current(size as i64)) {
      return self.fail(error.into());
    }

    Some(Ok(ChunkReader {
      id,
      size: size as u64,
      position,
      data: self.reader.data.slice(position..end_position),
    }))
  }
}

#[cfg(test)]
mod tests {
  use crate::{ChunkReader, InMemoryChunkDataSource};
  use xray_error::XRayResult;

  #[test]
  fn rejects_incomplete_chunk_header() -> XRayResult {
    let mut reader: ChunkReader<InMemoryChunkDataSource> = ChunkReader::from_bytes(&[0, 0, 0])?;
    let error: String = match reader.read_children() {
      Ok(_) => panic!("Expected incomplete chunk header to fail"),
      Err(error) => error.to_string(),
    };

    assert!(
      error.contains("Incomplete chunk header"),
      "Unexpected error: {error}"
    );

    Ok(())
  }

  #[test]
  fn rejects_chunk_data_beyond_source_end() -> XRayResult {
    let mut reader: ChunkReader<InMemoryChunkDataSource> =
      ChunkReader::from_bytes(&[1, 0, 0, 0, 10, 0, 0, 0, 0, 0])?;
    let error: String = match reader.read_children() {
      Ok(_) => panic!("Expected oversized chunk to fail"),
      Err(error) => error.to_string(),
    };

    assert!(
      error.contains("beyond source end"),
      "Unexpected error: {error}"
    );

    Ok(())
  }
}
