use crate::spawn::types::{U32Bytes, Vector3d};
use crate::spawn::utils::{read_f32_vector, read_u32_bytes};
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;

#[derive(Debug)]
pub struct Vertex {
  pub level_point: Vector3d,
  pub game_point: Vector3d,
  pub level_id: u8,
  pub level_vertex_id: u32, // u24
  pub vertex_type: U32Bytes,
  pub edge_offset: u32,
  pub level_point_offset: u32,
  pub edge_count: u8,
  pub level_point_count: u8,
}

impl Vertex {
  pub fn from_file(file: &mut FileSlice) -> Vertex {
    let level_point: Vector3d = read_f32_vector::<LittleEndian>(file);
    let game_point: Vector3d = read_f32_vector::<LittleEndian>(file);
    let level_id: u8 = file.read_u8().unwrap();
    let level_vertex_id: u32 = file.read_u24::<LittleEndian>().unwrap();
    let vertex_type: U32Bytes = read_u32_bytes(file);
    let edge_offset: u32 = file.read_u32::<LittleEndian>().unwrap();
    let level_point_offset: u32 = file.read_u32::<LittleEndian>().unwrap();
    let edge_count: u8 = file.read_u8().unwrap();
    let level_point_count: u8 = file.read_u8().unwrap();

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
