use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
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
  /// Read generic alife object base data from the file.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectAbstract> {
    let game_vertex_id: u16 = chunk.read_u16::<T>()?;
    let distance: f32 = chunk.read_f32::<T>()?;
    let direct_control: u32 = chunk.read_u32::<T>()?;
    let level_vertex_id: u32 = chunk.read_u32::<T>()?;
    let flags: u32 = chunk.read_u32::<T>()?;
    let custom_data: String = chunk.read_null_terminated_string()?;
    let story_id: u32 = chunk.read_u32::<T>()?;
    let spawn_story_id: u32 = chunk.read_u32::<T>()?;

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

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    Ok(())
  }
}
