use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_creature::AlifeObjectCreature;
use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
use crate::data::alife::alife_object_trader_abstract::AlifeObjectTraderAbstract;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, SpawnByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectActor {
  pub base: AlifeObjectCreature,
  pub trader: AlifeObjectTraderAbstract,
  pub skeleton: AlifeObjectSkeleton,
  pub holder_id: u16,
}

impl AlifeObjectReader<AlifeObjectActor> for AlifeObjectActor {
  /// Read actor data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      base: AlifeObjectCreature::read::<T>(reader)?,
      trader: AlifeObjectTraderAbstract::read::<T>(reader)?,
      skeleton: AlifeObjectSkeleton::read::<T>(reader)?,
      holder_id: reader.read_u16::<T>()?,
    })
  }

  /// Import actor data from ini config section.
  fn import(section: &Section) -> DatabaseResult<Self> {
    Ok(Self {
      base: AlifeObjectCreature::import(section)?,
      trader: AlifeObjectTraderAbstract::import(section)?,
      skeleton: AlifeObjectSkeleton::import(section)?,
      holder_id: read_ini_field("holder_id", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectActor {
  /// Write object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    self.base.write(writer)?;
    self.trader.write(writer)?;
    self.skeleton.write(writer)?;

    writer.write_u16::<SpawnByteOrder>(self.holder_id)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) {
    self.base.export(section, ini);
    self.trader.export(section, ini);
    self.skeleton.export(section, ini);

    ini
      .with_section(section)
      .set("holder_id", self.holder_id.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
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

    let original: AlifeObjectActor = AlifeObjectActor {
      base: AlifeObjectCreature {
        base: AlifeObjectDynamicVisual {
          base: AlifeObjectAbstract {
            game_vertex_id: 620,
            distance: 42.25,
            direct_control: 14,
            level_vertex_id: 52234,
            flags: 71,
            custom_data: String::from("custom-data"),
            story_id: 14,
            spawn_story_id: 336,
          },
          visual_name: String::from("visual-name"),
          visual_flags: 12,
        },
        team: 1,
        squad: 2,
        group: 3,
        health: 1.0,
        dynamic_out_restrictions: vec![1, 2, 3, 4],
        dynamic_in_restrictions: vec![5, 6, 7, 8],
        killer_id: 0,
        game_death_time: 0,
      },
      trader: AlifeObjectTraderAbstract {
        money: 5000,
        specific_character: String::from("specific-character"),
        trader_flags: 23,
        character_profile: String::from("character-profile"),
        community_index: 1,
        rank: 2,
        reputation: 3,
        character_name: String::from("character-name"),
        dead_body_can_take: 1,
        dead_body_closed: 1,
      },
      skeleton: AlifeObjectSkeleton {
        name: String::from("skeleton-name"),
        flags: 98,
        source_id: 12,
      },
      holder_id: 0,
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 185);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 185);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 185 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectActor::read::<SpawnByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
