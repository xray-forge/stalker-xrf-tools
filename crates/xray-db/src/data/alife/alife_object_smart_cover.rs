use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_dynamic::AlifeObjectDynamic;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::shape::Shape;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectSmartCover {
  pub base: AlifeObjectDynamic,
  pub shape: Vec<Shape>,
  pub description: String,
  pub hold_position_time: f32,
  pub enter_min_enemy_distance: f32,
  pub exit_min_enemy_distance: f32,
  pub is_combat_cover: u8,
  pub can_fire: u8,
}

impl AlifeObjectInheritedReader<AlifeObjectSmartCover> for AlifeObjectSmartCover {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectSmartCover> {
    let base: AlifeObjectDynamic = AlifeObjectDynamic::read_from_chunk::<T>(chunk)?;

    let shape: Vec<Shape> = chunk.read_shape_description::<SpawnByteOrder>()?;
    let description: String = chunk.read_null_terminated_string()?;
    let hold_position_time: f32 = chunk.read_f32::<SpawnByteOrder>()?;
    let enter_min_enemy_distance: f32 = chunk.read_f32::<SpawnByteOrder>()?;
    let exit_min_enemy_distance: f32 = chunk.read_f32::<SpawnByteOrder>()?;
    let is_combat_cover: u8 = chunk.read_u8()?;
    let can_fire: u8 = chunk.read_u8()?;

    Ok(AlifeObjectSmartCover {
      base,
      shape,
      description,
      hold_position_time,
      enter_min_enemy_distance,
      exit_min_enemy_distance,
      is_combat_cover,
      can_fire,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}
