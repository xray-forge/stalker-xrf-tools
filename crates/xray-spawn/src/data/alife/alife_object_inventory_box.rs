use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use crate::data::alife_object_base::{AlifeObjectGeneric, AlifeObjectInheritedReader};
use byteorder::ReadBytesExt;

pub struct AlifeObjectInventoryBox {
  pub base: AlifeObjectVisual,
  pub can_take: u8,
  pub is_closed: u8,
  pub tip: String,
}

impl AlifeObjectInheritedReader<AlifeObjectInventoryBox> for AlifeObjectInventoryBox {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectInventoryBox {
    let base: AlifeObjectVisual = AlifeObjectVisual::from_chunk(chunk);

    let can_take: u8 = chunk.read_u8().unwrap();
    let is_closed: u8 = chunk.read_u8().unwrap();
    let tip: String = chunk.read_null_terminated_string().unwrap();

    AlifeObjectInventoryBox {
      base,
      can_take,
      is_closed,
      tip,
    }
  }
}

impl AlifeObjectGeneric for AlifeObjectInventoryBox {}
