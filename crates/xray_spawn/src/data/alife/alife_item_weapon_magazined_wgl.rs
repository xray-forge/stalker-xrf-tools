use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_item_weapon_magazined::AlifeItemWeaponMagazined;
use crate::data::alife_object::AlifeObjectInherited;

pub struct AlifeItemWeaponMagazinedWgl {
  pub base: AlifeItemWeaponMagazined,
}

impl AlifeObjectInherited<AlifeItemWeaponMagazinedWgl> for AlifeItemWeaponMagazinedWgl {
  fn from_chunk(chunk: &mut Chunk) -> AlifeItemWeaponMagazinedWgl {
    let base: AlifeItemWeaponMagazined = AlifeItemWeaponMagazined::from_chunk(chunk);

    AlifeItemWeaponMagazinedWgl { base }
  }
}
