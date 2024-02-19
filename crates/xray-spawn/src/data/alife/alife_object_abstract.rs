use crate::chunk::chunk::Chunk;
use crate::data::alife_object_base::AlifeObjectInheritedReader;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeObjectAbstract {
  pub game_vertex_id: u16,
  pub distance: f32,
  pub direct_control: u32,
  pub level_vertex_id: u32,
  pub flags: u32,
  pub custom_data: String,
  pub story_id: u32,
  pub spawn_story_id: u32,
}

impl AlifeObjectInheritedReader<AlifeObjectAbstract> for AlifeObjectAbstract {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectAbstract {
    let game_vertex_id: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();
    let distance: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let direct_control: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let level_vertex_id: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let flags: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let custom_data: String = chunk.read_null_terminated_string().unwrap();
    let story_id: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let spawn_story_id: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();

    AlifeObjectAbstract {
      game_vertex_id,
      distance,
      direct_control,
      level_vertex_id,
      flags,
      custom_data,
      story_id,
      spawn_story_id,
    }
  }
}
