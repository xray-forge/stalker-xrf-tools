use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item_weapon::AlifeObjectItemWeapon;

pub struct AlifeObjectItemWeaponShotgun {
  pub base: AlifeObjectItemWeapon,
}

impl AlifeObjectInheritedReader<AlifeObjectItemWeaponShotgun> for AlifeObjectItemWeaponShotgun {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItemWeaponShotgun {
    let base: AlifeObjectItemWeapon = AlifeObjectItemWeapon::from_chunk(chunk);

    AlifeObjectItemWeaponShotgun { base }
  }
}

impl AlifeObjectGeneric for AlifeObjectItemWeaponShotgun {}
