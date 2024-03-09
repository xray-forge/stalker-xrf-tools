use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::export::file_import::read_ini_field;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectTraderAbstract {
  #[serde(rename = "money")]
  pub money: u32,
  #[serde(rename = "specificCharacter")]
  pub specific_character: String,
  #[serde(rename = "traderFlags")]
  pub trader_flags: u32,
  #[serde(rename = "characterProfile")]
  pub character_profile: String,
  #[serde(rename = "communityIndex")]
  pub community_index: u32,
  #[serde(rename = "rank")]
  pub rank: u32,
  #[serde(rename = "reputation")]
  pub reputation: u32,
  #[serde(rename = "characterName")]
  pub character_name: String,
  #[serde(rename = "deadBodyCanTake")]
  pub dead_body_can_take: u8,
  #[serde(rename = "deadBodyClosed")]
  pub dead_body_closed: u8,
}

impl AlifeObjectInheritedReader<AlifeObjectTraderAbstract> for AlifeObjectTraderAbstract {
  /// Read trader data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectTraderAbstract> {
    Ok(AlifeObjectTraderAbstract {
      money: reader.read_u32::<SpawnByteOrder>()?,
      specific_character: reader.read_null_terminated_win_string()?,
      trader_flags: reader.read_u32::<SpawnByteOrder>()?,
      character_profile: reader.read_null_terminated_win_string()?,
      community_index: reader.read_u32::<SpawnByteOrder>()?,
      rank: reader.read_u32::<SpawnByteOrder>()?,
      reputation: reader.read_u32::<SpawnByteOrder>()?,
      character_name: reader.read_null_terminated_win_string()?,
      dead_body_can_take: reader.read_u8()?,
      dead_body_closed: reader.read_u8()?,
    })
  }

  /// Import trader data from ini config section.
  fn import(section: &Section) -> io::Result<AlifeObjectTraderAbstract> {
    Ok(AlifeObjectTraderAbstract {
      money: read_ini_field("money", section)?,
      specific_character: read_ini_field("specific_character", section)?,
      trader_flags: read_ini_field("trader_flags", section)?,
      character_profile: read_ini_field("character_profile", section)?,
      community_index: read_ini_field("community_index", section)?,
      rank: read_ini_field("rank", section)?,
      reputation: read_ini_field("reputation", section)?,
      character_name: read_ini_field("character_name", section)?,
      dead_body_can_take: read_ini_field("dead_body_can_take", section)?,
      dead_body_closed: read_ini_field("dead_body_closed", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectTraderAbstract {
  /// Write trader data into the chunk.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_u32::<SpawnByteOrder>(self.money)?;
    writer.write_null_terminated_win_string(&self.specific_character)?;
    writer.write_u32::<SpawnByteOrder>(self.trader_flags)?;
    writer.write_null_terminated_win_string(&self.character_profile)?;
    writer.write_u32::<SpawnByteOrder>(self.community_index)?;
    writer.write_u32::<SpawnByteOrder>(self.rank)?;
    writer.write_u32::<SpawnByteOrder>(self.reputation)?;
    writer.write_null_terminated_win_string(&self.character_name)?;
    writer.write_u8(self.dead_body_can_take)?;
    writer.write_u8(self.dead_body_closed)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) {
    ini
      .with_section(section)
      .set("money", self.money.to_string())
      .set("specific_character", &self.specific_character)
      .set("trader_flags", self.trader_flags.to_string())
      .set("character_profile", &self.character_profile)
      .set("community_index", self.community_index.to_string())
      .set("rank", self.rank.to_string())
      .set("reputation", self.reputation.to_string())
      .set("character_name", &self.character_name)
      .set("dead_body_can_take", self.dead_body_can_take.to_string())
      .set("dead_body_closed", self.dead_body_closed.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_trader_abstract::AlifeObjectTraderAbstract;
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
      get_relative_test_sample_file_path(file!(), "alife_object_trader_abstract.chunk");

    let object: AlifeObjectTraderAbstract = AlifeObjectTraderAbstract {
      money: 1453,
      specific_character: String::from("specific-character"),
      trader_flags: 33,
      character_profile: String::from("character-profile"),
      community_index: 4,
      rank: 211,
      reputation: 300,
      character_name: String::from("character-name"),
      dead_body_can_take: 1,
      dead_body_closed: 0,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 74);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 74);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 74 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectTraderAbstract =
      AlifeObjectTraderAbstract::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
