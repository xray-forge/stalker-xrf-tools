use crate::chunk::chunk::Chunk;
use crate::data::artefact_spawn_point::ArtefactSpawnPoint;
use byteorder::{ByteOrder, ReadBytesExt};
use std::{fmt, io};

pub struct ArtefactSpawnsChunk {
  pub chunk: Chunk,
  pub nodes: Vec<ArtefactSpawnPoint>,
}

impl ArtefactSpawnsChunk {
  /// Read header chunk by position descriptor.
  pub fn read_from_chunk<T: ByteOrder>(mut chunk: Chunk) -> io::Result<ArtefactSpawnsChunk> {
    let mut nodes: Vec<ArtefactSpawnPoint> = Vec::new();
    let count: u32 = chunk.read_u32::<T>()?;

    log::info!("Parsing artefacts: {count}, {:?}", chunk.size / 20);

    assert_eq!(count as u64, chunk.size / 20);

    // Parsing CLevelPoint structure, 20 bytes per one.
    for _ in 0..count {
      nodes.push(ArtefactSpawnPoint::read_from_chunk::<T>(&mut chunk)?);
    }

    log::info!(
      "Parsed artefacts spawns: {:?} processed, {:?} remain",
      chunk.read_bytes_len(),
      chunk.read_bytes_remain(),
    );

    assert!(chunk.is_ended());

    Ok(ArtefactSpawnsChunk { chunk, nodes })
  }
}

impl fmt::Debug for ArtefactSpawnsChunk {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "ArtefactSpawnsChunk {{ chunk: {:?}, nodes: Vector[{}] }}",
      self.chunk,
      self.nodes.len()
    )
  }
}
