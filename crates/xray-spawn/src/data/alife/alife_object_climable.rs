use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_shape::AlifeObjectShape;

pub struct AlifeObjectClimable {
  pub base: AlifeObjectShape,
  pub game_material: String,
}

impl AlifeObjectInheritedReader<AlifeObjectClimable> for AlifeObjectClimable {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectClimable {
    let base: AlifeObjectShape = AlifeObjectShape::from_chunk(chunk);

    let game_material: String = chunk.read_null_terminated_string().unwrap();

    AlifeObjectClimable {
      base,
      game_material,
    }
  }
}

impl AlifeObjectGeneric for AlifeObjectClimable {}
