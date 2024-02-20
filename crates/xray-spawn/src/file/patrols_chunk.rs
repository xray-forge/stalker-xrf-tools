use crate::chunk::chunk::Chunk;
use crate::data::patrol::patrol::Patrol;
use byteorder::{ByteOrder, ReadBytesExt};
use std::{fmt, io};

pub struct PatrolsChunk {
  pub chunk: Chunk,
  pub patrols: Vec<Patrol>,
}

impl PatrolsChunk {
  /// Read patrols chunk by position descriptor.
  pub fn read_from_chunk<T: ByteOrder>(mut chunk: Chunk) -> io::Result<PatrolsChunk> {
    log::info!(
      "Parsing patrols: {:?} -> {:?}",
      chunk.start_pos(),
      chunk.end_pos()
    );

    let mut meta_chunk: Chunk = chunk.read_child_by_index(0)?;
    let mut data_chunk: Chunk = chunk.read_child_by_index(1)?;

    assert_eq!(meta_chunk.size, 4);

    let count: u32 = meta_chunk.read_u32::<T>()?;
    let patrols: Vec<Patrol> = Patrol::read_list_from_chunk::<T>(&mut data_chunk, count)?;

    assert_eq!(count, patrols.len() as u32);
    assert!(chunk.is_ended());

    log::info!(
      "Parsed patrols: {:?} / {count}, {:?} bytes",
      patrols.len(),
      chunk.read_bytes_len()
    );

    Ok(PatrolsChunk { chunk, patrols })
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
