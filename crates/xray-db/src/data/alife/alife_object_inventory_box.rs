use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectInventoryBox {
  pub base: AlifeObjectVisual,
  pub can_take: u8,
  pub is_closed: u8,
  pub tip: String,
}

impl AlifeObjectInheritedReader<AlifeObjectInventoryBox> for AlifeObjectInventoryBox {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectInventoryBox> {
    let base: AlifeObjectVisual = AlifeObjectVisual::read_from_chunk::<T>(chunk)?;

    let can_take: u8 = chunk.read_u8()?;
    let is_closed: u8 = chunk.read_u8()?;
    let tip: String = chunk.read_null_terminated_string()?;

    Ok(AlifeObjectInventoryBox {
      base,
      can_take,
      is_closed,
      tip,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeObjectInventoryBox {}
