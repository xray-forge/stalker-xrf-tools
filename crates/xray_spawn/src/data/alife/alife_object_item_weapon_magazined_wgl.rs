use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_item_weapon_magazined::AlifeObjectItemWeaponMagazined;
use crate::data::alife_object::AlifeObjectInherited;

pub struct AlifeObjectItemWeaponMagazinedWgl {
  pub base: AlifeObjectItemWeaponMagazined,
}

impl AlifeObjectInherited<AlifeObjectItemWeaponMagazinedWgl> for AlifeObjectItemWeaponMagazinedWgl {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItemWeaponMagazinedWgl {
    let base: AlifeObjectItemWeaponMagazined = AlifeObjectItemWeaponMagazined::from_chunk(chunk);

    AlifeObjectItemWeaponMagazinedWgl { base }
  }
}
