use crate::spawn::constants::CFS_COMPRESS_MARK;
use crate::spawn::types::{SpawnByteOrder, U32Bytes, Vector3d};
use byteorder::{ByteOrder, ReadBytesExt};
use bytes::Bytes;
use fileslice::FileSlice;
use parquet::file::reader::{ChunkReader, Length};
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
  pub fn read_all_children(file: &mut FileSlice) -> Vec<Chunk> {
    ChunkIterator::new(file).into_iter().collect()
  }
}

impl Chunk {
  pub fn start_pos(&self) -> u64 {
    self.file.start_pos()
  }

  pub fn end_pos(&self) -> u64 {
    self.file.end_pos()
  }

  #[allow(dead_code)]
  pub fn cursor_pos(&self) -> u64 {
    self.file.cursor_pos()
  }

  pub fn read_bytes_len(&self) -> u64 {
    self.file.cursor_pos() - self.file.start_pos()
  }

  pub fn read_bytes_remain(&self) -> u64 {
    self.file.end_pos() - self.file.cursor_pos()
  }
}

impl Chunk {
  /// Navigates to chunk with index and constructs chunk representation.
  pub fn read_by_index(&mut self, index: u32) -> Option<Chunk> {
    for (iteration, chunk) in ChunkIterator::new(&mut self.file).enumerate() {
      if index as usize == iteration {
        return Some(chunk);
      }
    }

    None
  }

  #[allow(dead_code)]
  pub fn read_children(&self, file: &FileSlice) -> Vec<Chunk> {
    ChunkIterator::new(&mut file.slice(self.position..(self.position + self.size as u64)))
      .into_iter()
      .collect()
  }

  #[allow(dead_code)]
  pub fn reset(&mut self) -> io::Result<u64> {
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

impl Length for Chunk {
  fn len(&self) -> u64 {
    self.file.end_pos() - self.file.start_pos()
  }
}

impl ChunkReader for Chunk {
  type T = FileSlice;

  fn get_read(&self, start: u64) -> parquet::errors::Result<FileSlice> {
    Ok(self.file.slice(start..self.file.end_pos()))
  }

  fn get_bytes(&self, start: u64, length: usize) -> parquet::errors::Result<Bytes> {
    let mut buf = vec![0; length];
    self
      .file
      .slice(start..(start + length as u64))
      .read_exact(&mut buf)?;
    Ok(buf.into())
  }
}

impl Read for Chunk {
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    return self.file.read(buf);
  }
}

#[derive(Debug)]
pub struct ChunkIterator<'lifetime> {
  pub index: u32,
  pub file: &'lifetime mut FileSlice,
}

impl<'lifetime> ChunkIterator<'lifetime> {
  pub fn new(file: &mut FileSlice) -> ChunkIterator {
    file.seek(SeekFrom::Start(0)).unwrap();

    return ChunkIterator { index: 0, file };
  }
}

/// Iterates over chunk and read child chunks.
impl<'lifetime> Iterator for ChunkIterator<'lifetime> {
  type Item = Chunk;

  fn next(&mut self) -> Option<Chunk> {
    let chunk_type = self.file.read_u32::<SpawnByteOrder>();
    let chunk_size = self.file.read_u32::<SpawnByteOrder>();

    if chunk_type.is_err() || chunk_size.is_err() {
      return None;
    }

    let chunk_id: u32 = chunk_type.unwrap();
    let chunk_size: u32 = chunk_size.unwrap();

    return if self.index == chunk_id & (!CFS_COMPRESS_MARK) {
      let position: u64 = self.file.seek(SeekFrom::Current(0)).unwrap();
      let mut file: FileSlice = self.file.slice(position..(position + chunk_size as u64));

      file.seek(SeekFrom::Start(0)).unwrap();

      let chunk = Chunk {
        id: chunk_id,
        is_compressed: chunk_id & CFS_COMPRESS_MARK == 1,
        size: chunk_size,
        position: self.file.seek(SeekFrom::Current(0)).unwrap(),
        file,
      };

      if chunk.is_compressed {
        panic!("Parsing not implemented compressed chunk.");
      }

      // Rewind for next iteration.
      self
        .file
        .seek(SeekFrom::Current(chunk_size as i64))
        .unwrap();

      // Iterate to next item.
      self.index += 1;

      Some(chunk)
    } else {
      None
    };
  }
}
