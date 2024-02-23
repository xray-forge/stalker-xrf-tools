use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectItemWeapon {
  pub base: AlifeObjectItem,
  pub ammo_current: u16,
  pub ammo_elapsed: u16,
  pub weapon_state: u8,
  pub addon_flags: u8,
  pub ammo_type: u8,
  pub elapsed_grenades: u8,
}

impl AlifeObjectInheritedReader<AlifeObjectItemWeapon> for AlifeObjectItemWeapon {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectItemWeapon> {
    let base: AlifeObjectItem = AlifeObjectItem::read_from_chunk::<T>(chunk)?;

    let ammo_current: u16 = chunk.read_u16::<SpawnByteOrder>()?;
    let ammo_elapsed: u16 = chunk.read_u16::<SpawnByteOrder>()?;
    let weapon_state: u8 = chunk.read_u8()?;
    let addon_flags: u8 = chunk.read_u8()?;
    let ammo_type: u8 = chunk.read_u8()?;
    let elapsed_grenades: u8 = chunk.read_u8()?;

    Ok(AlifeObjectItemWeapon {
      base,
      ammo_current,
      ammo_elapsed,
      weapon_state,
      addon_flags,
      ammo_type,
      elapsed_grenades,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeObjectItemWeapon {}
