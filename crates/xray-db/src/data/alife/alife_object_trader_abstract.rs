use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::export::file_import::read_ini_field;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use std::io;

#[derive(Clone, Debug, PartialEq)]
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

impl AlifeObjectInheritedReader<AlifeObjectTraderAbstract> for AlifeObjectTraderAbstract {
  /// Read trader data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectTraderAbstract> {
    Ok(AlifeObjectTraderAbstract {
      money: chunk.read_u32::<SpawnByteOrder>()?,
      specific_character: chunk.read_null_terminated_win_string()?,
      trader_flags: chunk.read_u32::<SpawnByteOrder>()?,
      character_profile: chunk.read_null_terminated_win_string()?,
      community_index: chunk.read_u32::<SpawnByteOrder>()?,
      rank: chunk.read_u32::<SpawnByteOrder>()?,
      reputation: chunk.read_u32::<SpawnByteOrder>()?,
      character_name: chunk.read_null_terminated_win_string()?,
      dead_body_can_take: chunk.read_u8()?,
      dead_body_closed: chunk.read_u8()?,
    })
  }

  /// Import trader data from ini config section.
  fn import(props: &Properties) -> io::Result<AlifeObjectTraderAbstract> {
    Ok(AlifeObjectTraderAbstract {
      money: read_ini_field("money", props)?,
      specific_character: read_ini_field("specific_character", props)?,
      trader_flags: read_ini_field("trader_flags", props)?,
      character_profile: read_ini_field("character_profile", props)?,
      community_index: read_ini_field("community_index", props)?,
      rank: read_ini_field("rank", props)?,
      reputation: read_ini_field("reputation", props)?,
      character_name: read_ini_field("character_name", props)?,
      dead_body_can_take: read_ini_field("dead_body_can_take", props)?,
      dead_body_closed: read_ini_field("dead_body_closed", props)?,
    })
  }
}

impl AlifeObjectGeneric for AlifeObjectTraderAbstract {
  type Order = SpawnByteOrder;

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
  fn export(&self, section: &str, ini: &mut Ini) {
    ini
      .with_section(Some(section))
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
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_trader_abstract::AlifeObjectTraderAbstract;
  use crate::test::utils::{
    get_test_sample_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_test_sample_file_sub_dir(file!(), "alife_object_trader_abstract.chunk");

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
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 74);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 74 + 8);

    let mut chunk: Chunk = Chunk::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectTraderAbstract =
      AlifeObjectTraderAbstract::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
