use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, SpawnByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectTraderAbstract {
  pub money: u32,
  pub specific_character: String,
  pub trader_flags: u32,
  pub character_profile: String,
  pub community_index: u32,
  pub rank: u32,
  pub reputation: u32,
  pub character_name: String,
  pub dead_body_can_take: u8,
  pub dead_body_closed: u8,
}

impl AlifeObjectReader<AlifeObjectTraderAbstract> for AlifeObjectTraderAbstract {
  /// Read trader data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
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
  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "ALife object '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
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
impl AlifeObjectWriter for AlifeObjectTraderAbstract {
  /// Write trader data into the chunk.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
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

    let original: AlifeObjectTraderAbstract = AlifeObjectTraderAbstract {
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

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 74);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 74);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 74 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectTraderAbstract::read::<SpawnByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
