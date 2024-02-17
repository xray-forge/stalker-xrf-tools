use crate::chunk::chunk::Chunk;
use crate::types::{SpawnByteOrder, Vector3d};
use byteorder::ReadBytesExt;
use std::fmt;

pub struct ArtefactSpawnsChunk {
  pub index: u32,
  pub nodes: Vec<ArtefactSpawnPoint>,
}

impl ArtefactSpawnsChunk {
  /// Read header chunk by position descriptor.
  pub fn from_chunk(mut chunk: Chunk) -> Option<ArtefactSpawnsChunk> {
    let mut nodes: Vec<ArtefactSpawnPoint> = Vec::new();

    let count: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();

    log::info!("Parsing artefacts: {count}, {:?}", chunk.size / 20);

    assert_eq!(count, chunk.size / 20);

    // Parsing CLevelPoint structure, 20 bytes per one.
    for _ in 0..count {
      let position: Vector3d = chunk.read_f32_vector::<SpawnByteOrder>().unwrap();
      let level_vertex_id: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
      let distance: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();

      nodes.push(ArtefactSpawnPoint::new(position, level_vertex_id, distance))
    }

    log::info!(
      "Parsed artefacts spawns: {:?} processed, {:?} remain",
      chunk.read_bytes_len(),
      chunk.read_bytes_remain(),
    );

    assert!(chunk.is_ended());

    return Some(ArtefactSpawnsChunk {
      index: chunk.index,
      nodes,
    });
  }
}

impl fmt::Debug for ArtefactSpawnsChunk {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "ArtefactSpawnsChunk {{ index: {}, nodes: Vector[{}] }}",
      self.index,
      self.nodes.len()
    )
  }
}

#[derive(Debug)]
pub struct ArtefactSpawnPoint {
  pub position: (f32, f32, f32),
  pub level_vertex_id: u32,
  pub distance: f32,
}

impl ArtefactSpawnPoint {
  pub fn new(position: (f32, f32, f32), level_vertex_id: u32, distance: f32) -> ArtefactSpawnPoint {
    ArtefactSpawnPoint {
      position,
      level_vertex_id,
      distance,
    }
  }
}
