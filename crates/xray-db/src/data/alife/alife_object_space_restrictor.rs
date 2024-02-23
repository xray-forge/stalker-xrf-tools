use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::shape::Shape;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectSpaceRestrictor {
  pub base: AlifeObjectAbstract,
  pub shape: Vec<Shape>,
  pub restrictor_type: u8,
}

impl AlifeObjectInheritedReader<AlifeObjectSpaceRestrictor> for AlifeObjectSpaceRestrictor {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectSpaceRestrictor> {
    let base: AlifeObjectAbstract = AlifeObjectAbstract::read_from_chunk::<T>(chunk)?;

    let shape: Vec<Shape> = chunk.read_shape_description::<SpawnByteOrder>()?;
    let restrictor_type: u8 = chunk.read_u8()?;

    Ok(AlifeObjectSpaceRestrictor {
      base,
      shape,
      restrictor_type,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeObjectSpaceRestrictor {}
