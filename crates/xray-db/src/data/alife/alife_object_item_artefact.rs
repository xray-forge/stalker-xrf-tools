use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item::AlifeObjectItem;
use byteorder::ByteOrder;
use std::io;

pub struct AlifeItemArtefact {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInheritedReader<AlifeItemArtefact> for AlifeItemArtefact {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeItemArtefact> {
    let base: AlifeObjectItem = AlifeObjectItem::read_from_chunk::<T>(chunk)?;

    Ok(AlifeItemArtefact { base })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeItemArtefact {}
