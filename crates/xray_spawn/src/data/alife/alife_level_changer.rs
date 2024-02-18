use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use crate::data::alife_object::AlifeObjectInherited;
use crate::types::{SpawnByteOrder, Vector3d};
use byteorder::ReadBytesExt;

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

impl AlifeObjectInherited<AlifeLevelChanger> for AlifeLevelChanger {
  fn from_chunk(chunk: &mut Chunk) -> AlifeLevelChanger {
    let base: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor::from_chunk(chunk);

    let dest_game_vertex_id: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();
    let dest_level_vertex_id: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let dest_position: Vector3d = chunk.read_f32_3d_vector::<SpawnByteOrder>().unwrap();
    let dest_direction: Vector3d = chunk.read_f32_3d_vector::<SpawnByteOrder>().unwrap();
    let angle_y: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let dest_level_name: String = chunk.read_null_terminated_string().unwrap();
    let dest_graph_point: String = chunk.read_null_terminated_string().unwrap();
    let silent_mode: u8 = chunk.read_u8().unwrap();

    let enabled: u8 = chunk.read_u8().unwrap();
    let hint: String = chunk.read_null_terminated_string().unwrap();
    let save_marker: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();

    assert_eq!(
      save_marker, 26,
      "Unexpected script data provided for level changer."
    );

    AlifeLevelChanger {
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
    }
  }
}
