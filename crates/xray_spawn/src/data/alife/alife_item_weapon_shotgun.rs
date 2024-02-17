use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_item_weapon::AlifeItemWeapon;
use crate::data::alife_object::AlifeObjectInherited;

pub struct AlifeItemWeaponShotgun {
  pub base: AlifeItemWeapon,
}

impl AlifeObjectInherited<AlifeItemWeaponShotgun> for AlifeItemWeaponShotgun {
  fn from_chunk(chunk: &mut Chunk) -> AlifeItemWeaponShotgun {
    let base: AlifeItemWeapon = AlifeItemWeapon::from_chunk(chunk);

    AlifeItemWeaponShotgun { base }
  }
}
