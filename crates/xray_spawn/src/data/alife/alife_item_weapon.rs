use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::data::alife_object::AlifeObjectInherited;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeItemWeapon {
  pub base: AlifeObjectItem,
  pub ammo_current: u16,
  pub ammo_elapsed: u16,
  pub weapon_state: u8,
  pub addon_flags: u8,
  pub ammo_type: u8,
  pub elapsed_grenades: u8,
}

impl AlifeObjectInherited<AlifeItemWeapon> for AlifeItemWeapon {
  fn from_chunk(chunk: &mut Chunk) -> AlifeItemWeapon {
    let base: AlifeObjectItem = AlifeObjectItem::from_chunk(chunk);

    let ammo_current: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();
    let ammo_elapsed: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();
    let weapon_state: u8 = chunk.read_u8().unwrap();
    let addon_flags: u8 = chunk.read_u8().unwrap();
    let ammo_type: u8 = chunk.read_u8().unwrap();
    let elapsed_grenades: u8 = chunk.read_u8().unwrap();

    AlifeItemWeapon {
      base,
      ammo_current,
      ammo_elapsed,
      weapon_state,
      addon_flags,
      ammo_type,
      elapsed_grenades,
    }
  }
}
