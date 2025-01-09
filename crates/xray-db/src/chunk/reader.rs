use crate::chunk::interface::ChunkDataSource;
use crate::chunk::iterator::ChunkIterator;
use crate::data::generic::shape::Shape;
use crate::data::generic::vector_3d::Vector3d;
use crate::error::database_error::DatabaseError;
use crate::error::database_invalid_chunk_error::DatabaseInvalidChunkError;
use crate::types::{DatabaseResult, U32Bytes};
use byteorder::{ByteOrder, ReadBytesExt};
use encoding_rs::WINDOWS_1251;
use fileslice::FileSlice;
use std::borrow::Cow;
use std::fmt;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Clone, PartialEq)]
pub struct ChunkReader<T: ChunkDataSource = FileSlice> {
  pub id: u32,
  pub size: u64,
  pub position: u64,
  pub is_compressed: bool,
  pub file: Box<T>,
}

impl ChunkReader {
  /// Create chunk based on whole file.
  pub fn from_file(file: File) -> DatabaseResult<ChunkReader> {
    Self::from_slice(FileSlice::new(file))
  }

  /// Create chunk based on file slice boundaries.
  pub fn from_slice(file: FileSlice) -> DatabaseResult<ChunkReader> {
    if file.is_empty() {
      return Err(DatabaseInvalidChunkError::new_database_error(
        "Trying to create chunk from empty file",
      ));
    }

    Ok(ChunkReader {
      id: 0,
      size: file.len() as u64,
      position: file.start_pos(),
      is_compressed: false,
      file: Box::new(file),
    })
  }
}

impl ChunkReader {
  /// Get start position of the chunk seek.
  pub fn start_pos(&self) -> u64 {
    self.file.start_pos()
  }

  /// Get end position of the chunk seek.
  pub fn end_pos(&self) -> u64 {
    self.file.end_pos()
  }

  /// Get current position of the chunk seek.
  pub fn cursor_pos(&self) -> u64 {
    self.file.cursor_pos()
  }

  /// Whether chunk is ended and contains no more data to read.
  pub fn is_ended(&self) -> bool {
    self.file.cursor_pos() == self.file.end_pos()
  }

  /// Whether chunk contains data to read.
  pub fn has_data(&self) -> bool {
    self.file.cursor_pos() < self.file.end_pos()
  }

  /// Get summary of bytes read from chunk based on current seek position.
  pub fn read_bytes_len(&self) -> u64 {
    self.file.cursor_pos() - self.file.start_pos()
  }

  /// Get summary of bytes remaining based on current seek position.
  pub fn read_bytes_remain(&self) -> u64 {
    self.file.end_pos() - self.file.cursor_pos()
  }
}

impl ChunkReader {
  /// Navigates to chunk with index and constructs chunk representation.
  pub fn read_child_by_index(&mut self, index: u32) -> DatabaseResult<ChunkReader> {
    for (iteration, chunk) in ChunkIterator::new(self).enumerate() {
      if index as usize == iteration {
        return Ok(chunk);
      }
    }

    Err(DatabaseInvalidChunkError::new_database_error(String::from(
      "Attempt to read chunk with index out of bonds",
    )))
  }

  /// Get list of all child samples in current chunk, do not mutate current chunk.
  pub fn get_children_cloned(&self) -> Vec<ChunkReader> {
    ChunkIterator::new(&mut self.clone()).collect()
  }

  /// Read list of all child samples in current chunk and advance further.
  pub fn read_children(&mut self) -> Vec<ChunkReader> {
    ChunkIterator::new(self).collect()
  }

  /// Reset seek position in chunk file.
  #[allow(dead_code)]
  pub fn reset_pos(&mut self) -> DatabaseResult<u64> {
    self
      .file
      .seek(SeekFrom::Start(0))
      .map_err(DatabaseError::from)
  }
}

impl ChunkReader {
  /// Read three float values.
  pub fn read_f32_3d_vector<T: ByteOrder>(&mut self) -> DatabaseResult<Vector3d<f32>> {
    Vector3d::read::<T>(self)
  }

  /// Read shape data.
  pub fn read_shapes<T: ByteOrder>(&mut self) -> DatabaseResult<Vec<Shape>> {
    Shape::read_list::<T>(self)
  }

  pub fn read_u32_bytes(&mut self) -> DatabaseResult<U32Bytes> {
    Ok((
      self.read_u8()?,
      self.read_u8()?,
      self.read_u8()?,
      self.read_u8()?,
    ))
  }

  /// Read serialized vector from chunk, where u32 count N is followed by N u16 entries.
  pub fn read_u16_vector<T: ByteOrder>(&mut self) -> DatabaseResult<Vec<u16>> {
    let mut vector: Vec<u16> = Vec::new();
    let count: u32 = self.read_u32::<T>()?;

    for _ in 0..count {
      vector.push(self.read_u16::<T>()?)
    }

    Ok(vector)
  }

  /// Read null terminated windows encoded string from file bytes.
  pub fn read_null_terminated_win_string(&mut self) -> DatabaseResult<String> {
    let offset: u64 = self.file.stream_position()?;
    let mut buffer: Vec<u8> = Vec::new();

    self.file.read_to_end(&mut buffer)?;

    if let Some(position) = buffer.iter().position(|&x| x == 0x00) {
      let slice: &[u8] = &buffer[..position];
      let (transformed, _, had_errors) = WINDOWS_1251.decode(slice);

      if had_errors {
        panic!("Unexpected errors when decoding windows-1251 string data");
      }

      // Try with windows 1251 conversion:
      let value: String = match transformed {
        Cow::Borrowed(value) => value.to_owned(),
        Cow::Owned(value) => value,
      };

      // Put seek right after string - length plus zero terminator.
      self
        .file
        .seek(SeekFrom::Start(offset + position as u64 + 1))
        .expect("Correct object seek movement");

      Ok(value)
    } else {
      panic!("No null terminator found in file");
    }
  }

  pub fn read_bytes(&mut self, count: usize) -> DatabaseResult<Vec<u8>> {
    let mut buffer: Vec<u8> = vec![0; count];

    self.read_exact(&mut buffer)?;

    Ok(buffer)
  }
}

impl fmt::Debug for ChunkReader {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "Chunk {{ index: {}, size: {}, position: {}, is_compressed: {} }}",
      self.id, self.size, self.position, self.is_compressed
    )
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::types::DatabaseResult;
  use fileslice::FileSlice;
  use xray_test_utils::utils::{get_relative_test_sample_sub_dir, open_test_resource_as_slice};

  #[test]
  fn test_read_empty_file() -> DatabaseResult {
    let file: FileSlice = open_test_resource_as_slice("empty")?;

    assert_eq!(file.start_pos(), 0);
    assert_eq!(file.end_pos(), 0);

    let result: DatabaseResult<ChunkReader> = ChunkReader::from_slice(file);

    assert!(
      result.is_err(),
      "File should be empty and fail to read data"
    );
    assert_eq!(
      result.unwrap_err().to_string(),
      String::from("Invalid chunk error: Trying to create chunk from empty file"),
      "Expect input error"
    );

    Ok(())
  }

  #[test]
  fn test_read_empty_chunk() -> DatabaseResult {
    let filename: String = get_relative_test_sample_sub_dir("empty_nested_single.chunk");
    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.start_pos(), 0);
    assert_eq!(file.end_pos(), 8);

    let reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert!(reader.is_ended(), "Expect empty chunk");

    Ok(())
  }

  #[test]
  fn test_read_empty_children() -> DatabaseResult {
    let filename: String = get_relative_test_sample_sub_dir("empty_nested_single.chunk");
    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    let chunks: Vec<ChunkReader> = ChunkReader::from_slice(file)?.get_children_cloned();

    assert_eq!(chunks.len(), 1, "Expect single chunk");
    assert_eq!(chunks.first().unwrap().size, 0);

    let filename: String = get_relative_test_sample_sub_dir("empty_nested_five.chunk");
    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    let chunks: Vec<ChunkReader> = ChunkReader::from_slice(file)?.get_children_cloned();

    assert_eq!(chunks.len(), 5, "Expect five chunks");
    assert_eq!(chunks.get(0).unwrap().size, 0);
    assert_eq!(chunks.get(1).unwrap().size, 0);
    assert_eq!(chunks.get(2).unwrap().size, 0);
    assert_eq!(chunks.get(3).unwrap().size, 0);
    assert_eq!(chunks.get(4).unwrap().size, 0);

    Ok(())
  }

  #[test]
  fn test_read_empty_unordered_children() -> DatabaseResult {
    let filename: String = get_relative_test_sample_sub_dir("empty_nested_five_unordered.chunk");
    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    let chunks: Vec<ChunkReader> = ChunkReader::from_slice(file)?.get_children_cloned();

    assert_eq!(chunks.len(), 5, "Expect five chunks");
    assert_eq!(chunks.get(0).unwrap().size, 0);
    assert_eq!(chunks.get(0).unwrap().id, 4);
    assert_eq!(chunks.get(1).unwrap().size, 0);
    assert_eq!(chunks.get(1).unwrap().id, 3);
    assert_eq!(chunks.get(2).unwrap().size, 0);
    assert_eq!(chunks.get(2).unwrap().id, 2);
    assert_eq!(chunks.get(3).unwrap().size, 0);
    assert_eq!(chunks.get(3).unwrap().id, 1);
    assert_eq!(chunks.get(4).unwrap().size, 0);
    assert_eq!(chunks.get(4).unwrap().id, 0);

    Ok(())
  }

  #[test]
  fn test_read_dummy_children() -> DatabaseResult {
    let filename: String = get_relative_test_sample_sub_dir("dummy_nested_single.chunk");
    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    let chunks: Vec<ChunkReader> = ChunkReader::from_slice(file)?.get_children_cloned();

    assert_eq!(chunks.len(), 1, "Expect single chunk");
    assert_eq!(chunks.first().unwrap().size, 8);

    let filename: String = get_relative_test_sample_sub_dir("dummy_nested_five.chunk");
    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    let chunks: Vec<ChunkReader> = ChunkReader::from_slice(file)?.get_children_cloned();

    assert_eq!(chunks.len(), 5, "Expect five chunks");
    assert_eq!(chunks.get(0).unwrap().size, 8);
    assert_eq!(chunks.get(1).unwrap().size, 24);
    assert_eq!(chunks.get(2).unwrap().size, 16);
    assert_eq!(chunks.get(3).unwrap().size, 0);
    assert_eq!(chunks.get(4).unwrap().size, 40);

    Ok(())
  }
}
