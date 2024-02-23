use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item::AlifeObjectItem;
use byteorder::ByteOrder;
use std::io;

pub struct AlifeObjectItemCustomOutfit {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInheritedReader<AlifeObjectItemCustomOutfit> for AlifeObjectItemCustomOutfit {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectItemCustomOutfit> {
    let base: AlifeObjectItem = AlifeObjectItem::read_from_chunk::<T>(chunk)?;

    Ok(AlifeObjectItemCustomOutfit { base })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeObjectItemCustomOutfit {}
