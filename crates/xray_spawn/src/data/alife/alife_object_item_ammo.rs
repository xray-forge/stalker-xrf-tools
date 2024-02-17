use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::data::alife_object::AlifeObjectInherited;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeObjectItemAmmo {
  pub base: AlifeObjectItem,
  pub ammo_left: u16,
}

impl AlifeObjectInherited<AlifeObjectItemAmmo> for AlifeObjectItemAmmo {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItemAmmo {
    let base: AlifeObjectItem = AlifeObjectItem::from_chunk(chunk);

    let ammo_left: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();

    AlifeObjectItemAmmo { base, ammo_left }
  }
}
