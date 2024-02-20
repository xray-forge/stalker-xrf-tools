use crate::chunk::chunk::Chunk;
use crate::types::Vector3d;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

#[derive(Debug)]
pub struct ArtefactSpawnPoint {
  pub position: (f32, f32, f32),
  pub level_vertex_id: u32,
  pub distance: f32,
}

impl ArtefactSpawnPoint {
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<ArtefactSpawnPoint> {
    let position: Vector3d = chunk.read_f32_3d_vector::<T>()?;
    let level_vertex_id: u32 = chunk.read_u32::<T>()?;
    let distance: f32 = chunk.read_f32::<T>()?;

    Ok(ArtefactSpawnPoint {
      position,
      level_vertex_id,
      distance,
    })
  }
}
