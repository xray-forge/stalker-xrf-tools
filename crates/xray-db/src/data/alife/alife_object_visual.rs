use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectVisual {
  pub base: AlifeObjectAbstract,
  pub visual_name: String,
  pub visual_flags: u8,
}

impl AlifeObjectInheritedReader<AlifeObjectVisual> for AlifeObjectVisual {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectVisual> {
    let base: AlifeObjectAbstract = AlifeObjectAbstract::read_from_chunk::<T>(chunk)?;

    let visual_name: String = chunk.read_null_terminated_string()?;
    let visual_flags: u8 = chunk.read_u8()?;

    Ok(AlifeObjectVisual {
      base,
      visual_name,
      visual_flags,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}
