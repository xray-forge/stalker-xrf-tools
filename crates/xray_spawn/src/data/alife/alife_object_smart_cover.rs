use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_dynamic::AlifeObjectDynamic;
use crate::data::alife_object::AlifeObjectInherited;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeObjectSmartCover {
  pub base: AlifeObjectDynamic,
  pub shape: Vec<f32>,
  pub description: String,
  pub hold_position_time: f32,
  pub enter_min_enemy_distance: f32,
  pub exit_min_enemy_distance: f32,
  pub is_combat_cover: u8,
  pub can_fire: u8,
}

impl AlifeObjectInherited<AlifeObjectSmartCover> for AlifeObjectSmartCover {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectSmartCover {
    let base: AlifeObjectDynamic = AlifeObjectDynamic::from_chunk(chunk);

    let shape: Vec<f32> = chunk.read_shape_description::<SpawnByteOrder>().unwrap();
    let description: String = chunk.read_null_terminated_string().unwrap();
    let hold_position_time: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let enter_min_enemy_distance: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let exit_min_enemy_distance: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let is_combat_cover: u8 = chunk.read_u8().unwrap();
    let can_fire: u8 = chunk.read_u8().unwrap();

    AlifeObjectSmartCover {
      base,
      shape,
      description,
      hold_position_time,
      enter_min_enemy_distance,
      exit_min_enemy_distance,
      is_combat_cover,
      can_fire,
    }
  }
}
