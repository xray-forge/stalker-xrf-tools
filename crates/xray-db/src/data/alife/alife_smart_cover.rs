use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_smart_cover::AlifeObjectSmartCover;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

/// Represents script extension of base server smart cover class.
pub struct AlifeSmartCover {
  pub base: AlifeObjectSmartCover,
  pub last_description: String,
  pub loopholes: Vec<SmartCoverLoophole>,
}

impl AlifeObjectInheritedReader<AlifeSmartCover> for AlifeSmartCover {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeSmartCover> {
    let base: AlifeObjectSmartCover = AlifeObjectSmartCover::read_from_chunk::<T>(chunk)?;

    let last_description: String = chunk.read_null_terminated_string()?;
    let count: u8 = chunk.read_u8()?;
    let mut loopholes: Vec<SmartCoverLoophole> = Vec::new();

    for _ in 0..count {
      let name: String = chunk.read_null_terminated_string()?;
      let enabled: u8 = chunk.read_u8()?;

      loopholes.push(SmartCoverLoophole { name, enabled })
    }

    Ok(AlifeSmartCover {
      base,
      last_description,
      loopholes,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

pub struct SmartCoverLoophole {
  pub name: String,
  pub enabled: u8,
}

impl AlifeObjectGeneric for AlifeSmartCover {}
