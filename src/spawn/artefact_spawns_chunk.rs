use crate::spawn::chunk::Chunk;
use crate::spawn::chunk_utils::read_f32_vector;
use crate::spawn::types::Vector3d;
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;
use std::fmt;

pub struct ArtefactSpawnsChunk {
  pub id: u32,
  pub nodes: Vec<ArtefactSpawnPoint>,
}

impl ArtefactSpawnsChunk {
  /// Read header chunk by position descriptor.
  pub fn from_chunk(file: &mut FileSlice, chunk: &Chunk) -> Option<ArtefactSpawnsChunk> {
    let mut file: FileSlice = chunk.in_slice(file);
    let mut nodes: Vec<ArtefactSpawnPoint> = Vec::new();

    let count: u32 = file.read_u32::<LittleEndian>().unwrap();

    log::info!("Parsing artefacts {count} | {:?}", chunk.size / 20);

    assert_eq!(count, chunk.size / 20);

    // Parsing CLevelPoint structure, 20 bytes per one.
    for _ in 0..count {
      let position: Vector3d = read_f32_vector(&mut file);
      let level_vertex_id: u32 = file.read_u32::<LittleEndian>().unwrap();
      let distance: f32 = file.read_f32::<LittleEndian>().unwrap();

      nodes.push(ArtefactSpawnPoint::new(position, level_vertex_id, distance))
    }

    log::info!(
      "Parsed artefacts spawns {:?} / {:?}",
      file.cursor_pos(),
      file.end_pos()
    );

    return Some(ArtefactSpawnsChunk {
      id: chunk.id,
      nodes,
    });
  }
}

impl fmt::Debug for ArtefactSpawnsChunk {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "ArtefactSpawnsChunk {{ id: {}, nodes: Vector[{}] }}",
      self.id,
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
