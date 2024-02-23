use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item_weapon::AlifeObjectItemWeapon;
use byteorder::ByteOrder;
use std::io;

pub struct AlifeObjectItemWeaponMagazined {
  pub base: AlifeObjectItemWeapon,
}

impl AlifeObjectInheritedReader<AlifeObjectItemWeaponMagazined> for AlifeObjectItemWeaponMagazined {
  fn read_from_chunk<T: ByteOrder>(
    chunk: &mut Chunk,
  ) -> io::Result<AlifeObjectItemWeaponMagazined> {
    let base: AlifeObjectItemWeapon = AlifeObjectItemWeapon::read_from_chunk::<T>(chunk)?;

    Ok(AlifeObjectItemWeaponMagazined { base })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeObjectItemWeaponMagazined {}
