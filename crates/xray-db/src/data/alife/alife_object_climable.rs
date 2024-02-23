use crate::chunk::chunk::Chunk;
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

    let game_material: String = chunk.read_null_terminated_string().unwrap();

    Ok(AlifeObjectClimable {
      base,
      game_material,
    })
  }
}

impl AlifeObjectGeneric for AlifeObjectClimable {}
