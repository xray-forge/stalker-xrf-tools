use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectBreakable {
  pub base: AlifeObjectVisual,
  pub health: f32,
}

impl AlifeObjectInheritedReader<AlifeObjectBreakable> for AlifeObjectBreakable {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectBreakable> {
    let base: AlifeObjectVisual = AlifeObjectVisual::read_from_chunk::<T>(chunk)?;
    let health: f32 = chunk.read_f32::<SpawnByteOrder>()?;

    Ok(AlifeObjectBreakable { base, health })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeObjectBreakable {}
