use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_actor::AlifeObjectActor;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, SpawnByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use typetag::serde;
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeActor {
  pub base: AlifeObjectActor,
  pub start_position_filled: u8,
  pub save_marker: u16,
}

impl AlifeObjectReader for AlifeActor {
  /// Read actor data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let object: Self = Self {
      base: AlifeObjectActor::read::<T>(reader)?,
      start_position_filled: reader.read_u8()?,
      save_marker: reader.read_u16::<SpawnByteOrder>()?,
    };

    assert_eq!(
      object.save_marker, 1,
      "Unexpected save data for actor object provided"
    );

    Ok(object)
  }

  /// Import actor data from ini file section.
  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "ALife object '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectActor::import(section_name, ini)?,
      start_position_filled: read_ini_field("start_position_filled", section)?,
      save_marker: read_ini_field("save_marker", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeActor {
  /// Write object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    self.base.write(writer)?;

    writer.write_u8(self.start_position_filled)?;
    writer.write_u16::<SpawnByteOrder>(self.save_marker)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    self.base.export(section, ini)?;

    ini
      .with_section(section)
      .set(
        "start_position_filled",
        self.start_position_filled.to_string(),
      )
      .set("save_marker", self.save_marker.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_actor::AlifeActor;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_actor::AlifeObjectActor;
  use crate::data::alife::alife_object_creature::AlifeObjectCreature;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
  use crate::data::alife::alife_object_trader_abstract::AlifeObjectTraderAbstract;
  use crate::data::meta::alife_object_generic::AlifeObjectWriter;
  use crate::data::meta::alife_object_reader::AlifeObjectReader;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: AlifeActor = AlifeActor {
      base: AlifeObjectActor {
        base: AlifeObjectCreature {
          base: AlifeObjectDynamicVisual {
            base: AlifeObjectAbstract {
              game_vertex_id: 621,
              distance: 55.25,
              direct_control: 15,
              level_vertex_id: 52235,
              flags: 72,
              custom_data: String::from("custom-data"),
              story_id: 15,
              spawn_story_id: 334,
            },
            visual_name: String::from("visual-name"),
            visual_flags: 13,
          },
          team: 2,
          squad: 3,
          group: 4,
          health: 1.0,
          dynamic_out_restrictions: vec![1, 2, 3, 4],
          dynamic_in_restrictions: vec![5, 6, 7, 8],
          killer_id: 0,
          game_death_time: 0,
        },
        trader: AlifeObjectTraderAbstract {
          money: 5000,
          specific_character: String::from("specific-character-0"),
          trader_flags: 23,
          character_profile: String::from("character-profile-0"),
          community_index: 1,
          rank: 2,
          reputation: 3,
          character_name: String::from("character-name-0"),
          dead_body_can_take: 1,
          dead_body_closed: 1,
        },
        skeleton: AlifeObjectSkeleton {
          name: String::from("skeleton-name-0"),
          flags: 98,
          source_id: 12,
        },
        holder_id: 0,
      },
      start_position_filled: 1,
      save_marker: 1,
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 196);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 196);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 196 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(AlifeActor::read::<SpawnByteOrder>(&mut reader)?, original);

    Ok(())
  }
}
