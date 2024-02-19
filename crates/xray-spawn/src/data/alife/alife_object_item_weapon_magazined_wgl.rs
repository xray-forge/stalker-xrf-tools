use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item_weapon_magazined::AlifeObjectItemWeaponMagazined;

pub struct AlifeObjectItemWeaponMagazinedWgl {
  pub base: AlifeObjectItemWeaponMagazined,
}

impl AlifeObjectInheritedReader<AlifeObjectItemWeaponMagazinedWgl>
  for AlifeObjectItemWeaponMagazinedWgl
{
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItemWeaponMagazinedWgl {
    let base: AlifeObjectItemWeaponMagazined = AlifeObjectItemWeaponMagazined::from_chunk(chunk);

    AlifeObjectItemWeaponMagazinedWgl { base }
  }
}

impl AlifeObjectGeneric for AlifeObjectItemWeaponMagazinedWgl {}
