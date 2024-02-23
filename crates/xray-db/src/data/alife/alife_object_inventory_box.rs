use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectInventoryBox {
  pub base: AlifeObjectVisual,
  pub can_take: u8,
  pub is_closed: u8,
  pub tip: String,
}

impl AlifeObjectInheritedReader<AlifeObjectInventoryBox> for AlifeObjectInventoryBox {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectInventoryBox> {
    let base: AlifeObjectVisual = AlifeObjectVisual::read_from_chunk::<T>(chunk)?;

    let can_take: u8 = chunk.read_u8().unwrap();
    let is_closed: u8 = chunk.read_u8().unwrap();
    let tip: String = chunk.read_null_terminated_string().unwrap();

    Ok(AlifeObjectInventoryBox {
      base,
      can_take,
      is_closed,
      tip,
    })
  }
}

impl AlifeObjectGeneric for AlifeObjectInventoryBox {}
