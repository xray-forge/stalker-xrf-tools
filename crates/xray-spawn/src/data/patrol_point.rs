use crate::chunk::chunk::Chunk;
use crate::types::Vector3d;
use byteorder::{ByteOrder, ReadBytesExt};

#[derive(Debug)]
pub struct PatrolPoint {
  pub name: String,
  pub position: (f32, f32, f32),
  pub flags: u32,
  pub level_vertex_id: u32,
  pub game_vertex_id: u16,
}

impl PatrolPoint {
  pub fn from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> PatrolPoint {
    let name: String = chunk.read_null_terminated_string().unwrap();
    let position: Vector3d = chunk.read_f32_3d_vector::<T>().unwrap();
    let flags: u32 = chunk.read_u32::<T>().unwrap();
    let level_vertex_id: u32 = chunk.read_u32::<T>().unwrap();
    let game_vertex_id: u16 = chunk.read_u16::<T>().unwrap();

    PatrolPoint {
      name,
      position,
      flags,
      level_vertex_id,
      game_vertex_id,
    }
  }
}
