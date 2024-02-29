use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::export::file_export::export_vector_to_string;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::Ini;
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectCreature {
  pub base: AlifeObjectDynamicVisual,
  pub team: u8,
  pub squad: u8,
  pub group: u8,
  pub health: f32,
  pub dynamic_out_restrictions: Vec<u16>,
  pub dynamic_in_restrictions: Vec<u16>,
  pub killer_id: u16,
  pub game_death_time: u64,
}

impl AlifeObjectInheritedReader<AlifeObjectCreature> for AlifeObjectCreature {
  /// Read alife creature object data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectCreature> {
    let base: AlifeObjectDynamicVisual = AlifeObjectDynamicVisual::read_from_chunk::<T>(chunk)?;

    let team: u8 = chunk.read_u8()?;
    let squad: u8 = chunk.read_u8()?;
    let group: u8 = chunk.read_u8()?;
    let health: f32 = chunk.read_f32::<SpawnByteOrder>()?;

    let dynamic_out_restrictions: Vec<u16> = chunk.read_u16_vector::<SpawnByteOrder>()?;
    let dynamic_in_restrictions: Vec<u16> = chunk.read_u16_vector::<SpawnByteOrder>()?;

    let killer_id: u16 = chunk.read_u16::<SpawnByteOrder>()?;
    let game_death_time: u64 = chunk.read_u64::<SpawnByteOrder>()?;

    Ok(AlifeObjectCreature {
      base,
      team,
      squad,
      group,
      health,
      dynamic_out_restrictions,
      dynamic_in_restrictions,
      killer_id,
      game_death_time,
    })
  }
}

impl AlifeObjectGeneric for AlifeObjectCreature {
  type Order = SpawnByteOrder;

  /// Write alife creature object data into the chunk.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_u8(self.team)?;
    writer.write_u8(self.squad)?;
    writer.write_u8(self.group)?;
    writer.write_f32::<Self::Order>(self.health)?;

    writer.write_u16_vector::<Self::Order>(&self.dynamic_out_restrictions)?;
    writer.write_u16_vector::<Self::Order>(&self.dynamic_in_restrictions)?;

    writer.write_u16::<Self::Order>(self.killer_id)?;
    writer.write_u64::<Self::Order>(self.game_death_time)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &String, ini: &mut Ini) {
    self.base.export(section, ini);

    ini
      .with_section(Some(section))
      .set("team", self.team.to_string())
      .set("squad", self.squad.to_string())
      .set("group", self.group.to_string())
      .set("health", self.health.to_string())
      .set(
        "dynamic_out_restrictions",
        &export_vector_to_string(&self.dynamic_out_restrictions),
      )
      .set(
        "dynamic_in_restrictions",
        &export_vector_to_string(&self.dynamic_in_restrictions),
      )
      .set("killer_id", self.killer_id.to_string())
      .set("game_death_time", self.killer_id.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_creature::AlifeObjectCreature;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
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
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_creature.chunk"));

    let object: AlifeObjectCreature = AlifeObjectCreature {
      base: AlifeObjectDynamicVisual {
        base: AlifeObjectAbstract {
          game_vertex_id: 1001,
          distance: 65.25,
          direct_control: 412421,
          level_vertex_id: 66231,
          flags: 33,
          custom_data: String::from("custom_data"),
          story_id: 400,
          spawn_story_id: 25,
        },
        visual_name: String::from("abcdef"),
        visual_flags: 33,
      },
      team: 2,
      squad: 3,
      group: 4,
      health: 1.0,
      dynamic_out_restrictions: vec![1, 2, 3, 4],
      dynamic_in_restrictions: vec![5, 6, 7, 8],
      killer_id: 25,
      game_death_time: 0,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 87);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 87);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 87 + 8);

    let mut chunk: Chunk = Chunk::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectCreature =
      AlifeObjectCreature::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
