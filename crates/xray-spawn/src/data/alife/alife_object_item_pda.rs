use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeObjectItemPda {
  pub base: AlifeObjectItem,
  pub owner: u16,
  pub character: String,
  pub info_portion: String,
}

impl AlifeObjectInheritedReader<AlifeObjectItemPda> for AlifeObjectItemPda {
  fn read_from_chunk(chunk: &mut Chunk) -> AlifeObjectItemPda {
    let base: AlifeObjectItem = AlifeObjectItem::read_from_chunk(chunk);

    let owner: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();
    let character: String = chunk.read_null_terminated_string().unwrap();
    let info_portion: String = chunk.read_null_terminated_string().unwrap();

    AlifeObjectItemPda {
      base,
      owner,
      character,
      info_portion,
    }
  }
}

impl AlifeObjectGeneric for AlifeObjectItemPda {}
