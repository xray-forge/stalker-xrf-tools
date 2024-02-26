use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
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
    let money: u32 = chunk.read_u32::<SpawnByteOrder>()?;
    let specific_character: String = chunk.read_null_terminated_string()?;
    let trader_flags: u32 = chunk.read_u32::<SpawnByteOrder>()?;
    let character_profile: String = chunk.read_null_terminated_string()?;
    let community_index: u32 = chunk.read_u32::<SpawnByteOrder>()?;
    let rank: u32 = chunk.read_u32::<SpawnByteOrder>()?;
    let reputation: u32 = chunk.read_u32::<SpawnByteOrder>()?;
    let character_name: String = chunk.read_null_terminated_string()?;
    let dead_body_can_take: u8 = chunk.read_u8()?;
    let dead_body_closed: u8 = chunk.read_u8()?;

    Ok(AlifeObjectTraderAbstract {
      money,
      specific_character,
      trader_flags,
      character_profile,
      community_index,
      rank,
      reputation,
      character_name,
      dead_body_can_take,
      dead_body_closed,
    })
  }
}

impl AlifeObjectGeneric for AlifeObjectTraderAbstract {
  type Order = SpawnByteOrder;

  /// Write trader data into the chunk.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_u32::<SpawnByteOrder>(self.money)?;
    writer.write_null_terminated_string(&self.specific_character)?;
    writer.write_u32::<SpawnByteOrder>(self.trader_flags)?;
    writer.write_null_terminated_string(&self.character_profile)?;
    writer.write_u32::<SpawnByteOrder>(self.community_index)?;
    writer.write_u32::<SpawnByteOrder>(self.rank)?;
    writer.write_u32::<SpawnByteOrder>(self.reputation)?;
    writer.write_null_terminated_string(&self.character_name)?;
    writer.write_u8(self.dead_body_can_take)?;
    writer.write_u8(self.dead_body_closed)?;

    Ok(())
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
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_trader_abstract.chunk"));

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

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectTraderAbstract =
      AlifeObjectTraderAbstract::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
