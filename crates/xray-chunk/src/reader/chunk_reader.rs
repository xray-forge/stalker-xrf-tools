use crate::iterator::chunk_iterator::ChunkIterator;
use crate::source::chunk_data_source::ChunkDataSource;
use crate::source::chunk_memory_source::InMemoryChunkDataSource;
use fileslice::FileSlice;
use parquet::file::reader::Length;
use std::fmt;
use std::fs::File;
use std::io::SeekFrom;
use xray_error::{XRayError, XRayResult};

#[derive(Clone)]
pub struct ChunkReader<T: ChunkDataSource = FileSlice> {
  pub id: u32,
  pub size: u64,
  pub position: u64,
  pub data: Box<T>,
}

impl ChunkReader<FileSlice> {
  /// Create chunk based on whole file.
  pub fn from_file(file: File) -> XRayResult<Self> {
    Self::from_slice(FileSlice::new(file))
  }

  /// Create chunk based on file slice boundaries.
  pub fn from_slice(slice: FileSlice) -> XRayResult<Self> {
    if slice.is_empty() {
      return Err(XRayError::new_invalid_error(
        "Failed to create chunk from empty source",
      ));
    }

    Ok(Self {
      id: 0,
      size: slice.len() as u64,
      position: slice.start_pos(),
      data: Box::new(slice),
    })
  }
}

impl ChunkReader<InMemoryChunkDataSource> {
  /// Create chunk based on whole file.
  pub fn from_bytes(buf: &[u8]) -> XRayResult<Self> {
    Self::from_source(InMemoryChunkDataSource::from_buffer(buf))
  }

  /// Create chunk based on source.
  pub fn from_source(source: InMemoryChunkDataSource) -> XRayResult<Self> {
    Ok(Self {
      id: 0,
      size: source.len(),
      position: 0,
      data: Box::new(source),
    })
  }
}

impl<T: ChunkDataSource> ChunkReader<T> {
  /// Get current position of the chunk seek.
  pub fn cursor_pos(&self) -> u64 {
    self.data.cursor_pos()
  }

  /// Get end position of the chunk seek.
  pub fn end_pos(&self) -> u64 {
    self.data.end_pos()
  }

  /// Whether chunk is ended and contains no more data to read.
  pub fn is_ended(&self) -> bool {
    self.data.cursor_pos() == self.data.end_pos()
  }

  /// Whether chunk contains data to read.
  pub fn has_data(&self) -> bool {
    self.data.cursor_pos() < self.data.end_pos()
  }

  /// Get summary of bytes read from chunk based on current seek position.
  pub fn read_bytes_len(&self) -> u64 {
    self.data.cursor_pos() - self.data.start_pos()
  }

  /// Get summary of bytes remaining based on current seek position.
  pub fn read_bytes_remain(&self) -> u64 {
    self.data.end_pos() - self.data.cursor_pos()
  }

  /// Reset seek position in chunk file.
  pub fn reset_pos(&mut self) -> XRayResult<u64> {
    Ok(self.data.set_seek(SeekFrom::Start(0))?)
  }

  /// Navigates to chunk with index and constructs chunk representation.
  pub fn read_child_by_index(&mut self, id: u32) -> XRayResult<Self> {
    for (iteration, chunk) in ChunkIterator::from_start(self).enumerate() {
      if id as usize == iteration {
        return Ok(chunk);
      }
    }

    Err(XRayError::new_invalid_error(format!(
      "Attempt to read not existing chunk with id {} in chunk {}",
      id, self.id
    )))
  }

  /// Get list of all child samples in current chunk, do not mutate current chunk.
  pub fn get_children_cloned(&self) -> Vec<Self> {
    ChunkIterator::from_start(&mut self.clone()).collect()
  }

  /// Read list of all child samples in current chunk and advance further.
  pub fn read_children(&mut self) -> Vec<Self> {
    ChunkIterator::<T>::from_start(self).collect()
  }

  /// Assert data in chink is read and nothing remains to read.
  #[inline]
  pub fn assert_read(&self, message: &str) -> XRayResult {
    if self.is_ended() {
      Ok(())
    } else {
      Err(XRayError::new_chunk_not_ended_error(
        message,
        self.read_bytes_remain(),
      ))
    }
  }
}

impl fmt::Debug for ChunkReader {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "Chunk {{ index: {}, size: {}, position: {} }}",
      self.id, self.size, self.position
    )
  }
}

#[cfg(test)]
mod tests {
  use crate::reader::chunk_reader::ChunkReader;
  use fileslice::FileSlice;
  use xray_error::XRayResult;
  use xray_test_utils::utils::{get_relative_test_sample_sub_dir, open_test_resource_as_slice};

  #[test]
  fn test_read_empty_file() -> XRayResult {
    let file: FileSlice = open_test_resource_as_slice("empty")?;

    assert_eq!(file.start_pos(), 0);
    assert_eq!(file.end_pos(), 0);

    let result: XRayResult<ChunkReader> = ChunkReader::from_slice(file);

    assert!(
      result.is_err(),
      "File should be empty and fail to read data"
    );
    assert_eq!(
      result.unwrap_err().to_string(),
      "Invalid error: Failed to create chunk from empty source",
      "Expect input error"
    );

    Ok(())
  }

  #[test]
  fn test_read_empty_chunk() -> XRayResult {
    let filename: String = get_relative_test_sample_sub_dir("empty_nested_single.chunk");
    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.start_pos(), 0);
    assert_eq!(file.end_pos(), 8);

    let reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert!(reader.is_ended(), "Expect empty chunk");

    Ok(())
  }

  #[test]
  fn test_read_empty_children() -> XRayResult {
    let filename: String = get_relative_test_sample_sub_dir("empty_nested_single.chunk");
    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    let chunks: Vec<ChunkReader> = ChunkReader::from_slice(file)?.get_children_cloned();

    assert_eq!(chunks.len(), 1, "Expect single chunk");
    assert_eq!(chunks.first().unwrap().size, 0);

    let filename: String = get_relative_test_sample_sub_dir("empty_nested_five.chunk");
    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    let chunks: Vec<ChunkReader> = ChunkReader::from_slice(file)?.get_children_cloned();

    assert_eq!(chunks.len(), 5, "Expect five chunks");
    assert_eq!(chunks[0].size, 0);
    assert_eq!(chunks[1].size, 0);
    assert_eq!(chunks[2].size, 0);
    assert_eq!(chunks[3].size, 0);
    assert_eq!(chunks[4].size, 0);

    Ok(())
  }

  #[test]
  fn test_read_empty_unordered_children() -> XRayResult {
    let filename: String = get_relative_test_sample_sub_dir("empty_nested_five_unordered.chunk");
    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    let chunks: Vec<ChunkReader> = ChunkReader::from_slice(file)?.get_children_cloned();

    assert_eq!(chunks.len(), 5, "Expect five chunks");
    assert_eq!(chunks[0].size, 0);
    assert_eq!(chunks[0].id, 4);
    assert_eq!(chunks[1].size, 0);
    assert_eq!(chunks[1].id, 3);
    assert_eq!(chunks[2].size, 0);
    assert_eq!(chunks[2].id, 2);
    assert_eq!(chunks[3].size, 0);
    assert_eq!(chunks[3].id, 1);
    assert_eq!(chunks[4].size, 0);
    assert_eq!(chunks[4].id, 0);

    Ok(())
  }

  #[test]
  fn test_read_dummy_children() -> XRayResult {
    let filename: String = get_relative_test_sample_sub_dir("dummy_nested_single.chunk");
    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    let chunks: Vec<ChunkReader> = ChunkReader::from_slice(file)?.get_children_cloned();

    assert_eq!(chunks.len(), 1, "Expect single chunk");
    assert_eq!(chunks.first().unwrap().size, 8);

    let filename: String = get_relative_test_sample_sub_dir("dummy_nested_five.chunk");
    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    let chunks: Vec<ChunkReader> = ChunkReader::from_slice(file)?.get_children_cloned();

    assert_eq!(chunks.len(), 5, "Expect five chunks");
    assert_eq!(chunks[0].size, 8);
    assert_eq!(chunks[1].size, 24);
    assert_eq!(chunks[2].size, 16);
    assert_eq!(chunks[3].size, 0);
    assert_eq!(chunks[4].size, 40);

    Ok(())
  }
}
