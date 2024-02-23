use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::shape::Shape;
use crate::types::SpawnByteOrder;
use byteorder::ByteOrder;
use std::io;

pub struct AlifeObjectShape {
  pub base: AlifeObjectAbstract,
  pub shape: Vec<Shape>,
}

impl AlifeObjectInheritedReader<AlifeObjectShape> for AlifeObjectShape {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectShape> {
    let base: AlifeObjectAbstract = AlifeObjectAbstract::read_from_chunk::<T>(chunk)?;

    let shape: Vec<Shape> = chunk.read_shape_description::<SpawnByteOrder>()?;

    Ok(AlifeObjectShape { base, shape })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}
