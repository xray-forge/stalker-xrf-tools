use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item_weapon_magazined::AlifeObjectItemWeaponMagazined;
use byteorder::ByteOrder;
use std::io;

pub struct AlifeObjectItemWeaponMagazinedWgl {
  pub base: AlifeObjectItemWeaponMagazined,
}

impl AlifeObjectInheritedReader<AlifeObjectItemWeaponMagazinedWgl>
  for AlifeObjectItemWeaponMagazinedWgl
{
  fn read_from_chunk<T: ByteOrder>(
    chunk: &mut Chunk,
  ) -> io::Result<AlifeObjectItemWeaponMagazinedWgl> {
    let base: AlifeObjectItemWeaponMagazined =
      AlifeObjectItemWeaponMagazined::read_from_chunk::<T>(chunk)?;

    Ok(AlifeObjectItemWeaponMagazinedWgl { base })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeObjectItemWeaponMagazinedWgl {}
