use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectItem {
  pub base: AlifeObjectVisual,
  pub condition: f32,
}

impl AlifeObjectInheritedReader<AlifeObjectItem> for AlifeObjectItem {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectItem> {
    let base: AlifeObjectVisual = AlifeObjectVisual::read_from_chunk::<T>(chunk)?;

    let condition: f32 = chunk.read_f32::<SpawnByteOrder>()?;
    let upgrades_count: u32 = chunk.read_u32::<SpawnByteOrder>()?;

    assert_eq!(upgrades_count, 0, "Unexpected upgraded item provided.");

    Ok(AlifeObjectItem { base, condition })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeObjectItem {}
