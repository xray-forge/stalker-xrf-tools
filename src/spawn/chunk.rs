use crate::spawn::constants::CFS_COMPRESS_MARK;
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;
use std::io::{Seek, SeekFrom};

/// Root level chunks by ID:
/// 0 - header
/// 1 - alife spawns
/// 2 - alife objects
/// 3 - patrols
/// 4 - game graphs
#[derive(Debug)]
pub struct Chunk {
  pub id: u32,
  pub size: u32,
  pub position: u64,
  pub is_compressed: bool,
}

impl Chunk {
  pub fn read_all(file: &mut FileSlice) -> Vec<Chunk> {
    ChunkIterator::new(file).into_iter().collect()
  }

  /// Navigates to chunk with index and constructs chunk representation.
  pub fn read_by_index(file: &mut FileSlice, index: u32) -> Option<Chunk> {
    for (iteration, chunk) in ChunkIterator::new(file).enumerate() {
      if index as usize == iteration {
        return Some(chunk);
      }
    }

    None
  }

  /// Navigates to chunk with index and provides chunk file slice.
  pub fn open_by_index(file: &mut FileSlice, index: u32) -> Option<(FileSlice, Chunk)> {
    Self::read_by_index(file, index).and_then(|chunk| Some((chunk.in_slice(file), chunk)))
  }
}

impl Chunk {
  pub fn in_slice(&self, file: &FileSlice) -> FileSlice {
    let mut slice: FileSlice = file.slice(self.position..(self.position + self.size as u64));

    slice.seek(SeekFrom::Start(0)).unwrap();

    return slice;
  }

  #[allow(dead_code)]
  pub fn read_children(&self, file: &FileSlice) -> Vec<Chunk> {
    ChunkIterator::new(&mut file.slice(self.position..(self.position + self.size as u64)))
      .into_iter()
      .collect()
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
    let chunk_type = self.file.read_u32::<LittleEndian>();
    let chunk_size = self.file.read_u32::<LittleEndian>();

    if chunk_type.is_err() || chunk_size.is_err() {
      return None;
    }

    let chunk_id: u32 = chunk_type.unwrap();
    let chunk_size: u32 = chunk_size.unwrap();

    return if self.index == chunk_id & (!CFS_COMPRESS_MARK) {
      let chunk = Chunk {
        id: chunk_id,
        is_compressed: chunk_id & CFS_COMPRESS_MARK == 1,
        size: chunk_size,
        position: self.file.seek(SeekFrom::Current(0)).unwrap(),
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

/// Iterates over slice and chunk for provided file entry.
#[derive(Debug)]
pub struct ChunkSliceIterator<'lifetime> {
  pub base: ChunkIterator<'lifetime>,
}

impl<'lifetime> ChunkSliceIterator<'lifetime> {
  pub fn new(file: &mut FileSlice) -> ChunkSliceIterator {
    return ChunkSliceIterator {
      base: ChunkIterator::new(file),
    };
  }
}

/// Iterates over chunk and read child chunks.
impl<'lifetime> Iterator for ChunkSliceIterator<'lifetime> {
  type Item = (FileSlice, Chunk);

  fn next(&mut self) -> Option<(FileSlice, Chunk)> {
    let next: Option<Chunk> = self.base.next();

    match next {
      Some(chunk) => Some((chunk.in_slice(self.base.file), chunk)),
      None => None,
    }
  }
}
