use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectItemAmmo {
  pub base: AlifeObjectItem,
  pub ammo_left: u16,
}

impl AlifeObjectInheritedReader<AlifeObjectItemAmmo> for AlifeObjectItemAmmo {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectItemAmmo> {
    let base: AlifeObjectItem = AlifeObjectItem::read_from_chunk::<T>(chunk)?;

    let ammo_left: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();

    Ok(AlifeObjectItemAmmo { base, ammo_left })
  }
}

impl AlifeObjectGeneric for AlifeObjectItemAmmo {}
