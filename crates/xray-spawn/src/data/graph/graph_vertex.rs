use crate::chunk::chunk::Chunk;
use crate::types::{U32Bytes, Vector3d};
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

#[derive(Debug)]
pub struct GraphVertex {
  pub level_point: Vector3d,
  pub game_point: Vector3d,
  pub level_id: u8,
  pub level_vertex_id: u32, // todo:CORE::unpack('V', CORE::pack('CCCC', $self->unpack('C3', 3), 0));
  pub vertex_type: U32Bytes,
  pub edge_offset: u32,
  pub level_point_offset: u32,
  pub edge_count: u8,
  pub level_point_count: u8,
}

impl GraphVertex {
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<GraphVertex> {
    let level_point: Vector3d = chunk.read_f32_3d_vector::<T>()?;
    let game_point: Vector3d = chunk.read_f32_3d_vector::<T>()?;
    let level_id: u8 = chunk.read_u8()?;
    let level_vertex_id: u32 = chunk.read_u32::<T>()?;
    let vertex_type: U32Bytes = chunk.read_u32_bytes()?;
    let edge_offset: u32 = chunk.read_u32::<T>()?;
    let level_point_offset: u32 = chunk.read_u32::<T>()?;
    let edge_count: u8 = chunk.read_u8()?;
    let level_point_count: u8 = chunk.read_u8()?;

    Ok(GraphVertex {
      level_point,
      game_point,
      level_id,
      level_vertex_id,
      vertex_type,
      edge_offset,
      level_point_offset,
      edge_count,
      level_point_count,
    })
  }
}
