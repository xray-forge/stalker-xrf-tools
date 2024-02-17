use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_item_weapon::AlifeItemWeapon;
use crate::data::alife_object::AlifeObjectInherited;

pub struct AlifeItemWeaponMagazined {
  pub base: AlifeItemWeapon,
}

impl AlifeObjectInherited<AlifeItemWeaponMagazined> for AlifeItemWeaponMagazined {
  fn from_chunk(chunk: &mut Chunk) -> AlifeItemWeaponMagazined {
    let base: AlifeItemWeapon = AlifeItemWeapon::from_chunk(chunk);

    AlifeItemWeaponMagazined { base }
  }
}
