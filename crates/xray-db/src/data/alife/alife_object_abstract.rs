use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

/// Generic alife object abstraction data.
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
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectAbstract> {
    let game_vertex_id: u16 = chunk.read_u16::<T>().unwrap();
    let distance: f32 = chunk.read_f32::<T>().unwrap();
    let direct_control: u32 = chunk.read_u32::<T>().unwrap();
    let level_vertex_id: u32 = chunk.read_u32::<T>().unwrap();
    let flags: u32 = chunk.read_u32::<T>().unwrap();
    let custom_data: String = chunk.read_null_terminated_string().unwrap();
    let story_id: u32 = chunk.read_u32::<T>().unwrap();
    let spawn_story_id: u32 = chunk.read_u32::<T>().unwrap();

    Ok(AlifeObjectAbstract {
      game_vertex_id,
      distance,
      direct_control,
      level_vertex_id,
      flags,
      custom_data,
      story_id,
      spawn_story_id,
    })
  }
}
