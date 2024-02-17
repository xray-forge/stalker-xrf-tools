use crate::spawn::chunk::iterator::ChunkIterator;
use crate::spawn::types::{U32Bytes, Vector3d};
use byteorder::{ByteOrder, ReadBytesExt};
use fileslice::FileSlice;
use std::io;
use std::io::{Read, Seek, SeekFrom};

#[derive(Clone, Debug)]
pub struct Chunk {
  pub id: u32,
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
      let value: String =
        String::from_utf8(buffer[..position].to_vec()).expect("Correct string read.");

      // Put seek right after string - length plus zero terminator.
      self
        .file
        .seek(SeekFrom::Start(offset + value.len() as u64 + 1))
        .expect("Correct object seek movement.");

      return Ok(value);
    } else {
      panic!("No null terminator found in file");
    }
  }

  /// Read shape data.
  pub fn read_shape_description<T: ByteOrder>(&mut self) -> io::Result<Vec<f32>> {
    let mut shape: Vec<f32> = Vec::new();
    let count: u8 = self.read_u8()?;

    assert_eq!(count, 1, "Single shape description expected.");

    let shape_type: u8 = self.read_u8()?;

    match shape_type {
      0 => {
        for _ in 0..4 {
          shape.push(self.read_f32::<T>()?)
        }
      }
      1 => {
        for _ in 0..12 {
          shape.push(self.read_f32::<T>()?)
        }
      }
      _ => panic!("Unexpected shape type provided"),
    }

    Ok(shape)
  }
}
