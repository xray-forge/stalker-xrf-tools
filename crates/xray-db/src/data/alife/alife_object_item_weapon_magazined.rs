use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item_weapon::AlifeObjectItemWeapon;

pub struct AlifeObjectItemWeaponMagazined {
  pub base: AlifeObjectItemWeapon,
}

impl AlifeObjectInheritedReader<AlifeObjectItemWeaponMagazined> for AlifeObjectItemWeaponMagazined {
  fn read_from_chunk(chunk: &mut Chunk) -> AlifeObjectItemWeaponMagazined {
    let base: AlifeObjectItemWeapon = AlifeObjectItemWeapon::read_from_chunk(chunk);

    AlifeObjectItemWeaponMagazined { base }
  }
}

impl AlifeObjectGeneric for AlifeObjectItemWeaponMagazined {}
