use crate::chunk::chunk::Chunk;
use crate::types::{U32Bytes, Vector3d};
use byteorder::{ByteOrder, ReadBytesExt};

#[derive(Debug)]
pub struct Vertex {
  pub level_point: Vector3d,
  pub game_point: Vector3d,
  pub level_id: u8,
  pub level_vertex_id: u32, // todo: u24?
  pub vertex_type: U32Bytes,
  pub edge_offset: u32,
  pub level_point_offset: u32,
  pub edge_count: u8,
  pub level_point_count: u8,
}

impl Vertex {
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> Vertex {
    let level_point: Vector3d = chunk.read_f32_3d_vector::<T>().unwrap();
    let game_point: Vector3d = chunk.read_f32_3d_vector::<T>().unwrap();
    let level_id: u8 = chunk.read_u8().unwrap();
    let level_vertex_id: u32 = chunk.read_u24::<T>().unwrap();
    let vertex_type: U32Bytes = chunk.read_u32_bytes().unwrap();
    let edge_offset: u32 = chunk.read_u32::<T>().unwrap();
    let level_point_offset: u32 = chunk.read_u32::<T>().unwrap();
    let edge_count: u8 = chunk.read_u8().unwrap();
    let level_point_count: u8 = chunk.read_u8().unwrap();

    Vertex {
      level_point,
      game_point,
      level_id,
      level_vertex_id,
      vertex_type,
      edge_offset,
      level_point_offset,
      edge_count,
      level_point_count,
    }
  }
}
