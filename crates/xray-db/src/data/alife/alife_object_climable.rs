use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_shape::AlifeObjectShape;
use byteorder::ByteOrder;
use std::io;

pub struct AlifeObjectClimable {
  pub base: AlifeObjectShape,
  pub game_material: String,
}

impl AlifeObjectInheritedReader<AlifeObjectClimable> for AlifeObjectClimable {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectClimable> {
    let base: AlifeObjectShape = AlifeObjectShape::read_from_chunk::<T>(chunk)?;

    let game_material: String = chunk.read_null_terminated_string()?;

    Ok(AlifeObjectClimable {
      base,
      game_material,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeObjectClimable {}
