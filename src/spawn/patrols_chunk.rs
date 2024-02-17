use crate::spawn::chunk::{Chunk, ChunkSliceIterator};
use crate::spawn::data::patrol::Patrol;
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;
use std::fmt;

pub struct PatrolsChunk {
  id: u32,
  patrols: Vec<Patrol>,
}

impl PatrolsChunk {
  /// Read patrols chunk by position descriptor.
  pub fn from_chunk(file: &mut FileSlice, chunk: &Chunk) -> Option<PatrolsChunk> {
    let mut file: FileSlice = chunk.in_slice(file);

    log::info!(
      "Parsing patrols: {:?} -> {:?}",
      file.start_pos(),
      file.end_pos()
    );

    let count: u32 = Self::read_patrols_count(&mut file);
    let patrols: Vec<Patrol> = Self::read_patrols(&mut file, count);

    log::info!(
      "Parsed patrols: {:?} / {count}, {:?} bytes",
      patrols.len(),
      file.end_pos() - file.start_pos()
    );

    assert_eq!(count, patrols.len() as u32);
    assert_eq!(file.cursor_pos(), file.end_pos());

    return Some(PatrolsChunk {
      id: chunk.id,
      patrols,
    });
  }

  fn read_patrols_count(file: &mut FileSlice) -> u32 {
    let mut base_chunk: Chunk = Chunk::read_by_index(file, 0).unwrap();

    assert_eq!(base_chunk.size, 4);

    base_chunk.file.read_u32::<LittleEndian>().unwrap()
  }

  fn read_patrols(file: &mut FileSlice, count: u32) -> Vec<Patrol> {
    let mut patrols: Vec<Patrol> = Vec::new();
    let mut index: u32 = 0;

    let mut patrols_chunk: Chunk = Chunk::read_by_index(file, 1).unwrap();

    for (mut slice, _) in ChunkSliceIterator::new(&mut patrols_chunk.file) {
      patrols.push(Patrol::read(&mut slice));
      index += 1;
    }

    assert_eq!(index, count);

    patrols
  }
}

impl fmt::Debug for PatrolsChunk {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "PatrolsChunk {{ id: {}, patrols: Vector[{}] }}",
      self.id,
      self.patrols.len(),
    )
  }
}
