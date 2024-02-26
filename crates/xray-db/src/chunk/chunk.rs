use crate::chunk::interface::ChunkDataSource;
use crate::chunk::iterator::ChunkIterator;
use crate::data::shape::Shape;
use crate::types::{Matrix3d, Sphere3d, U32Bytes, Vector3d};
use byteorder::{ByteOrder, ReadBytesExt};
use encoding_rs::WINDOWS_1251;
use fileslice::FileSlice;
use std::borrow::Cow;
use std::io::{Read, Seek, SeekFrom};
use std::{fmt, io};

#[derive(Clone, PartialEq)]
pub struct Chunk<T: ChunkDataSource = FileSlice> {
  pub index: u32,
  pub size: u64,
  pub position: u64,
  pub is_compressed: bool,
  pub file: Box<T>,
}

impl Chunk {
  /// Read all chunk descriptors from file and put seek into the end.
  pub fn read_all_from_file(chunk: &mut Chunk) -> Vec<Chunk> {
    ChunkIterator::new(chunk).collect()
  }

  pub fn from_file(file: FileSlice) -> io::Result<Chunk> {
    if file.is_empty() {
      return Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Trying to create chunk from empty file.",
      ));
    }

    Ok(Chunk {
      index: 0,
      size: file.len() as u64,
      position: file.start_pos(),
      is_compressed: false,
      file: Box::new(file),
    })
  }
}

impl Chunk {
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

impl Chunk {
  /// Navigates to chunk with index and constructs chunk representation.
  pub fn read_child_by_index(&mut self, index: u32) -> io::Result<Chunk> {
    for (iteration, chunk) in ChunkIterator::new(self).enumerate() {
      if index as usize == iteration {
        return Ok(chunk);
      }
    }

    Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      String::from("Attempt to read chunk with index out of bonds."),
    ))
  }

  /// Get list of all child chunks in current chunk.
  pub fn read_all_children(&self) -> Vec<Chunk> {
    ChunkIterator::new(&mut self.clone()).collect()
  }

  /// Reset seek position in chunk file.
  #[allow(dead_code)]
  pub fn reset_pos(&mut self) -> io::Result<u64> {
    self.file.seek(SeekFrom::Start(0))
  }
}

impl Chunk {
  /// Read three float values.
  pub fn read_f32_3d_vector<T: ByteOrder>(&mut self) -> io::Result<Vector3d<f32>> {
    Ok((
      self.read_f32::<T>()?,
      self.read_f32::<T>()?,
      self.read_f32::<T>()?,
    ))
  }

  pub fn read_u32_bytes(&mut self) -> io::Result<U32Bytes> {
    Ok((
      self.read_u8()?,
      self.read_u8()?,
      self.read_u8()?,
      self.read_u8()?,
    ))
  }

  /// Read serialized vector from chunk, where u32 count N is followed by N u16 entries.
  pub fn read_u16_vector<T: ByteOrder>(&mut self) -> io::Result<Vec<u16>> {
    let mut vector: Vec<u16> = Vec::new();
    let count: u32 = self.read_u32::<T>()?;

    for _ in 0..count {
      vector.push(self.read_u16::<T>()?)
    }

    Ok(vector)
  }

  /// Read null terminated windows encoded string from file bytes.
  pub fn read_null_terminated_win_string(&mut self) -> io::Result<String> {
    let offset: u64 = self.file.stream_position()?;
    let mut buffer: Vec<u8> = Vec::new();

    self.file.read_to_end(&mut buffer)?;

    if let Some(position) = buffer.iter().position(|&x| x == 0x00) {
      let slice: &[u8] = &buffer[..position];
      let (transformed, _encoding_used, had_errors) = WINDOWS_1251.decode(slice);

      if had_errors {
        panic!("Unexpected errors when decoding windows-1251 string data.");
      }

      // Try with windows 1251 conversion:
      let value: String = match transformed {
        Cow::Borrowed(value) => String::from(value),
        Cow::Owned(value) => value,
      };

      // Put seek right after string - length plus zero terminator.
      self
        .file
        .seek(SeekFrom::Start(offset + position as u64 + 1))
        .expect("Correct object seek movement.");

      Ok(value)
    } else {
      panic!("No null terminator found in file");
    }
  }

  /// Read shape data.
  pub fn read_shape_description<T: ByteOrder>(&mut self) -> io::Result<Vec<Shape>> {
    let mut shapes: Vec<Shape> = Vec::new();
    let count: u8 = self.read_u8().expect("Count flag to be read.");

    for _ in 0..count {
      match self.read_u8().expect("Shape type to be read.") {
        0 => shapes.push(Shape::Sphere(self.read_sphere::<T>()?)),
        1 => shapes.push(Shape::Box(self.read_matrix::<T>()?)),
        _ => panic!("Unexpected shape type provided"),
      }
    }

    Ok(shapes)
  }

  pub fn read_sphere<T: ByteOrder>(&mut self) -> io::Result<Sphere3d> {
    let center: Vector3d = self.read_f32_3d_vector::<T>()?;
    let radius: f32 = self.read_f32::<T>()?;

    Ok((center, radius))
  }

  pub fn read_matrix<T: ByteOrder>(&mut self) -> io::Result<Matrix3d> {
    Ok((
      self.read_f32_3d_vector::<T>()?,
      self.read_f32_3d_vector::<T>()?,
      self.read_f32_3d_vector::<T>()?,
      self.read_f32_3d_vector::<T>()?,
    ))
  }

  pub fn read_bytes(&mut self, count: usize) -> io::Result<Vec<u8>> {
    let mut buffer: Vec<u8> = vec![0; count];

    self.read_exact(&mut buffer)?;

    Ok(buffer)
  }
}

impl fmt::Debug for Chunk {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "Chunk {{ index: {}, size: {}, position: {}, is_compressed: {} }}",
      self.index, self.size, self.position, self.is_compressed
    )
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::test::utils::{get_test_chunk_sub_dir, open_test_resource_as_slice};
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_empty_file() -> io::Result<()> {
    let file: FileSlice = open_test_resource_as_slice(&String::from("chunks/empty"))?;

    assert_eq!(file.start_pos(), 0);
    assert_eq!(file.end_pos(), 0);

    let result: io::Result<Chunk> = Chunk::from_file(file);

    assert!(
      result.is_err(),
      "File should be empty and fail to read data."
    );
    assert_eq!(
      result.unwrap_err().kind(),
      io::ErrorKind::InvalidInput,
      "Expect input error."
    );

    Ok(())
  }

  #[test]
  fn test_read_empty_chunk() -> io::Result<()> {
    let filename: String = get_test_chunk_sub_dir(&String::from("empty_nested_single.chunk"));
    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.start_pos(), 0);
    assert_eq!(file.end_pos(), 8);

    let chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;

    assert!(chunk.is_ended(), "Expect empty chunk.");

    Ok(())
  }

  #[test]
  fn test_read_empty_children() -> io::Result<()> {
    let filename: String = get_test_chunk_sub_dir(&String::from("empty_nested_single.chunk"));
    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    let chunks: Vec<Chunk> = Chunk::from_file(file)?.read_all_children();

    assert_eq!(chunks.len(), 1, "Expect single chunk.");
    assert_eq!(chunks.first().unwrap().size, 0);

    let filename: String = get_test_chunk_sub_dir(&String::from("empty_nested_five.chunk"));
    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    let chunks: Vec<Chunk> = Chunk::from_file(file)?.read_all_children();

    assert_eq!(chunks.len(), 5, "Expect five chunks.");
    assert_eq!(chunks.get(0).unwrap().size, 0);
    assert_eq!(chunks.get(1).unwrap().size, 0);
    assert_eq!(chunks.get(2).unwrap().size, 0);
    assert_eq!(chunks.get(3).unwrap().size, 0);
    assert_eq!(chunks.get(4).unwrap().size, 0);

    Ok(())
  }

  #[test]
  fn test_read_dummy_children() -> io::Result<()> {
    let filename: String = get_test_chunk_sub_dir(&String::from("dummy_nested_single.chunk"));
    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    let chunks: Vec<Chunk> = Chunk::from_file(file)?.read_all_children();

    assert_eq!(chunks.len(), 1, "Expect single chunk.");
    assert_eq!(chunks.first().unwrap().size, 8);

    let filename: String = get_test_chunk_sub_dir(&String::from("dummy_nested_five.chunk"));
    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    let chunks: Vec<Chunk> = Chunk::from_file(file)?.read_all_children();

    assert_eq!(chunks.len(), 5, "Expect five chunks.");
    assert_eq!(chunks.get(0).unwrap().size, 8);
    assert_eq!(chunks.get(1).unwrap().size, 24);
    assert_eq!(chunks.get(2).unwrap().size, 16);
    assert_eq!(chunks.get(3).unwrap().size, 0);
    assert_eq!(chunks.get(4).unwrap().size, 40);

    Ok(())
  }
}
