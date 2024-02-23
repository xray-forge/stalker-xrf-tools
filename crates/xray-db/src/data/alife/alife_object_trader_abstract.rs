use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

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

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}
