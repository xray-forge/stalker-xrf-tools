use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item_weapon::AlifeObjectItemWeapon;
use byteorder::ByteOrder;
use std::io;

pub struct AlifeObjectItemWeaponShotgun {
  pub base: AlifeObjectItemWeapon,
}

impl AlifeObjectInheritedReader<AlifeObjectItemWeaponShotgun> for AlifeObjectItemWeaponShotgun {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectItemWeaponShotgun> {
    let base: AlifeObjectItemWeapon = AlifeObjectItemWeapon::read_from_chunk::<T>(chunk)?;

    Ok(AlifeObjectItemWeaponShotgun { base })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeObjectItemWeaponShotgun {}
