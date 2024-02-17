use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_shape::AlifeObjectShape;

pub struct AlifeObjectClimable {
  pub base: AlifeObjectShape,
  pub game_material: String,
}

impl AlifeObjectClimable {
  pub fn from_file(chunk: &mut Chunk) -> AlifeObjectClimable {
    let base: AlifeObjectShape = AlifeObjectShape::from_chunk(chunk);

    let game_material: String = chunk.read_null_terminated_string().unwrap();

    assert_eq!(
      chunk.read_bytes_remain(),
      0,
      "Expected all data to be read from chunk."
    );

    AlifeObjectClimable {
      base,
      game_material,
    }
  }
}
