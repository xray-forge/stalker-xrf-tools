use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_item_weapon::AlifeObjectItemWeapon;
use crate::data::alife_object_base::{AlifeObjectGeneric, AlifeObjectInheritedReader};

pub struct AlifeObjectItemWeaponMagazined {
  pub base: AlifeObjectItemWeapon,
}

impl AlifeObjectInheritedReader<AlifeObjectItemWeaponMagazined> for AlifeObjectItemWeaponMagazined {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItemWeaponMagazined {
    let base: AlifeObjectItemWeapon = AlifeObjectItemWeapon::from_chunk(chunk);

    AlifeObjectItemWeaponMagazined { base }
  }
}

impl AlifeObjectGeneric for AlifeObjectItemWeaponMagazined {}
