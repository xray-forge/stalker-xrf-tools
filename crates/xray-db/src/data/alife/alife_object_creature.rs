use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::export::file_export::export_vector_to_string;
use crate::export::file_import::{import_vector_from_string, read_ini_field};
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Properties};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectCreature {
  #[serde(rename = "base")]
  pub base: AlifeObjectDynamicVisual,
  #[serde(rename = "team")]
  pub team: u8,
  #[serde(rename = "squad")]
  pub squad: u8,
  #[serde(rename = "group")]
  pub group: u8,
  #[serde(rename = "health")]
  pub health: f32,
  #[serde(rename = "dynamicOutRestrictions")]
  pub dynamic_out_restrictions: Vec<u16>,
  #[serde(rename = "dynamicInRestrictions")]
  pub dynamic_in_restrictions: Vec<u16>,
  #[serde(rename = "killerId")]
  pub killer_id: u16,
  #[serde(rename = "gameDeathTime")]
  pub game_death_time: u64,
}

impl AlifeObjectInheritedReader<AlifeObjectCreature> for AlifeObjectCreature {
  /// Read alife creature object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectCreature> {
    Ok(AlifeObjectCreature {
      base: AlifeObjectDynamicVisual::read::<T>(reader)?,
      team: reader.read_u8()?,
      squad: reader.read_u8()?,
      group: reader.read_u8()?,
      health: reader.read_f32::<T>()?,
      dynamic_out_restrictions: reader.read_u16_vector::<T>()?,
      dynamic_in_restrictions: reader.read_u16_vector::<T>()?,
      killer_id: reader.read_u16::<T>()?,
      game_death_time: reader.read_u64::<T>()?,
    })
  }

  /// Import alife creature object from ini config section.
  fn import(props: &Properties) -> io::Result<AlifeObjectCreature> {
    Ok(AlifeObjectCreature {
      base: AlifeObjectDynamicVisual::import(props)?,
      team: read_ini_field("team", props)?,
      squad: read_ini_field("squad", props)?,
      group: read_ini_field("group", props)?,
      health: read_ini_field("health", props)?,
      dynamic_out_restrictions: import_vector_from_string(&read_ini_field::<String>(
        "dynamic_out_restrictions",
        props,
      )?)?,
      dynamic_in_restrictions: import_vector_from_string(&read_ini_field::<String>(
        "dynamic_in_restrictions",
        props,
      )?)?,
      killer_id: read_ini_field("killer_id", props)?,
      game_death_time: read_ini_field("game_death_time", props)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectCreature {
  /// Write alife creature object data into the chunk.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_u8(self.team)?;
    writer.write_u8(self.squad)?;
    writer.write_u8(self.group)?;
    writer.write_f32::<SpawnByteOrder>(self.health)?;

    writer.write_u16_vector::<SpawnByteOrder>(&self.dynamic_out_restrictions)?;
    writer.write_u16_vector::<SpawnByteOrder>(&self.dynamic_in_restrictions)?;

    writer.write_u16::<SpawnByteOrder>(self.killer_id)?;
    writer.write_u64::<SpawnByteOrder>(self.game_death_time)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) {
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
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_creature::AlifeObjectCreature;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::test::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_relative_test_sample_file_path(file!(), "alife_object_creature.chunk");

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
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 87);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 87 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectCreature =
      AlifeObjectCreature::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
