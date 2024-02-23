use crate::chunk::chunk::Chunk;
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
}

impl AlifeObjectGeneric for AlifeObjectItemWeaponMagazinedWgl {}
