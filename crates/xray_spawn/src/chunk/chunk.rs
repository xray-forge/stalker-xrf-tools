use crate::chunk::iterator::ChunkIterator;
use crate::data::shape::Shape;
use crate::types::{Matrix3d, Sphere3d, U32Bytes, Vector3d};
use byteorder::{ByteOrder, ReadBytesExt};
use encoding_rs::WINDOWS_1251;
use fileslice::FileSlice;
use std::borrow::Cow;
use std::io::{Read, Seek, SeekFrom};
use std::string::FromUtf8Error;
use std::{fmt, io};

#[derive(Clone)]
pub struct Chunk {
  pub index: u32,
  pub size: u32,
  pub position: u64,
  pub is_compressed: bool,
  pub file: FileSlice,
}

impl Chunk {
  /// Read all chunk descriptors from file and put seek into the end.
  pub fn read_all_from_file(file: &mut FileSlice) -> Vec<Chunk> {
    ChunkIterator::new(file).into_iter().collect()
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
  pub fn read_child_by_index(&mut self, index: u32) -> Option<Chunk> {
    for (iteration, chunk) in ChunkIterator::new(&mut self.file).enumerate() {
      if index as usize == iteration {
        return Some(chunk);
      }
    }

    None
  }

  /// Get list of all child chunks in current chunk.
  #[allow(dead_code)]
  pub fn read_all_children(&self, file: &FileSlice) -> Vec<Chunk> {
    ChunkIterator::new(&mut file.slice(self.position..(self.position + self.size as u64)))
      .into_iter()
      .collect()
  }

  /// Reset seek position in chunk file.
  #[allow(dead_code)]
  pub fn reset_pos(&mut self) -> io::Result<u64> {
    self.file.seek(SeekFrom::Start(0))
  }
}

impl Chunk {
  /// Read three float values.
  pub fn read_f32_vector<T: ByteOrder>(&mut self) -> io::Result<Vector3d<f32>> {
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

  /// Read null terminated string from file bytes.
  pub fn read_null_terminated_string(&mut self) -> io::Result<String> {
    let offset: u64 = self.file.seek(SeekFrom::Current(0))?;
    let mut buffer: Vec<u8> = Vec::new();

    self.file.read_to_end(&mut buffer)?;

    if let Some(position) = buffer.iter().position(|&x| x == 0x00) {
      let slice: &[u8] = &buffer[..position];
      let value: Result<String, FromUtf8Error> = String::from_utf8(slice.to_vec());
      let value: String = match value {
        Ok(it) => it,
        Err(_) => {
          let (transforemed, _encoding_used, had_errors) = WINDOWS_1251.decode(slice);

          if had_errors {
            panic!("Unexpected errors when coverting windows-1251 string data.");
          }

          // Retry with windows 1251 conversion:
          match transforemed {
            Cow::Borrowed(value) => String::from(value),
            Cow::Owned(value) => value,
          }
        }
      };

      // Put seek right after string - length plus zero terminator.
      self
        .file
        .seek(SeekFrom::Start(offset + position as u64 + 1))
        .expect("Correct object seek movement.");

      return Ok(value);
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
    let center: Vector3d = self.read_f32_vector::<T>()?;
    let radius: f32 = self.read_f32::<T>()?;

    Ok((center, radius))
  }

  pub fn read_matrix<T: ByteOrder>(&mut self) -> io::Result<Matrix3d> {
    Ok((
      self.read_f32_vector::<T>()?,
      self.read_f32_vector::<T>()?,
      self.read_f32_vector::<T>()?,
      self.read_f32_vector::<T>()?,
    ))
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
