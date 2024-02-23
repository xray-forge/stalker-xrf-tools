use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use byteorder::ByteOrder;
use std::io;

pub struct AlifeSmartZone {
  pub base: AlifeObjectSpaceRestrictor,
}

impl AlifeObjectInheritedReader<AlifeSmartZone> for AlifeSmartZone {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeSmartZone> {
    let base: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor::read_from_chunk::<T>(chunk)?;

    Ok(AlifeSmartZone { base })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}
