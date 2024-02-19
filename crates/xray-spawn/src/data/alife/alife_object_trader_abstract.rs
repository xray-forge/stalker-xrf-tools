use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

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
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectTraderAbstract {
    let money: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let specific_character: String = chunk.read_null_terminated_string().unwrap();
    let trader_flags: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let character_profile: String = chunk.read_null_terminated_string().unwrap();
    let community_index: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let rank: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let reputation: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let character_name: String = chunk.read_null_terminated_string().unwrap();
    let dead_body_can_take: u8 = chunk.read_u8().unwrap();
    let dead_body_closed: u8 = chunk.read_u8().unwrap();

    AlifeObjectTraderAbstract {
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
    }
  }
}
