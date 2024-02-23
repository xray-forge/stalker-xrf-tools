use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use crate::types::{SpawnByteOrder, Vector3d};
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeLevelChanger {
  pub base: AlifeObjectSpaceRestrictor,
  pub dest_game_vertex_id: u16,
  pub dest_level_vertex_id: u32,
  pub dest_position: Vector3d,
  pub dest_direction: Vector3d,
  pub angle_y: f32,
  pub dest_level_name: String,
  pub dest_graph_point: String,
  pub silent_mode: u8,
  pub enabled: u8,
  pub hint: String,
}

impl AlifeObjectInheritedReader<AlifeLevelChanger> for AlifeLevelChanger {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeLevelChanger> {
    let base: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor::read_from_chunk::<T>(chunk)?;

    let dest_game_vertex_id: u16 = chunk.read_u16::<SpawnByteOrder>()?;
    let dest_level_vertex_id: u32 = chunk.read_u32::<SpawnByteOrder>()?;
    let dest_position: Vector3d = chunk.read_f32_3d_vector::<SpawnByteOrder>()?;
    let dest_direction: Vector3d = chunk.read_f32_3d_vector::<SpawnByteOrder>()?;
    let angle_y: f32 = chunk.read_f32::<SpawnByteOrder>()?;
    let dest_level_name: String = chunk.read_null_terminated_string()?;
    let dest_graph_point: String = chunk.read_null_terminated_string()?;
    let silent_mode: u8 = chunk.read_u8()?;

    let enabled: u8 = chunk.read_u8()?;
    let hint: String = chunk.read_null_terminated_string()?;
    let save_marker: u16 = chunk.read_u16::<SpawnByteOrder>()?;

    assert_eq!(
      save_marker, 26,
      "Unexpected script data provided for level changer."
    );

    Ok(AlifeLevelChanger {
      base,
      dest_game_vertex_id,
      dest_level_vertex_id,
      dest_position,
      dest_direction,
      angle_y,
      dest_level_name,
      dest_graph_point,
      silent_mode,
      enabled,
      hint,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeLevelChanger {}
