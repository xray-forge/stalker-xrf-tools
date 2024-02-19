use crate::chunk::chunk::Chunk;
use crate::chunk::iterator::ChunkIterator;
use crate::data::patrol::Patrol;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;
use std::fmt;

pub struct PatrolsChunk {
  pub chunk: Chunk,
  pub patrols: Vec<Patrol>,
}

impl PatrolsChunk {
  /// Read patrols chunk by position descriptor.
  pub fn from_chunk(mut chunk: Chunk) -> Option<PatrolsChunk> {
    log::info!(
      "Parsing patrols: {:?} -> {:?}",
      chunk.start_pos(),
      chunk.end_pos()
    );

    let count: u32 = Self::read_patrols_count(&mut chunk);
    let patrols: Vec<Patrol> = Self::read_patrols(&mut chunk, count);

    log::info!(
      "Parsed patrols: {:?} / {count}, {:?} bytes",
      patrols.len(),
      chunk.read_bytes_len()
    );

    assert!(chunk.is_ended());
    assert_eq!(count, patrols.len() as u32);

    Some(PatrolsChunk { chunk, patrols })
  }

  fn read_patrols_count(chunk: &mut Chunk) -> u32 {
    let mut base_chunk: Chunk = chunk.read_child_by_index(0).unwrap();

    assert_eq!(base_chunk.size, 4);

    base_chunk.read_u32::<SpawnByteOrder>().unwrap()
  }

  fn read_patrols(chunk: &mut Chunk, count: u32) -> Vec<Patrol> {
    let mut patrols_chunk: Chunk = chunk.read_child_by_index(1).unwrap();
    let mut patrols: Vec<Patrol> = Vec::new();
    let mut index: u32 = 0;

    for mut patrol_chunk in ChunkIterator::new(&mut patrols_chunk) {
      patrols.push(Patrol::from_chunk(&mut patrol_chunk));
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
      "PatrolsChunk {{ chunk: {:?}, patrols: Vector[{}] }}",
      self.chunk,
      self.patrols.len(),
    )
  }
}
