use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_item_weapon::AlifeObjectItemWeapon;
use crate::data::alife_object::AlifeObjectInherited;

pub struct AlifeObjectItemWeaponShotgun {
  pub base: AlifeObjectItemWeapon,
}

impl AlifeObjectInherited<AlifeObjectItemWeaponShotgun> for AlifeObjectItemWeaponShotgun {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItemWeaponShotgun {
    let base: AlifeObjectItemWeapon = AlifeObjectItemWeapon::from_chunk(chunk);

    AlifeObjectItemWeaponShotgun { base }
  }
}
