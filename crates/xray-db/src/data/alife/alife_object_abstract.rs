use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::Ini;
use std::io;

/// Generic alife object abstraction data.
#[derive(Clone, Debug, PartialEq)]
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
    let custom_data: String = chunk.read_null_terminated_win_string()?;
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
}

impl AlifeObjectGeneric for AlifeObjectAbstract {
  type Order = SpawnByteOrder;

  /// Write abstract object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_u16::<Self::Order>(self.game_vertex_id)?;
    writer.write_f32::<Self::Order>(self.distance)?;
    writer.write_u32::<Self::Order>(self.direct_control)?;
    writer.write_u32::<Self::Order>(self.level_vertex_id)?;
    writer.write_u32::<Self::Order>(self.flags)?;
    writer.write_null_terminated_win_string(&self.custom_data)?;
    writer.write_u32::<Self::Order>(self.story_id)?;
    writer.write_u32::<Self::Order>(self.spawn_story_id)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &String, ini: &mut Ini) {
    ini
      .with_section(Some(section))
      .set("game_vertex_id", self.game_vertex_id.to_string())
      .set("distance", self.distance.to_string())
      .set("direct_control", self.direct_control.to_string())
      .set("level_vertex_id", self.level_vertex_id.to_string())
      .set("flags", self.flags.to_string())
      .set("custom_data", &self.custom_data)
      .set("story_id", self.story_id.to_string())
      .set("spawn_story_id", self.spawn_story_id.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_abstract.chunk"));

    let object: AlifeObjectAbstract = AlifeObjectAbstract {
      game_vertex_id: 1001,
      distance: 65.25,
      direct_control: 412421,
      level_vertex_id: 66231,
      flags: 33,
      custom_data: String::from("custom_data"),
      story_id: 400,
      spawn_story_id: 25,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 38);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 38);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 38 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectAbstract =
      AlifeObjectAbstract::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
